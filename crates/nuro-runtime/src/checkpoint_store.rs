use std::sync::Arc;

use nuro_core::{NuroError, Result};
use postgres::{Client, NoTls};
use rusqlite::{Connection, params};
use serde_json::Value;

const POSTGRES_MIGRATION: &str =
    include_str!("../migrations/postgres/001_create_runtime_tables.sql");

pub trait CheckpointStore: Send + Sync {
    fn save_checkpoint(&self, session_id: &str, node_id: &str, state: &Value) -> Result<()>;
    fn load_checkpoint(&self, session_id: &str, node_id: &str) -> Result<Option<Value>>;
}

impl<T> CheckpointStore for Arc<T>
where
    T: CheckpointStore + ?Sized,
{
    fn save_checkpoint(&self, session_id: &str, node_id: &str, state: &Value) -> Result<()> {
        (**self).save_checkpoint(session_id, node_id, state)
    }

    fn load_checkpoint(&self, session_id: &str, node_id: &str) -> Result<Option<Value>> {
        (**self).load_checkpoint(session_id, node_id)
    }
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

pub struct PostgresCheckpointStore {
    db_url: String,
}

impl PostgresCheckpointStore {
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

impl CheckpointStore for PostgresCheckpointStore {
    fn save_checkpoint(&self, session_id: &str, node_id: &str, state: &Value) -> Result<()> {
        let mut client = self.connect()?;
        let payload =
            serde_json::to_string(state).map_err(|e| NuroError::Storage(e.to_string()))?;
        client
            .execute(
                "INSERT INTO nuro_checkpoints(session_id, node_id, payload)
                 VALUES ($1, $2, $3::jsonb)
                 ON CONFLICT (session_id, node_id) DO UPDATE
                 SET payload = EXCLUDED.payload,
                     updated_at = NOW()",
                &[&session_id, &node_id, &payload],
            )
            .map_err(|e| NuroError::Storage(e.to_string()))?;
        Ok(())
    }

    fn load_checkpoint(&self, session_id: &str, node_id: &str) -> Result<Option<Value>> {
        let mut client = self.connect()?;
        let row = client
            .query_opt(
                "SELECT payload::text
                 FROM nuro_checkpoints
                 WHERE session_id = $1 AND node_id = $2",
                &[&session_id, &node_id],
            )
            .map_err(|e| NuroError::Storage(e.to_string()))?;

        match row {
            Some(row) => {
                let payload: String = row.get(0);
                let value = serde_json::from_str::<Value>(&payload)
                    .map_err(|e| NuroError::Storage(e.to_string()))?;
                Ok(Some(value))
            }
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

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
            "nuro-checkpoint-store-{name}-{}.db",
            unique_id("sqlite")
        ));
        path.to_string_lossy().to_string()
    }

    fn assert_store_contract<S>(store: &S, session_id: &str) -> Result<()>
    where
        S: CheckpointStore,
    {
        let state = serde_json::json!({
            "status": "interrupted",
            "cursor": 12,
        });

        store.save_checkpoint(session_id, "task-1", &state)?;

        let loaded = store
            .load_checkpoint(session_id, "task-1")?
            .expect("checkpoint should exist");
        assert_eq!(loaded, state);
        assert!(store.load_checkpoint(session_id, "missing")?.is_none());

        Ok(())
    }

    #[test]
    fn sqlite_checkpoint_store_satisfies_contract() {
        let db_path = temp_db("sqlite");
        let store = SqliteCheckpointStore::new(db_path.clone()).expect("sqlite checkpoint store");
        let session_id = unique_id("session");
        assert_store_contract(&store, &session_id).expect("sqlite contract");
        let _ = std::fs::remove_file(db_path);
    }

    #[test]
    fn postgres_checkpoint_store_satisfies_contract_when_url_is_configured() {
        let Ok(url) = std::env::var("NURO_POSTGRES_TEST_URL") else {
            return;
        };

        let store = PostgresCheckpointStore::new(url).expect("postgres checkpoint store");
        let session_id = unique_id("pg-session");
        assert_store_contract(&store, &session_id).expect("postgres contract");
    }
}
