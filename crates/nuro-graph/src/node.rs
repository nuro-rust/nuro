use std::any::Any;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Arc;

use async_trait::async_trait;
use nuro_core::{
    Agent, AgentContext, AgentInput, AgentOutput, Result,
    tool::{Tool, ToolContext, ToolOutput},
};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;

use crate::{CompiledGraph, GraphStateTrait};

#[derive(Default)]
pub struct NodeContext {
    data: HashMap<String, Box<dyn Any + Send + Sync>>,
}

impl NodeContext {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn insert<T>(&mut self, key: impl Into<String>, value: T)
    where
        T: Send + Sync + 'static,
    {
        self.data.insert(key.into(), Box::new(value));
    }

    pub fn get<T>(&self, key: &str) -> Option<&T>
    where
        T: 'static,
    {
        self.data.get(key).and_then(|b| b.downcast_ref::<T>())
    }
}

#[async_trait]
pub trait GraphNode<S>: Send + Sync
where
    S: GraphStateTrait,
{
    async fn run(&self, state: &S, ctx: &mut NodeContext) -> Result<S::Update>;
}

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
        Self {
            f,
            _marker: PhantomData,
        }
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

pub struct AgentNode<A, S>
where
    A: Agent + 'static,
    S: GraphStateTrait,
    S::Update: Default,
{
    agent: Arc<A>,
    input_mapper: Arc<dyn Fn(&S, &NodeContext) -> AgentInput + Send + Sync>,
    update_mapper: Arc<dyn Fn(&S, &AgentOutput, &NodeContext) -> S::Update + Send + Sync>,
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
            input_mapper: Arc::new(|_state, _ctx| {
                AgentInput::Text("(graph node input)".to_string())
            }),
            update_mapper: Arc::new(|_state, _output, _ctx| S::Update::default()),
        }
    }

    pub fn with_input_mapper<F>(mut self, mapper: F) -> Self
    where
        F: Fn(&S, &NodeContext) -> AgentInput + Send + Sync + 'static,
    {
        self.input_mapper = Arc::new(mapper);
        self
    }

    pub fn with_update_mapper<F>(mut self, mapper: F) -> Self
    where
        F: Fn(&S, &AgentOutput, &NodeContext) -> S::Update + Send + Sync + 'static,
    {
        self.update_mapper = Arc::new(mapper);
        self
    }
}

#[async_trait]
impl<A, S> GraphNode<S> for AgentNode<A, S>
where
    A: Agent + 'static,
    S: GraphStateTrait,
    S::Update: Default,
{
    async fn run(&self, state: &S, ctx: &mut NodeContext) -> Result<S::Update> {
        let input = (self.input_mapper)(state, ctx);
        let mut agent_ctx = AgentContext::new();
        let output = self.agent.invoke(input, &mut agent_ctx).await?;
        Ok((self.update_mapper)(state, &output, ctx))
    }
}

pub struct SubGraphNode<S>
where
    S: GraphStateTrait,
{
    graph: Arc<CompiledGraph<S>>,
    update_mapper: Arc<dyn Fn(&S, &S) -> S::Update + Send + Sync>,
}

impl<S> SubGraphNode<S>
where
    S: GraphStateTrait,
{
    pub fn new(graph: CompiledGraph<S>) -> Self
    where
        S::Update: Default,
    {
        Self {
            graph: Arc::new(graph),
            update_mapper: Arc::new(|_from, _to| S::Update::default()),
        }
    }

    pub fn with_update_mapper<F>(mut self, mapper: F) -> Self
    where
        F: Fn(&S, &S) -> S::Update + Send + Sync + 'static,
    {
        self.update_mapper = Arc::new(mapper);
        self
    }
}

#[async_trait]
impl<S> GraphNode<S> for SubGraphNode<S>
where
    S: GraphStateTrait,
{
    async fn run(&self, state: &S, _ctx: &mut NodeContext) -> Result<S::Update> {
        let next_state = self.graph.invoke(state.clone()).await?;
        Ok((self.update_mapper)(state, &next_state))
    }
}

pub struct GraphTool<S>
where
    S: GraphStateTrait,
{
    name: String,
    description: String,
    graph: Arc<CompiledGraph<S>>,
}

impl<S> GraphTool<S>
where
    S: GraphStateTrait,
{
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        graph: CompiledGraph<S>,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            graph: Arc::new(graph),
        }
    }
}

#[async_trait]
impl<S> Tool for GraphTool<S>
where
    S: GraphStateTrait + Serialize + DeserializeOwned,
{
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    async fn execute(&self, input: Value, _ctx: &ToolContext) -> Result<ToolOutput> {
        let state: S = serde_json::from_value(input)
            .map_err(|e| nuro_core::NuroError::InvalidInput(e.to_string()))?;
        let output = self.graph.invoke(state).await?;
        let value = serde_json::to_value(output)
            .map_err(|e| nuro_core::NuroError::InvalidInput(e.to_string()))?;
        Ok(ToolOutput::new(value))
    }
}
