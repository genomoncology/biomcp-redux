# 0399 — Keep local cache private and config symlinks intact

Status: OPEN.

Milestone: 5

## Outcome

On Unix, BioMCP creates its cache folder with mode 0700 and its cache database with mode 0600, as it already does for the key file. `key set` writes through a symlinked config file to its target and keeps the file's field order.

- Why: pre-release review `sdlc/records/2026-10-28-pre-release-review.md`, code findings 3 and 7. The cache holds licensed OncoKB replies and query history, and dotfile managers symlink config files.
- Starts from: `crates/biomcp/src/config.rs`, `crates/biomcp-fetch/src/cache.rs` and `crates/biomcp/src/config/key_file.rs`.
- Rules: never print key values; existing folders are not chmodded silently beyond what the key file already does.
- Proof: cases show the new folder and database modes, and that `key set` on a symlinked config updates the target and leaves the link.
- Changes: the files above and their cases.
- Defers: nothing.
