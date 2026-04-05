CREATE TABLE IF NOT EXISTS nuro_events (
    event_id TEXT PRIMARY KEY,
    session_id TEXT,
    run_id TEXT,
    payload JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_nuro_events_session_id ON nuro_events(session_id);
CREATE INDEX IF NOT EXISTS idx_nuro_events_session_run_id ON nuro_events(session_id, run_id);

CREATE TABLE IF NOT EXISTS nuro_checkpoints (
    session_id TEXT NOT NULL,
    node_id TEXT NOT NULL,
    payload JSONB NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY(session_id, node_id)
);
