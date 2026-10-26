# 0282 — Refuse state-changing and quota-spending actions over HTTP MCP and file replacement over MCP

Status: COMPLETE.
Landed: 0b77eccdce199bdbf990dfa8b7ee0c770d0b68f1

Milestone: 4

## Outcome

Over HTTP MCP, `sync`, `store clean` and `--fresh` are refused with a usage error, as saves and key registration already are. Over stdio MCP, `--save` with `--replace` is refused, so an agent cannot overwrite an existing file. README and help say that `--allow-host` and the Host allowlist do not authenticate callers.

- Why: the milestone 4 code review found that any peer that can reach the HTTP listener and send an allowed Host header can sync 128 MB bundles, spend the operator's key quota with `--fresh` and delete cache files, and that a prompt-injected stdio agent can replace any regular file the user owns with downloaded bytes.
- Starts from: `crates/biomcp/src/mcp/http.rs` lines 194 to 209, `command.rs` lines 62 to 67 and 336 to 366, `save.rs` lines 27 to 93, `command/sync.rs`, `store/cleanup.rs` and README's HTTP section.
- Keeps: every read action over both transports, CLI behavior unchanged, and new-file saves over stdio MCP.
- Changes: the MCP action policy, its refusal messages, help and README.
- Proof: HTTP cases assert each refused action returns its usage error with no request, write or deletion; a stdio case asserts `--save PATH --replace` is refused and leaves an existing file unchanged; a CLI case asserts `--replace` still works.
- Defers: bearer-token authentication for HTTP, which waits for a deployment that needs it.
