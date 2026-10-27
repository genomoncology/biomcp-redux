# 0296 — Check store ownership within one read snapshot

Status: OPEN.

Milestone: 5

## Outcome

The shared store ownership check reads owner metadata and table existence within one read snapshot, so a concurrent first-time initialization can never make a valid store look orphaned or at the wrong version.

- Why: 0286 routine j16229 failed one WikiPathways concurrent reader case with exit 5 and `local_error=version` while 1,745 other cases passed. The RCA found that `check_owners` reads owner metadata and table existence in separate autocommit snapshots, so an initialization committed between the two reads gives a false orphan or version rejection. The race is reachable in code; whether it explains j16229 and the three earlier opaque failures (j14991, j16003, j16035) waits on the bounded two-connection witness, and those failures are not reassigned without it. PM ruling 2026-10-26 (m8126 ask 26).
- Starts from: the shared `check_owners` validation used by the snapshot and bundle stores, the 0287 typed local cache errors and held-lock cases, and the coordinator's two-connection diagnosis job.
- Rules: an SQL error while reading the version keeps its normal SQL classification and is not reported as a version problem. A present malformed version keeps its existing contract.
- Keeps: existing caller transactions, the immediate recheck under the write lock, and the 0287 typed errors.
- Changes: the shared ownership validation and its cases.
- Proof: a deterministic two-connection case commits initialization between the old reads and the store is accepted; a real orphan and a real wrong version are still refused; a version query SQL error is reported as an SQL error. No source calls.
- Defers: the 0286 formatting scope, which stays separate.
