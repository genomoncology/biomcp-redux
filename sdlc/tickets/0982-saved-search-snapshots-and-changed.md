# 0982 — saved-search-snapshots-and-changed

Status: OPEN.

Milestone: 9

## Outcome

A user runs `saved changed [NAME]` and learns which IDs a saved search gained, lost or changed since its last check, with the states `first`, `unchanged`, `changed`, `not_due`, `rebaselined` and `incomplete` that `sdlc/planning/saved-work.md` (Change detection, Records: `biomcp.saved-snapshot/1`, `biomcp.change-report/1`) defines.

## Why

Step 3 of the saved-work design's Order; 0831 built step 1 (landed 2026-11-15).

## Proof

- Proof: two recorded days of one trial search with hand-counted adds, removals and updates; the report equals the hand count.
- Proof: the same replay under two row schemas reports `rebaselined`, never every row as changed.
- Proof: inside the once-an-hour floor `saved changed` answers `not_due` and runs nothing; a re-check reads the source, not the reply cache, through its own reviewed path (hosted refuses `--fresh`).

## Starts from

- Starts from: 0831's store (`crates/biomcp-query/src/store/saved/`), its migration rule and compatibility key; the design's Change detection section.

## Keeps

- Keeps: 0831's compatibility rules (additive migrations only); two newest snapshots per search; no source content stored, only IDs and fingerprints.

## Defers

- Defers: nothing.

## Changes

- Changes: the saved-work code and its cases.

## Progress
