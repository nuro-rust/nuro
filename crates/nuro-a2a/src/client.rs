use anyhow::{anyhow, Result};
use reqwest::Client;

use crate::types::{AgentCard, TaskCreateRequest, TaskCreateResponse};

/// A2A Client：通过 HTTP 与远程 Agent 交互的最简实现。
///
/// - `discover(url)`：读取 `/.well-known/agent.json` 并返回 `AgentCard`；
/// - `send_task(text)`：向 `/tasks` 发送一次性任务，请求体仅包含文本；
/// - `subscribe_task(task_id)`：请求 `/tasks/:id/stream`，解析 SSE 数据并以
///   `Vec<String>` 的形式返回各个数据块。
pub struct A2aClient {
    base_url: String,
    http: Client,
}

impl A2aClient {
    /// 使用给定的 base url 创建客户端，例如 `http://127.0.0.1:4000`。
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            http: Client::new(),
        }
    }

    /// 从 AgentCard 创建客户端，默认使用 card.url 作为 base url。
    pub fn from_card(card: &AgentCard) -> Self {
        Self::new(card.url.clone())
    }

    /// 从远程 URL 发现 Agent：请求 `GET {url}/.well-known/agent.json`。
    pub async fn discover(url: &str) -> Result<AgentCard> {
        let trimmed = url.trim_end_matches('/');
        let full = format!("{}/.well-known/agent.json", trimmed);
        let resp = Client::new().get(full).send().await?;
        if !resp.status().is_success() {
            return Err(anyhow!(
                "failed to discover agent: HTTP {}",
                resp.status()
            ));
        }
        let card = resp.json::<AgentCard>().await?;
        Ok(card)
    }

    /// 发送一次性任务，并返回 `(task_id, output)`。
    pub async fn send_task(&self, text: &str) -> Result<(String, String)> {
        let req = TaskCreateRequest {
            input: text.to_string(),
        };

        let url = format!("{}/tasks", self.base_url.trim_end_matches('/'));
        let resp = self.http.post(url).json(&req).send().await?;
        if !resp.status().is_success() {
            return Err(anyhow!(
                "failed to send task: HTTP {}",
                resp.status()
            ));
        }

        let data: TaskCreateResponse = resp.json().await?;
        Ok((data.id, data.output))
    }

    /// 订阅任务的 SSE 流，并以字符串切片形式返回所有 `data:` 块。
    pub async fn subscribe_task(&self, task_id: &str) -> Result<Vec<String>> {
        let url = format!(
            "{}/tasks/{}/stream",
            self.base_url.trim_end_matches('/'),
            task_id
        );

        let resp = self.http.get(url).send().await?;
        if !resp.status().is_success() {
            return Err(anyhow!(
                "failed to subscribe task stream: HTTP {}",
                resp.status()
            ));
        }

        let body = resp.text().await?;
        let mut chunks = Vec::new();

        for line in body.lines() {
            if let Some(rest) = line.strip_prefix("data:") {
                chunks.push(rest.trim().to_string());
            }
        }

        Ok(chunks)
    }
}
