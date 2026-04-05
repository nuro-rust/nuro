# Runtime Gateway Regression Baseline

The server/runtime regression set now covers the platform gateway paths that were previously missing.

Run:

```bash
cargo test -p nuro-runtime -p nuro-server
```

## Covered Scenarios

1. `tests::agent_tasks_are_completed_and_replayable`
   Validates session creation, replayable local agent execution, and event persistence.

2. `tests::approval_flow_runs_after_manual_approval`
   Validates the approval gate, checkpoint persistence, and resume-to-completion path.

3. `tests::policy_rules_can_reject_requests_and_update_metrics`
   Validates configurable deny policies plus observability counters.

4. `tests::mcp_tasks_are_executed_through_the_gateway`
   Validates MCP protocol execution and protocol-tagged replay events.

5. `tests::a2a_tasks_are_recorded_as_runtime_events`
   Validates A2A interop when the environment allows loopback binding.

## Postgres Verification

Postgres store contract tests are enabled automatically when `NURO_POSTGRES_TEST_URL` is set.

Example:

```bash
export NURO_POSTGRES_TEST_URL=postgres://postgres:postgres@127.0.0.1:5432/nuro
cargo test -p nuro-runtime postgres
```
