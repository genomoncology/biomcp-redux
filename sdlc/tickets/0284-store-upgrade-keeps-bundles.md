# 0284 — Keep synced bundles and cached replies across query store upgrades

Status: COMPLETE.
Landed: a9893ff35df0e894a9b8321ccbcf4c280f0cbf85

Milestone: 5

## Outcome

A new query store version migrates its tables in place or lives in its own file, so an upgrade keeps synced bundles and the reply cache. `store clean` removes a database's `-wal` and `-shm` files with it, and expired replies are pruned during normal writes so the cache file stays bounded.

- Why: the milestone 4 code review found that the store version is part of the cache file name, so every version bump leaves WHO, GTR, FDA CDx, cBioPortal and WikiPathways bundles missing until resynced, that `store clean` orphans SQLite sidecars, and that nothing bounds cache size.
- Starts from: `crates/biomcp/src/paths.rs` lines 15 to 37, `cache.rs` line 181, `bundle/store.rs` line 59, `cleanup.rs` lines 23 to 64, the transport migration tests and architecture sections 7.1 and 7.2.
- Keeps: SQLite ownership checks before any journal or schema change and the tested transport migrations.
- Changes: store file layout or migrations, cleanup and pruning.
- Proof: a case syncs a synthetic bundle under version N, opens version N plus 1 and reads it without resync; a case asserts `store clean` leaves no sidecar; a case asserts expired rows are pruned after writes. No source calls.
- Defers: nothing.
