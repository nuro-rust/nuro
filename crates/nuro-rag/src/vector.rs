use std::collections::{HashMap, HashSet};
use std::sync::Mutex;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use nuro_core::Result;

/// 向量存储后端抽象。
#[async_trait]
pub trait VectorStore: Send + Sync {
    async fn upsert(&self, _entries: &[VectorEntry]) -> Result<()> {
        Ok(())
    }

    async fn search(
        &self,
        _query_vector: &[f32],
        _limit: usize,
        _filter: Option<Value>,
    ) -> Result<Vec<VectorSearchResult>> {
        Ok(Vec::new())
    }

    async fn delete(&self, _ids: &[String]) -> Result<()> {
        Ok(())
    }
}

/// 向量化器抽象。
#[async_trait]
pub trait Embedder: Send + Sync {
    async fn embed(&self, _texts: &[String]) -> Result<Vec<Vec<f32>>> {
        Ok(Vec::new())
    }
}

/// 纯占位实现：不做任何真实存储，仅返回空结果。
#[derive(Default)]
pub struct NoopVectorStore;

#[async_trait]
impl VectorStore for NoopVectorStore {}

/// 纯占位实现：不做任何真实 embedding，仅返回空向量列表。
#[derive(Default)]
pub struct NoopEmbedder;

#[async_trait]
impl Embedder for NoopEmbedder {}

/// 简单的内存向量存储实现：
///
/// - 使用 `HashMap<id, VectorEntry>` 存储条目；
/// - `upsert` 按 id 覆盖；
/// - `search` 使用非常粗糙的“关键词包含 + Jaccard”相似度，
///   主要依赖 `metadata["text"]` 中的原始文档片段；
/// - 忽略传入的 `query_vector` 数值，仅通过 filter 中的 `"query"` 做匹配。
#[derive(Default)]
pub struct InMemoryVectorStore {
    inner: Mutex<HashMap<String, VectorEntry>>,
}

#[async_trait]
impl VectorStore for InMemoryVectorStore {
    async fn upsert(&self, entries: &[VectorEntry]) -> Result<()> {
        let mut guard = self.inner.lock().unwrap();
        for entry in entries {
            guard.insert(entry.id.clone(), entry.clone());
        }
        Ok(())
    }

    async fn search(
        &self,
        _query_vector: &[f32],
        limit: usize,
        filter: Option<Value>,
    ) -> Result<Vec<VectorSearchResult>> {
        let query = filter
            .as_ref()
            .and_then(|v| v.get("query"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .trim()
            .to_lowercase();

        let guard = self.inner.lock().unwrap();
        let mut results = Vec::new();

        if query.is_empty() {
            // 若未提供 query，则返回前 `limit` 条，打一个固定分数。
            for entry in guard.values().take(limit) {
                results.push(VectorSearchResult {
                    id: entry.id.clone(),
                    score: 0.0,
                    metadata: entry.metadata.clone(),
                });
            }
            return Ok(results);
        }

        let query_tokens: HashSet<String> =
            query.split_whitespace().map(|s| s.to_lowercase()).collect();

        for entry in guard.values() {
            let text = entry
                .metadata
                .get("text")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_lowercase();

            if text.is_empty() {
                continue;
            }

            let doc_tokens: HashSet<String> =
                text.split_whitespace().map(|s| s.to_lowercase()).collect();

            let intersection: HashSet<&String> = query_tokens.intersection(&doc_tokens).collect();
            if intersection.is_empty() {
                continue;
            }

            let union_size = query_tokens.union(&doc_tokens).count() as f32;
            let score = (intersection.len() as f32) / union_size;

            results.push(VectorSearchResult {
                id: entry.id.clone(),
                score,
                metadata: entry.metadata.clone(),
            });
        }

        results.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        if results.len() > limit {
            results.truncate(limit);
        }

        Ok(results)
    }

    async fn delete(&self, ids: &[String]) -> Result<()> {
        let mut guard = self.inner.lock().unwrap();
        for id in ids {
            guard.remove(id);
        }
        Ok(())
    }
}

/// 一个向量条目，用于 upsert。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorEntry {
    pub id: String,
    pub vector: Vec<f32>,
    pub metadata: Value,
}

/// 检索结果。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorSearchResult {
    pub id: String,
    pub score: f32,
    pub metadata: Value,
}
