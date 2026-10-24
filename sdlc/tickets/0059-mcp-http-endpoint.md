# 0059 — Serve MCP over streamable HTTP with a health endpoint

Status: COMPLETE.
Landed: 8d79b3ab97d42b2a1de2628a82fd8e71ffe31c4e

Milestone: 1

Depends on: 0056
Depends on: 0057

## Outcome

`biomcp serve --http [ADDR]` serves the one MCP tool over MCP's streamable HTTP transport at `/mcp` and answers `GET /health` with a small JSON body naming the version and whether the store opens. It binds to 127.0.0.1:8080 by default. A non-loopback address needs `--allow-host HOST` (repeatable) or the config's `http.allowed_hosts`, and requests with any other `Host` or `Origin` are refused. Hosted 0.9 users on notebooks, agent frameworks and reverse proxies used this (issues 45, 72, 117, 185 and 240).

- Starts from: `decisions/2026-10-24-1-0-serves-mcp-over-http.md`; architecture 8.7 (both MCP revisions) and 8.9; the stdio server in `crates/biomcp/src/mcp.rs`; the thread rules in section 11.
- PM decision: 2026-10-24. One small blocking HTTP server crate with a license on the allowlist, chosen and named in architecture 2.2 by this ticket. Handlers call the same parse and run as stdio, and the same schemas and statuses apply. Each request runs under the command budget. No sessions beyond what the transport requires. `/health` makes no source request; `biomcp health` (0057) is the command that checks sources.
- Proof: runner cases drive `/mcp` over HTTP for both MCP revisions (discover or initialize, `tools/list`, one replayed `tools/call`) and assert the same result as stdio. `/health` returns its schema-checked body. A request with a foreign `Host` is refused. A non-loopback bind without an allowlist is a usage error. Two concurrent calls both succeed.
- Defers: the wiki at `/`, TLS termination (left to a reverse proxy), and authentication.

- Reviewed reliability boundary, 2026-10-24: Keep RPC validation/execution shared but separate HTTP client transport state: equal IDs and legacy/modern revisions must not collide or change another client. Prefer stateless single-object JSON responses, no minted sessions/resumability/server notifications, no SSE unless explicitly implemented. Enforce each revision HTTP metadata/header/body rules, notification202 and method handling. Parse Host authorities and Origin URLs separately; normalize case/bracketedIPv6, reject malformed/duplicate/conflicting values before admission, require absent Origin and refuse browser Origin even on loopback. Check loopback Host, ignore forwarded headers, never use wildcard CORS; apply checks to health/errors too. Select/pin a blocking server dependency only after proving preallocation header/body caps, slow-read timeouts, connection/worker bounds and test shutdown; no handwritten HTTP parser or async repair. Bound pre-admission reads/queue waits, then preserve admitted command deadline. Never hold global locks across work. Do not describe existing response-suppression cancellation as query interruption. HTTP health uses the same owned Fetch/cache/query-store validation with bounded budget, no source traffic or secret/path/SQL output, recommended versioned200/503 result. Add interleaved revisions/equalIDs, metadata/method/notification/overload, Host/Origin/IPv6/forwarding and concurrent health/storage-refusal proofs through real isolated sockets.0056 and0057 precede HTTP; final0058 artifact gate follows it, packaging preparation may overlap.

- PM ruling m4402: native HTTP clients and reverse proxies only, stateless JSON responses, CORS disabled. Refuse a browser Origin; browser support requires a future consumer and explicit permitted-origin contract.

- Coordinator dependency review, 2026-10-24: HTTP consumes the shared configuration and Fetch initialization from 0056 and the implemented-source and health interfaces from 0057. These are real API dependencies. Preparation may stack once those early interfaces are published; the final installed-artifact HTTP proof follows server availability.
