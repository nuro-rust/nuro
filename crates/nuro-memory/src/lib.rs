//! nuro-memory — 记忆系统最小实现。
//!
//! 当前仅提供：
//! - `ConversationMemory`：基于 `max_messages` 的简单 FIFO 截断对话记忆；
//! - `MemoryStore` trait 与一个纯内存占位实现 `InMemoryMemoryStore`。
//!
//! 这些类型主要用于对外暴露 API 形状，方便后续平滑扩展为向量记忆、
//! 长短期记忆等更复杂能力。

mod conversation;
mod store;

pub use conversation::ConversationMemory;
pub use store::{MemoryStore, InMemoryMemoryStore};
