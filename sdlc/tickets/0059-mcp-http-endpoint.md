# 0059 — Serve MCP over streamable HTTP with a health endpoint

Status: OPEN.

Milestone: 1

## Outcome

`biomcp serve --http [ADDR]` serves the one MCP tool over MCP's streamable HTTP transport at `/mcp` and answers `GET /health` with a small JSON body naming the version and whether the store opens. It binds to 127.0.0.1:8080 by default. A non-loopback address needs `--allow-host HOST` (repeatable) or the config's `http.allowed_hosts`, and requests with any other `Host` or `Origin` are refused. Hosted 0.9 users on notebooks, agent frameworks and reverse proxies used this (issues 45, 72, 117, 185 and 240).

- Starts from: `decisions/2026-10-24-1-0-serves-mcp-over-http.md`; architecture 8.7 (both MCP revisions) and 8.9; the stdio server in `crates/biomcp/src/mcp.rs`; the thread rules in section 11.
- PM decision: 2026-10-24. One small blocking HTTP server crate with a license on the allowlist, chosen and named in architecture 2.2 by this ticket. Handlers call the same parse and run as stdio, and the same schemas and statuses apply. Each request runs under the command budget. No sessions beyond what the transport requires. `/health` makes no source request; `biomcp health` (0057) is the command that checks sources.
- Proof: runner cases drive `/mcp` over HTTP for both MCP revisions (discover or initialize, `tools/list`, one replayed `tools/call`) and assert the same result as stdio. `/health` returns its schema-checked body. A request with a foreign `Host` is refused. A non-loopback bind without an allowlist is a usage error. Two concurrent calls both succeed.
- Defers: the wiki at `/`, TLS termination (left to a reverse proxy), and authentication.
