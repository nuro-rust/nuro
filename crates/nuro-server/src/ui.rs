pub fn playground_html() -> String {
    r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>Nuro Playground</title>
  <style>
    :root {
      --bg: #f3efe3;
      --panel: rgba(255, 252, 246, 0.84);
      --ink: #1f1a17;
      --accent: #c45b37;
      --accent-soft: #f4c8b9;
      --edge: rgba(31, 26, 23, 0.12);
    }
    * { box-sizing: border-box; }
    body {
      margin: 0;
      font-family: "Avenir Next", "Segoe UI", sans-serif;
      color: var(--ink);
      background:
        radial-gradient(circle at top left, rgba(196, 91, 55, 0.22), transparent 34%),
        radial-gradient(circle at bottom right, rgba(60, 112, 106, 0.18), transparent 30%),
        linear-gradient(135deg, #efe6d1, #f8f4eb 58%, #efe8dc);
      min-height: 100vh;
      padding: 24px;
    }
    .shell {
      max-width: 1040px;
      margin: 0 auto;
      display: grid;
      gap: 18px;
    }
    .hero, .panel {
      background: var(--panel);
      border: 1px solid var(--edge);
      border-radius: 24px;
      backdrop-filter: blur(14px);
      box-shadow: 0 18px 52px rgba(31, 26, 23, 0.08);
    }
    .hero { padding: 28px; }
    .hero h1 {
      font-family: Georgia, "Times New Roman", serif;
      font-size: clamp(2.2rem, 6vw, 4.2rem);
      margin: 0 0 8px;
      line-height: 0.96;
    }
    .hero p {
      margin: 0;
      max-width: 760px;
      font-size: 1rem;
      opacity: 0.82;
    }
    .grid {
      display: grid;
      gap: 18px;
      grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    }
    .panel { padding: 20px; }
    label {
      display: block;
      margin: 0 0 10px;
      font-size: 0.86rem;
      text-transform: uppercase;
      letter-spacing: 0.08em;
      opacity: 0.78;
    }
    input, textarea, select {
      width: 100%;
      border: 1px solid rgba(31, 26, 23, 0.16);
      border-radius: 14px;
      padding: 12px 14px;
      font: inherit;
      background: rgba(255, 255, 255, 0.84);
      color: var(--ink);
    }
    textarea { min-height: 132px; resize: vertical; }
    .row {
      display: grid;
      grid-template-columns: 1fr 1fr;
      gap: 12px;
    }
    button {
      border: 0;
      border-radius: 999px;
      padding: 12px 18px;
      font: inherit;
      cursor: pointer;
      color: white;
      background: linear-gradient(135deg, #b44724, #d06d42);
      box-shadow: 0 10px 20px rgba(196, 91, 55, 0.28);
    }
    .muted {
      background: linear-gradient(135deg, #476d68, #69948d);
    }
    pre {
      margin: 0;
      white-space: pre-wrap;
      word-break: break-word;
      font-family: "SFMono-Regular", Consolas, monospace;
      font-size: 0.9rem;
      line-height: 1.5;
    }
    .controls {
      display: flex;
      flex-wrap: wrap;
      gap: 10px;
      margin-top: 12px;
    }
    .hint {
      margin-top: 10px;
      font-size: 0.92rem;
      opacity: 0.72;
    }
    .toggle {
      display: flex;
      align-items: center;
      gap: 8px;
      margin: 10px 0 0;
      font-size: 0.95rem;
      text-transform: none;
      letter-spacing: 0;
    }
    @media (max-width: 720px) {
      body { padding: 16px; }
      .row { grid-template-columns: 1fr; }
    }
  </style>
</head>
<body>
  <main class="shell">
    <section class="hero">
      <h1>Runtime Playground</h1>
      <p>Drive agent, MCP, and A2A tasks through the same runtime gateway. Approval gates, replay, and trace views are all exposed from the same session model.</p>
    </section>
    <section class="grid">
      <section class="panel">
        <label for="session">Session Id</label>
        <input id="session" placeholder="Leave blank to create one automatically" />

        <div class="row" style="margin-top:12px;">
          <div>
            <label for="protocol">Protocol</label>
            <select id="protocol">
              <option value="agent">agent</option>
              <option value="mcp">mcp</option>
              <option value="a2a">a2a</option>
            </select>
          </div>
          <div>
            <label for="approval">Approval</label>
            <label class="toggle"><input id="approval" type="checkbox" /> Require manual approval</label>
          </div>
        </div>

        <label for="input" style="margin-top:12px;">Input</label>
        <textarea id="input" placeholder="Try: calc: 4 * (3 + 2)"></textarea>

        <div class="row" id="interop-row" style="margin-top:12px;">
          <div>
            <label for="mcp-tool">MCP Tool</label>
            <input id="mcp-tool" value="calculator" />
          </div>
          <div>
            <label for="a2a-url">A2A URL</label>
            <input id="a2a-url" placeholder="http://127.0.0.1:4000" />
          </div>
        </div>

        <label for="mcp-args" style="margin-top:12px;">MCP Arguments JSON</label>
        <textarea id="mcp-args">{"expression":"3 * 7"}</textarea>

        <div class="controls">
          <button id="run">Submit Task</button>
          <button id="replay" class="muted">Fetch Replay</button>
        </div>
        <p class="hint">Trace viewer lives at <code>/trace</code>. The replay button uses the current session id and returns raw event JSON.</p>
      </section>

      <section class="panel">
        <label>Result</label>
        <pre id="result">Waiting for a request…</pre>
      </section>
    </section>
  </main>

  <script>
    const session = document.getElementById("session");
    const protocol = document.getElementById("protocol");
    const approval = document.getElementById("approval");
    const input = document.getElementById("input");
    const mcpTool = document.getElementById("mcp-tool");
    const mcpArgs = document.getElementById("mcp-args");
    const a2aUrl = document.getElementById("a2a-url");
    const result = document.getElementById("result");

    async function submit() {
      const target = protocol.value === "mcp"
        ? { protocol: "mcp", tool_name: mcpTool.value, arguments: JSON.parse(mcpArgs.value || "{}") }
        : protocol.value === "a2a"
          ? { protocol: "a2a", url: a2aUrl.value }
          : { protocol: "agent" };

      const payload = {
        session_id: session.value || null,
        input: input.value,
        target,
        requires_approval: approval.checked,
      };

      const response = await fetch("/v1/tasks", {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify(payload),
      });

      const body = await response.text();
      result.textContent = body;
      try {
        const parsed = JSON.parse(body);
        if (parsed.session_id) {
          session.value = parsed.session_id;
        }
      } catch (_) {}
    }

    async function replaySession() {
      if (!session.value) {
        result.textContent = "Set or create a session first.";
        return;
      }
      const response = await fetch(`/v1/replay/sessions/${encodeURIComponent(session.value)}`);
      result.textContent = await response.text();
    }

    document.getElementById("run").addEventListener("click", submit);
    document.getElementById("replay").addEventListener("click", replaySession);
  </script>
</body>
</html>
"#
    .to_string()
}

pub fn trace_viewer_html() -> String {
    r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>Nuro Trace Viewer</title>
  <style>
    :root {
      --bg: #10181f;
      --panel: #14212b;
      --line: rgba(255,255,255,0.1);
      --ink: #e4eef4;
      --accent: #9ad1c6;
      --accent-2: #f0a868;
    }
    * { box-sizing: border-box; }
    body {
      margin: 0;
      min-height: 100vh;
      font-family: "IBM Plex Sans", "Segoe UI", sans-serif;
      background:
        radial-gradient(circle at top left, rgba(154, 209, 198, 0.18), transparent 26%),
        radial-gradient(circle at bottom right, rgba(240, 168, 104, 0.16), transparent 28%),
        var(--bg);
      color: var(--ink);
      padding: 24px;
    }
    .frame {
      max-width: 960px;
      margin: 0 auto;
      display: grid;
      gap: 18px;
    }
    .panel {
      background: rgba(20, 33, 43, 0.92);
      border: 1px solid var(--line);
      border-radius: 24px;
      padding: 22px;
    }
    h1 {
      margin: 0 0 8px;
      font-size: clamp(2rem, 5vw, 3.4rem);
      line-height: 0.98;
    }
    p { margin: 0; opacity: 0.72; }
    input, button {
      font: inherit;
      border-radius: 999px;
      border: 1px solid var(--line);
      padding: 12px 16px;
    }
    input {
      width: 100%;
      background: rgba(255,255,255,0.06);
      color: var(--ink);
    }
    button {
      background: linear-gradient(135deg, var(--accent), #5cb4a5);
      color: #082227;
      cursor: pointer;
      border: 0;
      font-weight: 600;
    }
    .controls {
      display: grid;
      grid-template-columns: 1fr auto;
      gap: 12px;
      margin-top: 14px;
    }
    .event {
      border-left: 3px solid var(--accent);
      padding: 12px 14px;
      margin-top: 12px;
      background: rgba(255,255,255,0.03);
      border-radius: 14px;
    }
    .event strong { color: var(--accent-2); }
    pre {
      margin: 8px 0 0;
      white-space: pre-wrap;
      word-break: break-word;
      font-family: "SFMono-Regular", Consolas, monospace;
      font-size: 0.88rem;
    }
    @media (max-width: 720px) {
      body { padding: 16px; }
      .controls { grid-template-columns: 1fr; }
    }
  </style>
</head>
<body>
  <main class="frame">
    <section class="panel">
      <h1>Trace Viewer</h1>
      <p>Inspect replayable runtime events for a session. This viewer reads the same replay endpoint used by automated diagnostics.</p>
      <div class="controls">
        <input id="session-id" placeholder="session-1" />
        <button id="load">Load Session</button>
      </div>
    </section>
    <section class="panel" id="timeline">
      Enter a session id to inspect the replay log.
    </section>
  </main>

  <script>
    async function loadSession() {
      const sessionId = document.getElementById("session-id").value.trim();
      const timeline = document.getElementById("timeline");
      if (!sessionId) {
        timeline.textContent = "A session id is required.";
        return;
      }

      const response = await fetch(`/v1/replay/sessions/${encodeURIComponent(sessionId)}`);
      const data = await response.json();

      if (!response.ok) {
        timeline.textContent = JSON.stringify(data, null, 2);
        return;
      }

      timeline.innerHTML = data.events.map((event) => {
        const kind = Object.keys(event.kind)[0];
        return `<article class="event"><strong>${kind}</strong><div>${event.timestamp_ms}</div><pre>${JSON.stringify(event, null, 2)}</pre></article>`;
      }).join("") || "No events for this session.";
    }

    document.getElementById("load").addEventListener("click", loadSession);
  </script>
</body>
</html>
"#
    .to_string()
}
