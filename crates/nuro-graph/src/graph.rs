use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use nuro_core::{NuroError, Result};

use crate::{GraphNode, GraphStateTrait, NodeContext};

/// 状态检查点存储抽象。
///
/// 当前仅在 `InMemoryCheckpointer` 中用于开发/调试场景，未来可以替换为
/// 基于数据库或对象存储的持久化实现。
pub trait Checkpointer<S>: Send + Sync
where
    S: GraphStateTrait,
{
    /// 保存指定节点执行后的完整状态快照。
    fn save_state(&self, node_id: &str, state: &S) -> Result<()>;

    /// 加载某个节点最近一次保存的状态快照。
    ///
    /// 默认实现返回 `Ok(None)`，表示未找到对应检查点。
    fn load_state(&self, _node_id: &str) -> Result<Option<S>> {
        Ok(None)
    }
}

/// 纯内存版检查点存储，用于开发与测试。
///
/// - 按节点 id 存储最新一次执行后的状态快照；
/// - 使用互斥锁保证线程安全；
/// - 仅适用于单进程、低并发场景。
pub struct InMemoryCheckpointer<S>
where
    S: GraphStateTrait,
{
    inner: Mutex<HashMap<String, S>>,
}

impl<S> InMemoryCheckpointer<S>
where
    S: GraphStateTrait,
{
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(HashMap::new()),
        }
    }

    pub fn get(&self, node_id: &str) -> Option<S> {
        self.inner
            .lock()
            .ok()
            .and_then(|m| m.get(node_id).cloned())
    }
}

impl<S> Checkpointer<S> for InMemoryCheckpointer<S>
where
    S: GraphStateTrait,
{
    fn save_state(&self, node_id: &str, state: &S) -> Result<()> {
        let mut guard = self
            .inner
            .lock()
            .map_err(|_| NuroError::InvalidInput("failed to lock InMemoryCheckpointer".into()))?;
        guard.insert(node_id.to_string(), state.clone());
        Ok(())
    }

    fn load_state(&self, node_id: &str) -> Result<Option<S>> {
        let guard = self
            .inner
            .lock()
            .map_err(|_| NuroError::InvalidInput("failed to lock InMemoryCheckpointer".into()))?;
        Ok(guard.get(node_id).cloned())
    }
}

/// 构建中的状态图。
pub struct StateGraph<S>
where
    S: GraphStateTrait,
{
    nodes: HashMap<String, Arc<dyn GraphNode<S>>>,
    edges: HashMap<String, Vec<String>>,             // 普通有向边
    conditional_edges: HashMap<String, ConditionalEdge<S>>, // 条件边
    entry: Option<String>,
    finish: Option<String>,
}

struct ConditionalEdge<S>
where
    S: GraphStateTrait,
{
    router: Arc<dyn Fn(&S) -> String + Send + Sync>,
    routes: HashMap<String, String>,
}

impl<S> StateGraph<S>
where
    S: GraphStateTrait,
{
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashMap::new(),
            conditional_edges: HashMap::new(),
            entry: None,
            finish: None,
        }
    }

    /// 添加一个节点。
    pub fn add_node<N>(mut self, id: impl Into<String>, node: N) -> Self
    where
        N: GraphNode<S> + 'static,
    {
        let id = id.into();
        self.nodes.insert(id, Arc::new(node));
        self
    }

    /// 添加一条普通有向边 `from -> to`。
    pub fn add_edge(mut self, from: impl Into<String>, to: impl Into<String>) -> Self {
        let from = from.into();
        let to = to.into();
        self.edges.entry(from).or_default().push(to);
        self
    }

    /// 添加一条条件边：
    ///
    /// - `router` 根据当前状态返回一个路由 key；
    /// - `routes` 将 key 映射到下一跳节点 id。
    pub fn add_conditional_edge(
        mut self,
        from: impl Into<String>,
        router: impl Fn(&S) -> String + Send + Sync + 'static,
        routes: HashMap<String, String>,
    ) -> Self {
        let from = from.into();
        let edge = ConditionalEdge {
            router: Arc::new(router),
            routes,
        };
        self.conditional_edges.insert(from, edge);
        self
    }

    /// 设置入口节点 id。
    pub fn set_entry_point(mut self, id: impl Into<String>) -> Self {
        self.entry = Some(id.into());
        self
    }

    /// 设置结束节点 id。
    pub fn set_finish_point(mut self, id: impl Into<String>) -> Self {
        self.finish = Some(id.into());
        self
    }

    /// 编译为只读的 `CompiledGraph`，用于实际运行。
    ///
    /// 在编译阶段会做若干完整性检查：
    /// - 入口节点必须存在；
    /// - 如果设置了结束节点，则结束节点也必须存在；
    /// - 所有边的起点/终点都必须在节点集合中出现。
    pub fn compile(self) -> Result<CompiledGraph<S>> {
        let entry = self
            .entry
            .ok_or_else(|| NuroError::InvalidInput("entry point is not set".into()))?;

        if !self.nodes.contains_key(&entry) {
            return Err(NuroError::InvalidInput(format!(
                "entry node '{}' not found in graph",
                entry
            )));
        }

        if let Some(ref finish) = self.finish {
            if !self.nodes.contains_key(finish) {
                return Err(NuroError::InvalidInput(format!(
                    "finish node '{}' not found in graph",
                    finish
                )));
            }
        }

        // 校验普通边引用的节点是否存在。
        for (from, tos) in &self.edges {
            if !self.nodes.contains_key(from) {
                return Err(NuroError::InvalidInput(format!(
                    "edge references unknown source node '{}'",
                    from
                )));
            }
            for to in tos {
                if !self.nodes.contains_key(to) {
                    return Err(NuroError::InvalidInput(format!(
                        "edge from '{}' references unknown target node '{}'",
                        from, to
                    )));
                }
            }
        }

        // 校验条件边引用的节点是否存在。
        for (from, cond) in &self.conditional_edges {
            if !self.nodes.contains_key(from) {
                return Err(NuroError::InvalidInput(format!(
                    "conditional edge references unknown source node '{}'",
                    from
                )));
            }
            for (key, to) in &cond.routes {
                if !self.nodes.contains_key(to) {
                    return Err(NuroError::InvalidInput(format!(
                        "conditional edge from '{}' with route key '{}' references unknown target node '{}'",
                        from, key, to
                    )));
                }
            }
        }

        Ok(CompiledGraph {
            entry,
            finish: self.finish,
            nodes: self.nodes,
            edges: self.edges,
            conditional_edges: self.conditional_edges,
            checkpointer: None,
        })
    }
}

/// 已编译完成、可执行的状态图。
pub struct CompiledGraph<S>
where
    S: GraphStateTrait,
{
    entry: String,
    finish: Option<String>,
    nodes: HashMap<String, Arc<dyn GraphNode<S>>>,
    edges: HashMap<String, Vec<String>>,
    conditional_edges: HashMap<String, ConditionalEdge<S>>,
    checkpointer: Option<Arc<dyn Checkpointer<S>>>,
}

impl<S> CompiledGraph<S>
where
    S: GraphStateTrait,
{
    /// 挂载一个检查点存储实现。图的执行并不强依赖检查点存在。
    pub fn with_checkpointer<C>(mut self, checkpointer: C) -> Self
    where
        C: Checkpointer<S> + 'static,
    {
        self.checkpointer = Some(Arc::new(checkpointer));
        self
    }

    /// 按有向图顺序依次执行节点：
    /// - 从 entry 节点开始；
    /// - 每个节点运行后通过 `apply_update` 合并状态；
    /// - 若存在条件边，则优先根据 router 结果选下一跳；
    /// - 否则选择第一条普通出边；
    /// - 若到达 finish 节点或无出边，则结束执行。
    pub async fn invoke(&self, mut state: S) -> Result<S> {
        let mut ctx = NodeContext::new();
        let mut current = self.entry.clone();

        loop {
            let node = self.nodes.get(&current).ok_or_else(|| {
                NuroError::InvalidInput(format!("node '{}' not found in compiled graph", current))
            })?;

            let update = node.run(&state, &mut ctx).await?;
            state.apply_update(update);

            if let Some(cp) = &self.checkpointer {
                cp.save_state(&current, &state)?;
            }

            if let Some(ref finish) = self.finish {
                if &current == finish {
                    break;
                }
            }

            // 条件路由优先。
            if let Some(cond) = self.conditional_edges.get(&current) {
                let key = (cond.router)(&state);
                if let Some(next) = cond.routes.get(&key) {
                    current = next.clone();
                    continue;
                }
            }

            // 普通有向边（按插入顺序取第一条）。
            if let Some(nexts) = self.edges.get(&current) {
                if let Some(next) = nexts.first() {
                    current = next.clone();
                    continue;
                }
            }

            // 没有出边时终止。
            break;
        }

        Ok(state)
    }

    /// 从某个节点的检查点恢复执行的占位接口：
    ///
    /// - 若未挂载 `Checkpointer`，返回错误；
    /// - 若找不到对应节点的检查点，同样返回错误；
    /// - 目前的实现会从加载出的状态重新执行整张图（仍从 entry 开始），
    ///   未来可以扩展为从任意节点继续执行。
    pub async fn resume(&self, node_id: &str) -> Result<S> {
        let cp = self
            .checkpointer
            .as_ref()
            .ok_or_else(|| NuroError::InvalidInput("cannot resume without a checkpointer".into()))?;

        let state = cp
            .load_state(node_id)?
            .ok_or_else(|| NuroError::InvalidInput(format!(
                "no checkpoint found for node '{}'",
                node_id
            )))?;

        self.invoke(state).await
    }
}
