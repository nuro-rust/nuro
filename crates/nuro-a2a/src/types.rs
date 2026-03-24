use serde::{Deserialize, Serialize};

/// Agent Card — 描述一个可通过 A2A 协议访问的 Agent。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCard {
    pub name: String,
    pub description: String,
    pub url: String,
    pub version: String,
}

/// 创建任务的请求体。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskCreateRequest {
    pub input: String,
}

/// 创建任务的响应体：包含任务 id 以及一次性执行结果。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskCreateResponse {
    pub id: String,
    pub output: String,
}
