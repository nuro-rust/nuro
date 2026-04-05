use std::sync::Arc;

use async_trait::async_trait;
use nuro_core::{NuroError, Result, Tool, ToolContext, ToolOutput};
use serde::Deserialize;
use serde_json::{Value, json};

use crate::VectorStore;

/// 最小可用的 RAG 检索工具：
///
/// - 输入：`{"query": String}`；
/// - 若未配置向量存储（store = None），返回固定文案 "no documents indexed"；
/// - 若已配置向量存储，则基于关键词匹配从 `VectorStore` 中检索相关文档，
///   返回命中列表（含 id/score/metadata）。
pub struct RetrieverTool {
    store: Option<Arc<dyn VectorStore>>,
    top_k: usize,
}

impl RetrieverTool {
    /// 创建一个未绑定存储的检索工具，占位实现始终返回 "no documents indexed"。
    pub fn new() -> Self {
        Self {
            store: None,
            top_k: 4,
        }
    }

    /// 使用给定的 `VectorStore` 构造检索工具。
    pub fn with_store(store: Arc<dyn VectorStore>) -> Self {
        Self {
            store: Some(store),
            top_k: 4,
        }
    }

    /// 使用给定的 `VectorStore` 与返回条数上限构造检索工具。
    pub fn with_store_and_limit(store: Arc<dyn VectorStore>, top_k: usize) -> Self {
        Self {
            store: Some(store),
            top_k: top_k.max(1),
        }
    }
}

#[derive(Debug, Deserialize)]
struct RetrieverInput {
    query: String,
}

#[async_trait]
impl Tool for RetrieverTool {
    fn name(&self) -> &str {
        "retriever"
    }

    fn description(&self) -> &str {
        "Retrieve documents relevant to a natural language query using a simple keyword-based matcher over an in-memory index."
    }

    async fn execute(&self, input: Value, _ctx: &ToolContext) -> Result<ToolOutput> {
        let parsed: RetrieverInput = serde_json::from_value(input)
            .map_err(|e| NuroError::InvalidInput(format!("invalid retriever input: {e}")))?;

        // 未配置存储时，保持原有行为：返回固定提示。
        let Some(store) = &self.store else {
            let content = json!({
                "query": parsed.query,
                "result": "no documents indexed",
            });
            return Ok(ToolOutput::new(content));
        };

        let filter = json!({ "query": parsed.query });
        let hits = store.search(&[], self.top_k, Some(filter)).await?;

        if hits.is_empty() {
            let content = json!({
                "query": parsed.query,
                "result": "no documents found",
                "hits": Vec::<Value>::new(),
            });
            return Ok(ToolOutput::new(content));
        }

        let serialized_hits: Vec<Value> = hits
            .into_iter()
            .map(|h| {
                json!({
                    "id": h.id,
                    "score": h.score,
                    "metadata": h.metadata,
                })
            })
            .collect();

        let content = json!({
            "query": parsed.query,
            "top_k": self.top_k,
            "hits": serialized_hits,
        });

        Ok(ToolOutput::new(content))
    }
}
