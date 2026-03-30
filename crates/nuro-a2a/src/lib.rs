//! nuro-a2a — Agent-to-Agent 协议最小骨架实现。
//!
//! 该 crate 仅提供：
//! - `AgentCard` 结构体：描述一个远程 Agent 的基本信息；
//! - `A2aServer::builder().agent(A).serve(addr)`：占位实现，只打印日志并返回 `Ok(())`；
//! - `A2aClient::discover(url)`：占位实现，返回 `Err(anyhow!("not implemented"))`；
//! - `A2aClient::subscribe_task`：占位方法，目前什么都不做。
//!
//! 目标是对外暴露 API 形状，为后续真实协议实现铺路。

mod client;
mod server;
mod types;

pub use client::A2aClient;
pub use server::{A2aServer, A2aServerBuilder};
pub use types::AgentCard;
