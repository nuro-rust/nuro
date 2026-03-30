use nuro_core::{Event, NuroError, Result};

use crate::EventStore;

#[derive(Debug, Clone)]
pub enum ReplayMode {
    Strict,
    Lenient,
}

#[derive(Debug, Clone)]
pub struct ReplayResult {
    pub events: Vec<Event>,
    pub warnings: Vec<String>,
}

pub struct ReplayEngine<S>
where
    S: EventStore,
{
    store: S,
}

impl<S> ReplayEngine<S>
where
    S: EventStore,
{
    pub fn new(store: S) -> Self {
        Self { store }
    }

    pub fn replay_session(&self, session_id: &str, mode: ReplayMode) -> Result<ReplayResult> {
        let events = self.store.list_by_session(session_id)?;
        self.validate(events, mode)
    }

    pub fn replay_session_run(
        &self,
        session_id: &str,
        run_id: &str,
        mode: ReplayMode,
    ) -> Result<ReplayResult> {
        let events = self.store.list_by_session_run(session_id, run_id)?;
        self.validate(events, mode)
    }

    fn validate(&self, events: Vec<Event>, mode: ReplayMode) -> Result<ReplayResult> {
        let mut warnings = Vec::new();
        let mut prev_ts: Option<u64> = None;

        for event in &events {
            if event.schema_version != "v1" {
                let msg = format!(
                    "event {} has unsupported schema_version {}",
                    event.event_id, event.schema_version
                );
                match mode {
                    ReplayMode::Strict => return Err(NuroError::InvalidInput(msg)),
                    ReplayMode::Lenient => warnings.push(msg),
                }
            }

            if let Some(prev) = prev_ts
                && event.timestamp_ms < prev
            {
                let msg = format!(
                    "event {} timestamp {} is earlier than previous {}",
                    event.event_id, event.timestamp_ms, prev
                );
                match mode {
                    ReplayMode::Strict => return Err(NuroError::InvalidInput(msg)),
                    ReplayMode::Lenient => warnings.push(msg),
                }
            }

            prev_ts = Some(event.timestamp_ms);
        }

        Ok(ReplayResult { events, warnings })
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use nuro_core::{Event, EventKind};

    use crate::{EventStore, ReplayEngine, ReplayMode, SqliteEventStore};

    fn temp_db(name: &str) -> String {
        let mut path: PathBuf = std::env::temp_dir();
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        path.push(format!("nuro-replay-{name}-{nanos}.db"));
        path.to_string_lossy().to_string()
    }

    #[test]
    fn replay_by_session_run_filters_events() {
        let db = temp_db("session-run");
        let store = SqliteEventStore::new(db.clone()).expect("store");

        let mut e1 = Event::new(EventKind::LlmRequest { messages: vec![] });
        e1.session_id = Some("s1".to_string());
        e1.run_id = Some("r1".to_string());

        let mut e2 = Event::new(EventKind::ToolCallStart {
            tool_name: "t".to_string(),
            input: serde_json::json!({}),
        });
        e2.session_id = Some("s1".to_string());
        e2.run_id = Some("r2".to_string());

        store.append(&e1).expect("append e1");
        store.append(&e2).expect("append e2");

        let engine = ReplayEngine::new(store);
        let result = engine
            .replay_session_run("s1", "r1", ReplayMode::Strict)
            .expect("replay");

        assert_eq!(result.events.len(), 1);
        assert_eq!(result.events[0].run_id.as_deref(), Some("r1"));

        let _ = std::fs::remove_file(db);
    }

    #[test]
    fn lenient_mode_keeps_events_and_collects_warnings() {
        let db = temp_db("lenient");
        let store = SqliteEventStore::new(db.clone()).expect("store");

        let mut e1 = Event::new(EventKind::LlmRequest { messages: vec![] });
        e1.session_id = Some("s2".to_string());
        e1.run_id = Some("r1".to_string());
        e1.schema_version = "v0".to_string();

        let mut e2 = Event::new(EventKind::ToolCallEnd {
            tool_name: "t".to_string(),
            output: serde_json::json!({"ok": true}),
        });
        e2.session_id = Some("s2".to_string());
        e2.run_id = Some("r1".to_string());
        e2.timestamp_ms = e1.timestamp_ms.saturating_sub(10);

        store.append(&e1).expect("append e1");
        store.append(&e2).expect("append e2");

        let engine = ReplayEngine::new(store);
        let result = engine
            .replay_session_run("s2", "r1", ReplayMode::Lenient)
            .expect("replay lenient");

        assert_eq!(result.events.len(), 2);
        assert!(!result.warnings.is_empty());

        let _ = std::fs::remove_file(db);
    }
}
