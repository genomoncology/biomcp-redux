# 0353 — Accept the same MCP protocol revisions on HTTP as on stdio

Status: OPEN.

Milestone: 5

## Outcome

HTTP MCP at `/mcp` accepts every protocol revision that stdio `initialize` accepts, including 2025-06-18, through the legacy initialization edge. A request without an `MCP-Protocol-Version` header is handled as the MCP Streamable HTTP specification's backward-compatibility rule says, not refused with -32020. `biomcp help serve` lists the supported revisions and the header rule.

- Why: the docs lead's second-pass product findings (2026-10-27). HTTP refuses 2025-06-18 with -32022 and a missing header with -32020, while stdio accepts 2025-06-18. Hosted clients that 0.9 served over HTTP may still send 2025-06-18 or no header.
- Starts from: `mcp/protocol.rs` `REVISIONS`, `mcp/legacy.rs`, the HTTP server's header check, the architecture's HTTP section and the pinned specification pages it cites.
- Rules: PM ruling 2026-10-27 (m10617): stdio initialize echoes a known legacy revision and answers 2025-11-25 to an unknown one; HTTP accepts 2026-07-28, 2025-11-25, 2025-06-18 and 2025-03-26. The developer cites the specification text for each accepted revision and for the missing-header rule, and updates the architecture section. The 2026-07-28 request mirror checks stay as they are. Host and Origin checks stay as they are.
- Proof: HTTP cases for initialize and tools/call at 2025-06-18, at 2025-11-25 and with no header succeed; a revision BioMCP does not support returns the specification's error; stdio cases are unchanged.
- Changes: MCP protocol negotiation, the HTTP header check, help text, the architecture section and cases.
- Defers: nothing. Joins the 0336 refresh.
