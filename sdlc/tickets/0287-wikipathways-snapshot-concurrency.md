# 0287 — Diagnose opaque native concurrency failures in snapshot and bundle stores

Status: OPEN.

Milestone: 4

Depends on: 0281

## Outcome

A bounded diagnosis names why `wikipathways/concurrent-readers-during-pair-replacement` exited 5 in j16003 while its provider state was ok, or adds the witness that will name it next time. A witnessed defect gets a fix and a deterministic case that fails without it.

- Why: the case failed once in j16003 and passed unchanged in j16017, and 0255's earlier diagnosis of the same case found no cause, so this is a recurrence that a passing retry does not settle.
- Starts from: the j16003 failure and j16017 pass, 0255's added status and stdout retention, the possible post-header paths through snapshot `Store::open` and `save_source_snapshot`, and 0281's busy-cache handling.
- Rules: the case keeps its assertions. The diagnosis first checks whether 0281's busy-read and best-effort admission already explain the exit, and says so with evidence instead of assuming it. The failing envelope gains its local error kind so a SQLite busy or lock error is visible.
- Proof: a deterministic case that holds the snapshot store's write lock during a reader reproduces the exit before the fix and passes after it, or the ticket records that no such reproduction exists and the case now retains the local error kind. No source calls and no blind retry loop.
- Changes: the case's retained evidence, the local error kind in the envelope if missing, and the snapshot store code only if a defect is witnessed.
- Defers: nothing.
- PM ruling 2026-10-26 (m8126 ask 13): scope widens to the three opaque native concurrency failures, j14991 WikiPathways writer, j16003 WikiPathways reader and j16035 FDA CDx writer, whose unchanged retries passed. Typed local cache errors are kept at the common output boundary so every envelope names a busy or locked store. Deterministic held-store-lock cases cover the reader and sync envelopes, including the FDA CDx writer. The hard policy for an external lock holder stays. SQLite zero-wait or startup contention is a hypothesis until a case reproduces it.
