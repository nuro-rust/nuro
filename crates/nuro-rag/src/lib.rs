//! nuro-rag — RAG（Retrieval-Augmented Generation）相关组件的最小实现。
//!
//! 当前提供：
//! - `DocumentIndexer`：遍历本地目录、对文本文件做简单分块并写入向量存储；
//! - `RetrieverTool`：实现 `nuro-core::Tool` 的检索工具，基于关键词匹配从
//!   `VectorStore` 中检索相关文档；
//! - `VectorStore` / `Embedder` trait 以及对应的实现：
//!   - `NoopVectorStore` / `NoopEmbedder`：占位实现；
//!   - `InMemoryVectorStore`：基于关键词 + Jaccard 相似度的内存检索实现。
//!
//! 目标是保证 API 形状稳定，后续可以在不破坏现有调用的前提下接入真实
//! 向量库与 embedding 服务。

mod retriever;
mod vector;
mod indexer;

pub use retriever::RetrieverTool;
pub use vector::{
    Embedder, InMemoryVectorStore, NoopEmbedder, NoopVectorStore, VectorStore, VectorEntry,
    VectorSearchResult,
};
pub use indexer::DocumentIndexer;
