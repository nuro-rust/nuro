# Runtime Server Gateway

`nuro-server` is no longer just the demo chat surface. The runtime gateway now exposes:

- `POST /v1/sessions`: create or upsert a session with metadata
- `GET /v1/sessions/:id`: inspect session state and task membership
- `POST /v1/tasks`: execute agent, MCP, or A2A work in a replayable session
- `GET /v1/tasks/:id`: inspect task status, audit trail, and token usage
- `POST /v1/tasks/:id/approve`: continue an approval-gated task
- `POST /v1/tasks/:id/interrupt`: pause a pending task before execution
- `POST /v1/tasks/:id/resume`: resume an interrupted task from the persisted checkpoint
- `GET /v1/replay/sessions/:id`: fetch replayable runtime events
- `GET /metrics`: Prometheus-style counters and gauges
- `GET /playground`: browser client for manual workflow testing
- `GET /trace`: minimal trace viewer for replay inspection

## Storage

`nuro-runtime` now ships two store backends on the same traits:

- `SqliteEventStore` / `SqliteCheckpointStore`
- `PostgresEventStore` / `PostgresCheckpointStore`

Postgres schema creation is handled by [`001_create_runtime_tables.sql`](/Users/bytedance/workspace/nuro/crates/nuro-runtime/migrations/postgres/001_create_runtime_tables.sql).

## Observability

The gateway records:

- session id
- task id
- run id
- protocol (`agent`, `mcp`, `a2a`)
- estimated input/output tokens
- approval and policy audit actions

These fields are emitted through `tracing` and surfaced back in task records plus `/metrics`.

## Example Requests

Create a session:

```bash
curl -s http://127.0.0.1:3000/v1/sessions \
  -H 'content-type: application/json' \
  -d '{"metadata":{"tenant":"demo"}}'
```

Run a replayable agent task:

```bash
curl -s http://127.0.0.1:3000/v1/tasks \
  -H 'content-type: application/json' \
  -d '{"input":"calc: 8 * 5"}'
```

Run an approval-gated task:

```bash
curl -s http://127.0.0.1:3000/v1/tasks \
  -H 'content-type: application/json' \
  -d '{"input":"draft release note","requires_approval":true}'
```

Run an MCP task:

```bash
curl -s http://127.0.0.1:3000/v1/tasks \
  -H 'content-type: application/json' \
  -d '{"input":"calculate via mcp","target":{"protocol":"mcp","tool_name":"calculator","arguments":{"expression":"6 * 7"}}}'
```

Replay a session:

```bash
curl -s http://127.0.0.1:3000/v1/replay/sessions/session-1
```
