use std::collections::HashMap;
use std::sync::Mutex;

use async_trait::async_trait;
use nuro_core::{Result, message::Message};

/// 抽象的记忆存储接口。
///
/// 为了保持 MVP 简洁，所有方法的语义都尽量宽松，仅用于占位和后续扩展。
#[async_trait]
pub trait MemoryStore: Send + Sync {
    /// 追加一条消息到某个会话中。
    async fn add(&self, conversation_id: &str, message: Message) -> Result<()>;

    /// 按简单字符串 query 查询相关消息。
    ///
    /// 约定：实现可以根据 `conversation_id` 做范围限定，也可以忽略它并全局搜索。
    async fn query(&self, _conversation_id: &str, _query: &str) -> Result<Vec<Message>> {
        Ok(Vec::new())
    }

    /// 获取某个会话的完整消息列表。
    async fn get_conversation(&self, conversation_id: &str) -> Result<Vec<Message>>;

    /// 覆盖保存整个会话的消息列表。
    async fn save_conversation(&self, conversation_id: &str, messages: &[Message]) -> Result<()>;
}

/// 纯内存版的实现：
///
/// - 使用 `HashMap<conversation_id, Vec<Message>>` 保存消息；
/// - 不做容量控制与持久化，仅用于开发与测试；
/// - 线程安全，但不保证高并发场景下的性能。
#[derive(Default)]
pub struct InMemoryMemoryStore {
    inner: Mutex<HashMap<String, Vec<Message>>>,
}

impl InMemoryMemoryStore {
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl MemoryStore for InMemoryMemoryStore {
    async fn add(&self, conversation_id: &str, message: Message) -> Result<()> {
        let mut guard = self.inner.lock().unwrap();
        guard
            .entry(conversation_id.to_string())
            .or_default()
            .push(message);
        Ok(())
    }

    /// 一个极简的包含匹配查询实现：
    /// - 仅在指定 `conversation_id` 的会话内搜索；
    /// - 使用不区分大小写的子串匹配；
    /// - 若 query 为空字符串，则返回整个会话的消息列表。
    async fn query(&self, conversation_id: &str, query: &str) -> Result<Vec<Message>> {
        let guard = self.inner.lock().unwrap();
        let messages = guard.get(conversation_id).cloned().unwrap_or_default();

        if query.trim().is_empty() {
            return Ok(messages);
        }

        let q = query.to_lowercase();
        let filtered = messages
            .into_iter()
            .filter(|m| {
                m.text_content()
                    .map(|t| t.to_lowercase().contains(&q))
                    .unwrap_or(false)
            })
            .collect();

        Ok(filtered)
    }

    async fn get_conversation(&self, conversation_id: &str) -> Result<Vec<Message>> {
        let guard = self.inner.lock().unwrap();
        Ok(guard.get(conversation_id).cloned().unwrap_or_default())
    }

    async fn save_conversation(&self, conversation_id: &str, messages: &[Message]) -> Result<()> {
        let mut guard = self.inner.lock().unwrap();
        guard.insert(conversation_id.to_string(), messages.to_vec());
        Ok(())
    }
}
