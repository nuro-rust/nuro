use std::sync::Arc;

use async_trait::async_trait;
use nuro_core::{
    message::{ContentBlock, Message},
    tool::{Tool, ToolContext},
    Agent, AgentContext, AgentInput, AgentOutput, Event, EventKind, LlmProvider, LlmRequest,
    NuroError, Result,
};
use nuro_tools::ToolBox;
use serde_json::Value;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

/// Hook 钩子接口：允许在 AgentLoop 的 THINK / ACT / OBSERVE 生命周期中插入自定义逻辑。
///
/// 所有方法都有默认空实现，使用者只需实现自己关心的部分即可。
#[async_trait]
pub trait Hook: Send + Sync {
    /// 在调用 LLM 之前触发（THINK 之前）。
    async fn before_llm(&self, _messages: &[Message], _ctx: &mut AgentContext) -> Result<()> {
        Ok(())
    }

    /// 在收到 LLM 响应之后触发（THINK 之后）。
    async fn after_llm(&self, _response: &Message, _ctx: &mut AgentContext) -> Result<()> {
        Ok(())
    }

    /// 在调用工具之前触发（ACT 之前）。
    async fn before_tool(
        &self,
        _tool_name: &str,
        _input: &Value,
        _ctx: &mut AgentContext,
    ) -> Result<()> {
        Ok(())
    }

    /// 在工具执行结束后触发（OBSERVE 之后）。
    async fn after_tool(
        &self,
        _tool_name: &str,
        _output: &Value,
        _ctx: &mut AgentContext,
    ) -> Result<()> {
        Ok(())
    }
}

/// Guardrail 判定结果：当前版本仅区分允许 / 拦截。
#[derive(Debug, Clone)]
pub enum GuardrailDecision {
    Allow,
    Block { reason: String },
}

/// Guardrail 安全护栏接口：
///
/// - `check_input` 在 AgentLoop 开始执行前检查输入；
/// - `check_output` 在返回最终输出前检查结果；
///
/// 任一返回 `Block` 时，AgentLoop 会中止并返回错误。
#[async_trait]
pub trait Guardrail: Send + Sync {
    async fn check_input(
        &self,
        _input: &AgentInput,
        _ctx: &AgentContext,
    ) -> Result<GuardrailDecision> {
        Ok(GuardrailDecision::Allow)
    }

    async fn check_output(
        &self,
        _output: &AgentOutput,
        _ctx: &AgentContext,
    ) -> Result<GuardrailDecision> {
        Ok(GuardrailDecision::Allow)
    }
}

pub struct AgentLoop {
    llm: Arc<dyn LlmProvider>,
    tools: ToolBox,
    system_prompt: Option<String>,
    hooks: Vec<Arc<dyn Hook>>,
    guardrails: Vec<Arc<dyn Guardrail>>,
}

impl AgentLoop {
    pub fn builder() -> AgentLoopBuilder {
        AgentLoopBuilder::new()
    }

    fn build_messages(&self, input: AgentInput) -> Vec<Message> {
        let mut messages = Vec::new();
        if let Some(prompt) = &self.system_prompt {
            messages.push(Message::system(prompt));
        }
        match input {
            AgentInput::Text(text) => messages.push(Message::user(text)),
            AgentInput::Messages(mut ms) => messages.append(&mut ms),
        }
        messages
    }

    async fn run_inner(
        &self,
        input: AgentInput,
        ctx: &mut AgentContext,
        tx: Option<mpsc::Sender<Result<Event>>>,
    ) -> Result<AgentOutput> {
        // Guardrail：检查输入。
        for guardrail in &self.guardrails {
            match guardrail.check_input(&input, ctx).await? {
                GuardrailDecision::Allow => {}
                GuardrailDecision::Block { reason } => {
                    return Err(NuroError::InvalidInput(format!(
                        "input blocked by guardrail: {reason}"
                    )));
                }
            }
        }

        let mut messages = self.build_messages(input);

        // 发送 LLM 请求事件。
        if let Some(tx) = tx.as_ref() {
            let event = Event {
                kind: EventKind::LlmRequest {
                    messages: messages.clone(),
                },
            };
            let _ = tx.send(Ok(event)).await;
        }

        // THINK: 调用 LLM（前后执行 Hook）。
        for hook in &self.hooks {
            hook.before_llm(&messages, ctx).await?;
        }

        let request = LlmRequest {
            messages: messages.clone(),
        };
        let response = self.llm.generate(request).await?;

        for hook in &self.hooks {
            hook.after_llm(&response.message, ctx).await?;
        }

        messages.push(response.message.clone());

        // 将 LLM 响应通过事件输出（可拆成若干块，以模拟简单流式）。
        if let Some(tx) = tx.as_ref() {
            emit_llm_response_events(tx, &response.message).await;
        }

        // 解析 ToolUse。
        let tool_calls = response.message.tool_uses();

        // 如果没有 Tool 调用，直接返回。
        if tool_calls.is_empty() {
            let output = AgentOutput::new(messages);
            return self.apply_output_guardrails(output, ctx).await;
        }

        // ACT & OBSERVE：依次执行工具并把结果追加到消息列表。
        for call in tool_calls {
            if let Some(tx) = tx.as_ref() {
                let start_event = Event {
                    kind: EventKind::ToolCallStart {
                        tool_name: call.name.clone(),
                        input: call.input.clone(),
                    },
                };
                let _ = tx.send(Ok(start_event)).await;
            }

            let tool = match self.tools.get(&call.name) {
                Some(t) => t,
                None => {
                    let content = serde_json::json!({
                        "error": format!("tool '{}' not found", call.name),
                    });

                    for hook in &self.hooks {
                        hook.after_tool(&call.name, &content, ctx).await?;
                    }

                    if let Some(tx) = tx.as_ref() {
                        let end_event = Event {
                            kind: EventKind::ToolCallEnd {
                                tool_name: call.name.clone(),
                                output: content.clone(),
                            },
                        };
                        let _ = tx.send(Ok(end_event)).await;
                    }

                    messages.push(Message::tool_result(call.id, content, true));
                    continue;
                }
            };

            let tool_ctx = ToolContext::new();

            for hook in &self.hooks {
                hook.before_tool(&call.name, &call.input, ctx).await?;
            }

            match tool.execute(call.input.clone(), &tool_ctx).await {
                Ok(output) => {
                    for hook in &self.hooks {
                        hook.after_tool(&call.name, &output.content, ctx).await?;
                    }

                    if let Some(tx) = tx.as_ref() {
                        let end_event = Event {
                            kind: EventKind::ToolCallEnd {
                                tool_name: call.name.clone(),
                                output: output.content.clone(),
                            },
                        };
                        let _ = tx.send(Ok(end_event)).await;
                    }

                    messages.push(Message::tool_result(call.id.clone(), output.content, false));
                }
                Err(err) => {
                    let content = serde_json::json!({ "error": err.to_string() });

                    for hook in &self.hooks {
                        hook.after_tool(&call.name, &content, ctx).await?;
                    }

                    if let Some(tx) = tx.as_ref() {
                        let end_event = Event {
                            kind: EventKind::ToolCallEnd {
                                tool_name: call.name.clone(),
                                output: content.clone(),
                            },
                        };
                        let _ = tx.send(Ok(end_event)).await;
                    }

                    messages.push(Message::tool_result(call.id.clone(), content, true));
                }
            }
        }

        let output = AgentOutput::new(messages);
        self.apply_output_guardrails(output, ctx).await
    }

    async fn apply_output_guardrails(
        &self,
        output: AgentOutput,
        ctx: &AgentContext,
    ) -> Result<AgentOutput> {
        for guardrail in &self.guardrails {
            match guardrail.check_output(&output, ctx).await? {
                GuardrailDecision::Allow => {}
                GuardrailDecision::Block { reason } => {
                    return Err(NuroError::Llm(format!(
                        "output blocked by guardrail: {reason}"
                    )));
                }
            }
        }
        Ok(output)
    }

    /// 同步执行 AgentLoop，返回完整的 `AgentOutput`。
    pub async fn run(&self, input: AgentInput, ctx: &mut AgentContext) -> Result<AgentOutput> {
        self.run_inner(input, ctx, None).await
    }

    /// 极简版流式接口：
    /// - 在后台执行完整的 AgentLoop；
    /// - 通过 `Event` 事件流报告 LLM 请求/响应与工具调用；
    /// - 发生错误时，最后一个元素为 `Err(NuroError)`。
    pub fn stream(
        &self,
        input: AgentInput,
        mut ctx: AgentContext,
    ) -> ReceiverStream<Result<Event>> {
        let (tx, rx) = mpsc::channel(16);
        let this = self.clone();

        tokio::spawn(async move {
            let tx_events = tx.clone();
            let result = this.run_inner(input, &mut ctx, Some(tx_events)).await;
            if let Err(err) = result {
                let _ = tx.send(Err(err)).await;
            }
        });

        ReceiverStream::new(rx)
    }
}

/// 将最终 LLM 消息拆成若干块，通过事件流输出。
///
/// 由于底层 Provider 当前只支持非流式接口，这里只是对完整文本做一次简单分块，
/// 主要用于示例与调试用途。
async fn emit_llm_response_events(tx: &mpsc::Sender<Result<Event>>, message: &Message) {
    let text = message.text_content().unwrap_or_default();

    if text.is_empty() {
        let event = Event {
            kind: EventKind::LlmResponse {
                message: message.clone(),
            },
        };
        let _ = tx.send(Ok(event)).await;
        return;
    }

    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    let chunk_count = 3usize.min(len).max(1);
    let chunk_size = ((len as f32) / (chunk_count as f32)).ceil() as usize;

    let mut start = 0usize;
    while start < len {
        let end = (start + chunk_size).min(len);
        let chunk: String = chars[start..end].iter().collect();

        let chunk_msg = Message::new(message.role.clone(), vec![ContentBlock::Text(chunk)]);
        let event = Event {
            kind: EventKind::LlmResponse { message: chunk_msg },
        };

        if tx.send(Ok(event)).await.is_err() {
            break;
        }

        start = end;
    }
}

impl Clone for AgentLoop {
    fn clone(&self) -> Self {
        Self {
            llm: self.llm.clone(),
            tools: self.tools.clone(),
            system_prompt: self.system_prompt.clone(),
            hooks: self.hooks.clone(),
            guardrails: self.guardrails.clone(),
        }
    }
}

pub struct AgentLoopBuilder {
    llm: Option<Arc<dyn LlmProvider>>,
    tools: ToolBox,
    system_prompt: Option<String>,
    hooks: Vec<Arc<dyn Hook>>,
    guardrails: Vec<Arc<dyn Guardrail>>,
}

impl AgentLoopBuilder {
    pub fn new() -> Self {
        Self {
            llm: None,
            tools: ToolBox::new(),
            system_prompt: None,
            hooks: Vec::new(),
            guardrails: Vec::new(),
        }
    }

    pub fn llm(mut self, llm: impl LlmProvider + 'static) -> Self {
        self.llm = Some(Arc::new(llm));
        self
    }

    pub fn system_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.system_prompt = Some(prompt.into());
        self
    }

    pub fn tool(mut self, tool: impl Tool + 'static) -> Self {
        self.tools = self.tools.with_tool(tool);
        self
    }

    pub fn toolbox(mut self, toolbox: ToolBox) -> Self {
        self.tools = toolbox;
        self
    }

    /// 注册一个 Hook。
    pub fn hook<H>(mut self, hook: H) -> Self
    where
        H: Hook + 'static,
    {
        self.hooks.push(Arc::new(hook));
        self
    }

    /// 注册一个 Guardrail。
    pub fn guardrail<G>(mut self, guardrail: G) -> Self
    where
        G: Guardrail + 'static,
    {
        self.guardrails.push(Arc::new(guardrail));
        self
    }

    pub fn build(self) -> Result<AgentLoop> {
        let llm = self
            .llm
            .ok_or_else(|| NuroError::InvalidInput("LLM provider is required".to_string()))?;
        Ok(AgentLoop {
            llm,
            tools: self.tools,
            system_prompt: self.system_prompt,
            hooks: self.hooks,
            guardrails: self.guardrails,
        })
    }
}

#[async_trait]
impl Agent for AgentLoop {
    async fn invoke(&self, input: AgentInput, ctx: &mut AgentContext) -> Result<AgentOutput> {
        self.run(input, ctx).await
    }
}
