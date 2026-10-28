# 0398 — Recover bundle part files after an interrupted sync

Status: COMPLETE.
Landed: e2132dd471d61a4a355c03b5b97a028d45957247

Milestone: 5

## Outcome

A bundle sync interrupted by a signal or kill leaves no orphaned part file: when a later claim finds an expired holder, it resumes from or deletes that holder's part file after the existing path checks. A released row whose part file is simply missing inside the owned parts folder is treated as a fresh claim, while symlinks, paths outside the folder and wrong names are still refused.

- Why: pre-release review `sdlc/records/2026-10-28-pre-release-review.md`, code findings 1 and 2. Today an interrupted sync leaves up to 128 MiB per part forever, and a missing part file makes every later sync of that bundle fail with a local error.
- Starts from: `crates/biomcp-fetch/src/bundle/download.rs`, `bundle/store.rs` and `crates/biomcp-fetch/tests/bundle/download.rs`.
- Rules: no new framework or signal handler unless the fix needs one; keep the lease and validation rules.
- Proof: a case kills a sync mid-download, expires the lease and shows the next sync leaves no orphan; the `missing` scenario now expects recovery; refusal cases for symlinks and outside paths still pass.
- Changes: the store claim logic and its cases.
- Defers: nothing.
