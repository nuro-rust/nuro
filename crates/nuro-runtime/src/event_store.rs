use nuro_core::{Event, NuroError, Result};
use rusqlite::{Connection, params};

pub trait EventStore: Send + Sync {
    fn append(&self, event: &Event) -> Result<()>;
    fn list_by_session(&self, session_id: &str) -> Result<Vec<Event>>;

    fn list_by_session_run(&self, session_id: &str, run_id: &str) -> Result<Vec<Event>> {
        let events = self.list_by_session(session_id)?;
        Ok(events
            .into_iter()
            .filter(|e| e.run_id.as_deref() == Some(run_id))
            .collect())
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
