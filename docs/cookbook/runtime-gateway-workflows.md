# Runtime Gateway Cookbook

This cookbook covers the three baseline workflows needed for the platformized server surface.

## 1. Local Agent With Replay

Use the default agent target when you want the runtime gateway to own execution, tracing, and replay.

```json
{
  "input": "calc: 12 / 3"
}
```

Expected outcome:

- task reaches `completed`
- replay contains `LlmRequest`, `LlmResponse`, `ToolCallStart`, and `ToolCallEnd`
- `/metrics` increments token and task counters

## 2. Approval And Resume

Use explicit approval when a request must be reviewed before execution.

Submit:

```json
{
  "input": "prepare customer-facing announcement",
  "requires_approval": true
}
```

Approve:

```json
{
  "actor": "reviewer",
  "comment": "approved for release"
}
```

Expected outcome:

- task first lands in `pending_approval`
- checkpoint persists the task payload under the session/task key
- approval or resume changes the task back to `running`, then `completed`
- audit trail records the pause and resume actions

## 3. Cross-Protocol Interop

The same task model now supports both MCP and A2A targets.

MCP:

```json
{
  "input": "calculate through mcp",
  "target": {
    "protocol": "mcp",
    "tool_name": "calculator",
    "arguments": { "expression": "9 * 9" }
  }
}
```

A2A:

```json
{
  "input": "remote hello",
  "target": {
    "protocol": "a2a",
    "url": "http://127.0.0.1:4107"
  }
}
```

Expected outcome:

- both flows write replayable runtime events into the same session event store
- protocol boundaries are visible as `mcp::...` or `a2a::...` tool events
- the task/audit/metrics model stays unchanged across protocols
