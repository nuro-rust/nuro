use std::sync::Arc;

use nuro_core::{Event, NuroError, Result};
use postgres::{Client, NoTls};
use rusqlite::{Connection, params};

const POSTGRES_MIGRATION: &str =
    include_str!("../migrations/postgres/001_create_runtime_tables.sql");

pub trait EventStore: Send + Sync {
    fn append(&self, event: &Event) -> Result<()>;
    fn list_by_session(&self, session_id: &str) -> Result<Vec<Event>>;

    fn list_by_session_run(&self, session_id: &str, run_id: &str) -> Result<Vec<Event>> {
        let events = self.list_by_session(session_id)?;
        Ok(events
            .into_iter()
            .filter(|event| event.run_id.as_deref() == Some(run_id))
            .collect())
    }
}

impl<T> EventStore for Arc<T>
where
    T: EventStore + ?Sized,
{
    fn append(&self, event: &Event) -> Result<()> {
        (**self).append(event)
    }

    fn list_by_session(&self, session_id: &str) -> Result<Vec<Event>> {
        (**self).list_by_session(session_id)
    }

    fn list_by_session_run(&self, session_id: &str, run_id: &str) -> Result<Vec<Event>> {
        (**self).list_by_session_run(session_id, run_id)
    }
}

pub struct SqliteEventStore {
    db_path: String,
}

impl SqliteEventStore {
    pub fn new(db_path: impl Into<String>) -> Result<Self> {
        let db_path = db_path.into();
        let conn = Connection::open(&db_path).map_err(|e| NuroError::Storage(e.to_string()))?;
        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS nuro_events (
                event_id TEXT PRIMARY KEY,
                session_id TEXT,
                payload TEXT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_nuro_events_session_id ON nuro_events(session_id);
            ",
        )
        .map_err(|e| NuroError::Storage(e.to_string()))?;
        Ok(Self { db_path })
    }

    fn connect(&self) -> Result<Connection> {
        Connection::open(&self.db_path).map_err(|e| NuroError::Storage(e.to_string()))
    }
}

impl EventStore for SqliteEventStore {
    fn append(&self, event: &Event) -> Result<()> {
        let conn = self.connect()?;
        let payload =
            serde_json::to_string(event).map_err(|e| NuroError::Storage(e.to_string()))?;
        conn.execute(
            "INSERT OR REPLACE INTO nuro_events(event_id, session_id, payload) VALUES (?1, ?2, ?3)",
            params![event.event_id, event.session_id, payload],
        )
        .map_err(|e| NuroError::Storage(e.to_string()))?;
        Ok(())
    }

    fn list_by_session(&self, session_id: &str) -> Result<Vec<Event>> {
        let conn = self.connect()?;
        let mut stmt = conn
            .prepare("SELECT payload FROM nuro_events WHERE session_id = ?1 ORDER BY rowid ASC")
            .map_err(|e| NuroError::Storage(e.to_string()))?;

        let rows = stmt
            .query_map([session_id], |row| row.get::<_, String>(0))
            .map_err(|e| NuroError::Storage(e.to_string()))?;

        let mut events = Vec::new();
        for payload in rows {
            let payload = payload.map_err(|e| NuroError::Storage(e.to_string()))?;
            let event = serde_json::from_str::<Event>(&payload)
                .map_err(|e| NuroError::Storage(e.to_string()))?;
            events.push(event);
        }
        Ok(events)
    }
}

pub struct PostgresEventStore {
    db_url: String,
}

impl PostgresEventStore {
    pub fn new(db_url: impl Into<String>) -> Result<Self> {
        let db_url = db_url.into();
        let mut client =
            Client::connect(&db_url, NoTls).map_err(|e| NuroError::Storage(e.to_string()))?;
        client
            .batch_execute(POSTGRES_MIGRATION)
            .map_err(|e| NuroError::Storage(e.to_string()))?;
        Ok(Self { db_url })
    }

    fn connect(&self) -> Result<Client> {
        Client::connect(&self.db_url, NoTls).map_err(|e| NuroError::Storage(e.to_string()))
    }
}

impl EventStore for PostgresEventStore {
    fn append(&self, event: &Event) -> Result<()> {
        let mut client = self.connect()?;
        let payload =
            serde_json::to_string(event).map_err(|e| NuroError::Storage(e.to_string()))?;
        client
            .execute(
                "INSERT INTO nuro_events(event_id, session_id, run_id, payload)
                 VALUES ($1, $2, $3, $4::jsonb)
                 ON CONFLICT (event_id) DO UPDATE
                 SET session_id = EXCLUDED.session_id,
                     run_id = EXCLUDED.run_id,
                     payload = EXCLUDED.payload",
                &[&event.event_id, &event.session_id, &event.run_id, &payload],
            )
            .map_err(|e| NuroError::Storage(e.to_string()))?;
        Ok(())
    }

    fn list_by_session(&self, session_id: &str) -> Result<Vec<Event>> {
        let mut client = self.connect()?;
        let rows = client
            .query(
                "SELECT payload::text
                 FROM nuro_events
                 WHERE session_id = $1
                 ORDER BY created_at ASC, event_id ASC",
                &[&session_id],
            )
            .map_err(|e| NuroError::Storage(e.to_string()))?;

        rows.into_iter()
            .map(|row| {
                let payload: String = row.get(0);
                serde_json::from_str::<Event>(&payload)
                    .map_err(|e| NuroError::Storage(e.to_string()))
            })
            .collect()
    }

    fn list_by_session_run(&self, session_id: &str, run_id: &str) -> Result<Vec<Event>> {
        let mut client = self.connect()?;
        let rows = client
            .query(
                "SELECT payload::text
                 FROM nuro_events
                 WHERE session_id = $1 AND run_id = $2
                 ORDER BY created_at ASC, event_id ASC",
                &[&session_id, &run_id],
            )
            .map_err(|e| NuroError::Storage(e.to_string()))?;

        rows.into_iter()
            .map(|row| {
                let payload: String = row.get(0);
                serde_json::from_str::<Event>(&payload)
                    .map_err(|e| NuroError::Storage(e.to_string()))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use nuro_core::EventKind;

    use super::*;

    fn unique_id(prefix: &str) -> String {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);
        format!("{prefix}-{nanos}")
    }

    fn temp_db(name: &str) -> String {
        let mut path: PathBuf = std::env::temp_dir();
        path.push(format!(
            "nuro-event-store-{name}-{}.db",
            unique_id("sqlite")
        ));
        path.to_string_lossy().to_string()
    }

    fn sample_event(session_id: &str, run_id: &str, kind: EventKind) -> Event {
        let mut event = Event::new(kind);
        event.session_id = Some(session_id.to_string());
        event.run_id = Some(run_id.to_string());
        event
    }

    fn assert_store_contract<S>(store: &S, session_id: &str) -> Result<()>
    where
        S: EventStore,
    {
        let first = sample_event(
            session_id,
            "run-a",
            EventKind::LlmRequest { messages: vec![] },
        );
        let second = sample_event(
            session_id,
            "run-a",
            EventKind::ToolCallStart {
                tool_name: "calculator".to_string(),
                input: serde_json::json!({"expression": "1 + 1"}),
            },
        );
        let third = sample_event(
            session_id,
            "run-b",
            EventKind::ToolCallEnd {
                tool_name: "calculator".to_string(),
                output: serde_json::json!(2.0),
            },
        );

        store.append(&first)?;
        store.append(&second)?;
        store.append(&third)?;

        let session_events = store.list_by_session(session_id)?;
        assert_eq!(session_events.len(), 3);
        assert_eq!(session_events[0].event_id, first.event_id);
        assert_eq!(session_events[1].event_id, second.event_id);

        let run_events = store.list_by_session_run(session_id, "run-a")?;
        assert_eq!(run_events.len(), 2);
        assert_eq!(run_events[0].event_id, first.event_id);
        assert_eq!(run_events[1].event_id, second.event_id);

        Ok(())
    }

    #[test]
    fn sqlite_event_store_satisfies_contract() {
        let db_path = temp_db("sqlite");
        let store = SqliteEventStore::new(db_path.clone()).expect("sqlite store");
        let session_id = unique_id("session");
        assert_store_contract(&store, &session_id).expect("sqlite contract");
        let _ = std::fs::remove_file(db_path);
    }

    #[test]
    fn postgres_event_store_satisfies_contract_when_url_is_configured() {
        let Ok(url) = std::env::var("NURO_POSTGRES_TEST_URL") else {
            return;
        };

        let store = PostgresEventStore::new(url).expect("postgres store");
        let session_id = unique_id("pg-session");
        assert_store_contract(&store, &session_id).expect("postgres contract");
    }
}
