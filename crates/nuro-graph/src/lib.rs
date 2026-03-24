//! StateGraph 最小实现。
//!
//! 该模块提供一个基于状态的有向图执行引擎，用于编排多个节点的执行顺序。
//! 当前实现聚焦于：
//! - `GraphStateTrait`：带增量 `Update` 的状态抽象；
//! - `GraphNode` + `FnNode`/`AgentNode` 适配器；
//! - `StateGraph` builder 与 `CompiledGraph::invoke` 顺序执行；
//! - 可选的 `Checkpointer` / `InMemoryCheckpointer`；
//! - `NodeContext`：简单的键值上下文，内部使用 `Box<dyn Any>` 做类型擦除。
//!
//! 这是一个可用但非常精简的版本，未来可以按设计文档扩展条件边、循环、
//! 中断/恢复等高级能力。

mod state;
mod node;
mod graph;

pub use crate::state::GraphStateTrait;
pub use crate::node::{GraphNode, FnNode, AgentNode, NodeContext};
pub use crate::graph::{StateGraph, CompiledGraph, Checkpointer, InMemoryCheckpointer};
