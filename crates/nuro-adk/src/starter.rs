use std::time::{SystemTime, UNIX_EPOCH};

use nuro_core::{AgentContext, AgentInput, Result, SessionContext, Tool};
use nuro_llm::MockLlmProvider;
#[cfg(feature = "openai")]
use nuro_llm::OpenAiLlmProvider;
use nuro_runtime::AgentLoop;
use nuro_tools::ToolBox;

pub struct AdkStarterBuilder {
    system_prompt: String,
    toolbox: ToolBox,
    session_id: String,
}

impl AdkStarterBuilder {
    pub fn new() -> Self {
        Self {
            system_prompt: "You are a helpful agent powered by Nuro ADK starter.".to_string(),
            toolbox: ToolBox::new(),
            session_id: "adk-session".to_string(),
        }
    }

    pub fn system_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.system_prompt = prompt.into();
        self
    }

    pub fn session_id(mut self, session_id: impl Into<String>) -> Self {
        self.session_id = session_id.into();
        self
    }

    pub fn tool(mut self, tool: impl Tool + 'static) -> Self {
        self.toolbox = self.toolbox.with_tool(tool);
        self
    }

    pub fn toolbox(mut self, toolbox: ToolBox) -> Self {
        self.toolbox = toolbox;
        self
    }

    pub fn build_with_mock(self) -> Result<AdkStarterApp> {
        let agent = AgentLoop::builder()
            .llm(MockLlmProvider::new())
            .system_prompt(self.system_prompt)
            .toolbox(self.toolbox)
            .build()?;
        Ok(AdkStarterApp {
            agent,
            session_id: self.session_id,
        })
    }

    #[cfg(feature = "openai")]
    pub fn build_with_openai_model(self, model: impl Into<String>) -> Result<AdkStarterApp> {
        let provider = OpenAiLlmProvider::new_with_model(model)?;
        let agent = AgentLoop::builder()
            .llm(provider)
            .system_prompt(self.system_prompt)
            .toolbox(self.toolbox)
            .build()?;
        Ok(AdkStarterApp {
            agent,
            session_id: self.session_id,
        })
    }
}

impl Default for AdkStarterBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub struct AdkStarterApp {
    agent: AgentLoop,
    session_id: String,
}

impl AdkStarterApp {
    pub fn new_context(&self) -> AgentContext {
        AgentContext::new().with_session(
            SessionContext::new(self.session_id.clone()).with_run_id(format!("run-{}", now_ms())),
        )
    }

    pub async fn invoke_text(&self, input: impl Into<String>) -> Result<String> {
        let mut ctx = self.new_context();
        let output = self
            .agent
            .run(AgentInput::text(input.into()), &mut ctx)
            .await?;
        Ok(output.text().unwrap_or_default())
    }
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}
