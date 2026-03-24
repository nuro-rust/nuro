use nuro_core::message::Message;

/// 简单的对话记忆：仅按消息条数做 FIFO 截断，不涉及 token 计算。
#[derive(Debug, Clone)]
pub struct ConversationMemory {
    max_messages: usize,
    messages: Vec<Message>,
}

impl ConversationMemory {
    /// 创建一个新的对话记忆实例。
    pub fn new(max_messages: usize) -> Self {
        Self {
            max_messages,
            messages: Vec::new(),
        }
    }

    /// 当前配置的最大消息条数。
    pub fn max_messages(&self) -> usize {
        self.max_messages
    }

    /// 追加一条消息，并在必要时从头部丢弃旧消息以满足 `max_messages` 约束。
    pub fn push(&mut self, message: Message) {
        self.messages.push(message);
        self.truncate_if_needed();
    }

    fn truncate_if_needed(&mut self) {
        if self.messages.len() > self.max_messages {
            let overflow = self.messages.len() - self.max_messages;
            self.messages.drain(0..overflow);
        }
    }

    /// 以切片形式返回当前所有记忆中的消息（按时间顺序）。
    pub fn messages(&self) -> &[Message] {
        &self.messages
    }
}
