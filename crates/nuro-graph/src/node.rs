use std::any::Any;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Arc;

use async_trait::async_trait;
use nuro_core::{Agent, AgentContext, AgentInput, Result};

use crate::GraphStateTrait;

/// 节点执行上下文：提供一个简单的、基于字符串 key 的类型安全存取接口，
/// 方便在节点之间共享少量辅助数据（如 LLM Provider、计数器等）。
#[derive(Default)]
pub struct NodeContext {
    data: HashMap<String, Box<dyn Any + Send + Sync>>,    
}

impl NodeContext {
    pub fn new() -> Self {
        Self { data: HashMap::new() }
    }

    /// 按 key 存入一个任意类型的值。
    pub fn insert<T>(&mut self, key: impl Into<String>, value: T)
    where
        T: Send + Sync + 'static,
    {
        self.data.insert(key.into(), Box::new(value));
    }

    /// 按 key 以引用形式取出指定类型的值。
    pub fn get<T>(&self, key: &str) -> Option<&T>
    where
        T: 'static,
    {
        self.data.get(key).and_then(|b| b.downcast_ref::<T>())
    }
}

/// 图节点抽象：给定当前状态与上下文，返回一个状态增量。
#[async_trait]
pub trait GraphNode<S>: Send + Sync
where
    S: GraphStateTrait,
{
    async fn run(&self, state: &S, ctx: &mut NodeContext) -> Result<S::Update>;
}

/// 使用闭包实现的节点适配器。
///
/// 闭包为同步函数：方便在 demo 与简单业务中快速定义节点逻辑。
pub struct FnNode<S, F>
where
    S: GraphStateTrait,
    F: Fn(&S, &mut NodeContext) -> S::Update + Send + Sync + 'static,
{
    f: F,
    _marker: PhantomData<S>,
}

impl<S, F> FnNode<S, F>
where
    S: GraphStateTrait,
    F: Fn(&S, &mut NodeContext) -> S::Update + Send + Sync + 'static,
{
    pub fn new(f: F) -> Self {
        Self { f, _marker: PhantomData }
    }
}

#[async_trait]
impl<S, F> GraphNode<S> for FnNode<S, F>
where
    S: GraphStateTrait,
    F: Fn(&S, &mut NodeContext) -> S::Update + Send + Sync + 'static,
{
    async fn run(&self, state: &S, ctx: &mut NodeContext) -> Result<S::Update> {
        Ok((self.f)(state, ctx))
    }
}

/// 使用 `Agent` 适配为图节点的占位实现。
///
/// 当前版本中，`AgentNode` 只负责调用底层 Agent，并丢弃结果，返回
/// `Default::default()` 作为状态增量，以保证编译通过。
/// 未来版本会扩展为可配置的输入/输出映射逻辑。
pub struct AgentNode<A, S>
where
    A: Agent + 'static,
    S: GraphStateTrait,
    S::Update: Default,
{
    agent: Arc<A>,
    _marker: PhantomData<S>,
}

impl<A, S> AgentNode<A, S>
where
    A: Agent + 'static,
    S: GraphStateTrait,
    S::Update: Default,
{
    pub fn new(agent: A) -> Self {
        Self {
            agent: Arc::new(agent),
            _marker: PhantomData,
        }
    }
}

#[async_trait]
impl<A, S> GraphNode<S> for AgentNode<A, S>
where
    A: Agent + 'static,
    S: GraphStateTrait,
    S::Update: Default,
{
    async fn run(&self, _state: &S, _ctx: &mut NodeContext) -> Result<S::Update> {
        // 占位实现：目前不从 Agent 结果构造状态增量，只是走一遍调用，
        // 以验证协议与类型形状。未来可以在这里接入真正的映射逻辑。
        let mut ctx = AgentContext::new();
        let _ = self
            .agent
            .invoke(AgentInput::Text("(graph node input)".to_string()), &mut ctx)
            .await;

        Ok(S::Update::default())
    }
}
