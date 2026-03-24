use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use serde_json::json;

use nuro_core::Result;

use crate::{VectorEntry, VectorStore};

/// DocumentIndexer 负责遍历本地目录、对文本文件做简单切分，并将结果写入
/// 向量存储（当前实现基于 InMemoryVectorStore，使用关键词匹配检索）。
///
/// 设计目标：
/// - 保持实现尽量简单，无额外重依赖；
/// - 仅假定文档为 UTF-8 文本文件；
/// - 切分策略为固定字符数窗口，不做复杂的段落/句子分析。
pub struct DocumentIndexer {
    store: Arc<dyn VectorStore>,
    chunk_size: usize,
}

impl DocumentIndexer {
    /// 使用给定的 VectorStore 创建索引器。
    ///
    /// `chunk_size` 控制每个文档块的最大字符数，默认建议 512~1024。
    pub fn new(store: Arc<dyn VectorStore>, chunk_size: usize) -> Self {
        Self { store, chunk_size }
    }

    /// 返回内部使用的 VectorStore，便于与 `RetrieverTool` 共享同一个存储实例。
    pub fn store(&self) -> Arc<dyn VectorStore> {
        self.store.clone()
    }

    /// 遍历指定目录下的所有文件并建立索引。
    ///
    /// - 递归遍历目录；
    /// - 对每个文件调用 `fs::read_to_string` 尝试按 UTF-8 读取；
    /// - 将文本按固定字符数切分为多个 chunk，写入 VectorStore；
    /// - 非 UTF-8 或读取失败的文件会被静默忽略。
    pub async fn index_directory(&self, root: impl AsRef<Path>) -> Result<()> {
        let mut files = Vec::new();
        collect_files(root.as_ref(), &mut files);

        let mut entries = Vec::new();

        for path in files {
            let Ok(content) = fs::read_to_string(&path) else {
                continue;
            };

            let chunks = chunk_text(&content, self.chunk_size);
            if chunks.is_empty() {
                continue;
            }

            let source = path.to_string_lossy().to_string();

            for (idx, chunk) in chunks.into_iter().enumerate() {
                let id = format!("{}#{}", source, idx);
                let metadata = json!({
                    "source": source,
                    "chunk_index": idx,
                    "text": chunk,
                });

                entries.push(VectorEntry {
                    id,
                    vector: Vec::new(),
                    metadata,
                });
            }
        }

        if !entries.is_empty() {
            self.store.upsert(&entries).await?;
        }

        Ok(())
    }
}

fn collect_files(root: &Path, out: &mut Vec<PathBuf>) {
    if root.is_file() {
        out.push(root.to_path_buf());
        return;
    }

    if let Ok(read_dir) = fs::read_dir(root) {
        for entry in read_dir.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_files(&path, out);
            } else if path.is_file() {
                out.push(path);
            }
        }
    }
}

fn chunk_text(text: &str, chunk_size: usize) -> Vec<String> {
    if text.is_empty() || chunk_size == 0 {
        return Vec::new();
    }

    let chars: Vec<char> = text.chars().collect();
    let mut chunks = Vec::new();
    let mut start = 0usize;

    while start < chars.len() {
        let end = (start + chunk_size).min(chars.len());
        let chunk: String = chars[start..end].iter().collect();
        chunks.push(chunk);
        start = end;
    }

    chunks
}
