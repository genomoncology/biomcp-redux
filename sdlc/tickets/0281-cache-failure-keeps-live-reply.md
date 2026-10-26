# 0281 — Keep a checked live reply when the cache is busy or fails

Status: OPEN.

Milestone: 4

## Outcome

A live reply that decoded and passed its check is returned even when storing it, storing a 404, or removing a stale entry fails, and the outcome carries a cache problem instead of a failed source. A busy cache read is treated as a miss and goes live. A bundle import no longer holds the process-wide cache connection for its whole insert.

- Why: the milestone 4 code review found that `cache.write(...)?` in `admit_live` turns a successful provider answer into a local failure with exit 5 when another process holds the write lock, as a bundle import does for its whole transaction.
- Starts from: `crates/biomcp-fetch/src/cache_request.rs` lines 95, 119 to 125 and 170 to 178, `cache.rs` lines 195 to 227, `bundle/store.rs` lines 167 to 188, `status.rs` lines 47 to 49 and `command.rs` lines 525 to 529.
- Keeps: admission only after decode and check, credential containment before any write, replay behavior, and deadline-bounded waits.
- Changes: cache admission and removal error handling, the busy-read path, and bundle import through its own connection in batched transactions.
- Proof: a case holds `BEGIN IMMEDIATE` on the cache from another connection and asserts `get gene BRAF` from replay returns the gene with a cache problem and exit 0, a second asserts a busy read goes live, and a third asserts a concurrent bundle import does not fail another leg before its deadline.
- Defers: nothing.
- PM ruling 2026-10-26 (m8126 ask 4): because replay HGNC republishes its bundle and fails under a held write lock before any live reply, the CLI witness uses an existing replay command that reads no bundle and asserts exit 0 with a cache problem. An initialized MCP server that seeds, takes the lock, then gets a gene witnesses checked live admission, and direct transport cases cover the busy read, write, 404 and removal paths. Strict replay is unchanged. Bundle import stages in a connection-local TEMP table and publishes in one atomic durable step.
