# 0203 — Remove expired pools and snapshots, and add store clean

Status: OPEN.

Milestone: 1

Depends on: none

## Outcome

Local state stops growing without bound. The 0197 review found that pools and trial and citation snapshots expire only at read time and are never removed, and that architecture 7.2 promises `biomcp store clean` and a size cap that do not exist, while 7.1 says cleanup waits for a consumer. PM ruling 2026-10-26:

- Expired pools and snapshots can never be read again, so the write transaction that publishes a new pool or snapshot also deletes expired pools and snapshots and their dependent rows, in a bounded batch. Live pages are never touched.
- Expired replies stay, because a failed source serves them as `stale`. `biomcp store clean` removes expired replies, expired pools and snapshots, and cache files from older incompatible schema versions, and reports what it removed.
- The config size cap is dropped from 1.0.

- Starts from: `crates/biomcp/src/store/pool.rs`, the trial and citation snapshot stores, `crates/biomcp-fetch/src/cache.rs`, and architecture 7.1 and 7.2.
- Keeps: pool expiry semantics, `--fresh`, stale serving, foreign schema tables untouched, and cache file selection by schema version.
- Changes: the store and cache modules above, the CLI grammar and help for `store clean`, its cases under `tests/cases/**`, `sdlc/planning/architecture.md` 7.1 and 7.2.
- Proof: a store with expired and live pools publishes a new pool; expired rows and their items are gone and live pages still read. `store clean` on a cache with expired and fresh replies removes only expired ones, removes an older incompatible cache file, leaves the current file and foreign tables alone, and prints counts within a measured ceiling. A concurrent reader of a live pool is unaffected. Offline landing gate passes.
- Defers: a size cap and store statistics.
