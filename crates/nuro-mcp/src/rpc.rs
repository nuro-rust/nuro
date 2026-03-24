use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 简化版 JSON-RPC 请求结构，仅支持 `list_tools` / `call_tool` 两种方法。
#[derive(Debug, Serialize, Deserialize)]
pub struct RpcRequest {
    pub id: String,
    pub method: String,
    #[serde(default)]
    pub params: Value,
}

/// 简化版 JSON-RPC 响应结构。
#[derive(Debug, Serialize, Deserialize)]
pub struct RpcResponse {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<RpcError>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RpcError {
    pub message: String,
}
