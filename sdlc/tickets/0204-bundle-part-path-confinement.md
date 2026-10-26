# 0204 — Confine bundle recovery to its own part files

Status: COMPLETE.
Landed: 2539af89881276f4cd44ed615977671a30d48f2f

Milestone: 1

Depends on: none

## Outcome

A bundle sync never deletes a file outside its own parts folder. The 0197 review found that `crates/biomcp-fetch/src/bundle/store.rs` reads `bundle_sync.part` from SQLite and `bundle/download.rs` removes that path after optional resume, with no check that it lies under the active cache's parts folder. A corrupt or edited row can name an unrelated file for deletion.

- Starts from: `bundle/store.rs` and `bundle/download.rs`.
- Keeps: resume behavior for valid parts and atomic bundle replacement.
- Changes: `crates/biomcp-fetch/src/bundle/**` and its tests.
- Fix: before opening or deleting a recovered part, check that it is a regular file directly under the expected parts folder with the transport's part naming. Otherwise return a local error and leave the path alone.
- Proof: an isolated cache row naming a sentinel file outside the parts folder makes sync fail with a local error and the sentinel survives. A valid prior part still resumes or is removed as before. Offline landing gate passes.
