use nuro_core::{NuroError, Result};
use rusqlite::{Connection, params};
use serde_json::Value;

pub trait CheckpointStore: Send + Sync {
    fn save_checkpoint(&self, session_id: &str, node_id: &str, state: &Value) -> Result<()>;
    fn load_checkpoint(&self, session_id: &str, node_id: &str) -> Result<Option<Value>>;
}

pub struct SqliteCheckpointStore {
    db_path: String,
}

impl SqliteCheckpointStore {
    pub fn new(db_path: impl Into<String>) -> Result<Self> {
        let db_path = db_path.into();
        let conn = Connection::open(&db_path).map_err(|e| NuroError::Storage(e.to_string()))?;
        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS nuro_checkpoints (
                session_id TEXT NOT NULL,
                node_id TEXT NOT NULL,
                payload TEXT NOT NULL,
                PRIMARY KEY(session_id, node_id)
            );
            ",
        )
        .map_err(|e| NuroError::Storage(e.to_string()))?;
        Ok(Self { db_path })
    }

    fn connect(&self) -> Result<Connection> {
        Connection::open(&self.db_path).map_err(|e| NuroError::Storage(e.to_string()))
    }
}

impl CheckpointStore for SqliteCheckpointStore {
    fn save_checkpoint(&self, session_id: &str, node_id: &str, state: &Value) -> Result<()> {
        let conn = self.connect()?;
        let payload =
            serde_json::to_string(state).map_err(|e| NuroError::Storage(e.to_string()))?;
        conn.execute(
            "INSERT OR REPLACE INTO nuro_checkpoints(session_id, node_id, payload) VALUES (?1, ?2, ?3)",
            params![session_id, node_id, payload],
        )
        .map_err(|e| NuroError::Storage(e.to_string()))?;
        Ok(())
    }

    fn load_checkpoint(&self, session_id: &str, node_id: &str) -> Result<Option<Value>> {
        let conn = self.connect()?;
        let mut stmt = conn
            .prepare("SELECT payload FROM nuro_checkpoints WHERE session_id = ?1 AND node_id = ?2")
            .map_err(|e| NuroError::Storage(e.to_string()))?;

        let payload = stmt.query_row(params![session_id, node_id], |row| row.get::<_, String>(0));

        match payload {
            Ok(raw) => {
                let value = serde_json::from_str::<Value>(&raw)
                    .map_err(|e| NuroError::Storage(e.to_string()))?;
                Ok(Some(value))
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(NuroError::Storage(e.to_string())),
        }
    }
}
