# 0283 — Make MCP server calls always finish and health checks read-only

Status: COMPLETE.
Landed: 8a4dd276390f1acef722c3a549a9735f0de771c5

Milestone: 5

## Outcome

A stdio tool worker that panics still sends its completion with an internal error, so its slot frees and the server exits at end of input. A cancelled stdio call stops its work and frees capacity. An oversized frame waits for active calls to reply before the server stops. HTTP at capacity answers 503 instead of resetting the connection, and `/health` makes no database write. An offline tool call under `serve --observe` reuses the server's observations file.

- Why: the milestone 4 code review found unanswered calls after a panic, lost replies on an oversized frame, connection resets at capacity, a write on every health check and a local error for offline calls under `--observe`.
- Starts from: `crates/biomcp/src/mcp.rs` lines 57, 84 to 86, 118 to 124, 149 to 154 and 179, `mcp/http.rs` lines 100 to 102, `store/mod.rs` lines 40 to 81, `command.rs` lines 91 to 115, `grammar.rs` line 385 and `observe.rs` lines 147 to 153.
- Keeps: the eight-call capacity, framing limits and every existing MCP case.
- Changes: the worker guard, cancellation, oversized-frame shutdown, HTTP capacity reply, a read-only health store open and observations reuse.
- Proof: one case per behavior: an injected test-only panic, a cancel that frees a slot, an oversized frame after a slow call, a ninth HTTP request getting 503, health on a read-only cache folder, and an offline call under `--observe`.
- Defers: nothing.
