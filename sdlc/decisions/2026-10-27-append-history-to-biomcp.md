# 1.0 history is appended to the public BioMCP history

Date: 2026-10-27

Decided by Ian, recorded by the PM.

The public BioMCP repository keeps its existing history and release tags, and this repository's history is added on top of it instead of replacing it. One merge commit on the public main has two parents, the last public main commit and the approved 1.0 candidate, and its files equal the 1.0 candidate exactly. The 1.0 tag points at that merge commit or at a commit whose files equal the approved candidate.

- Old tags such as v0.9.3 keep working, and `git log` shows both histories.
- This repository is public, so its history needs no scrub, as `2026-10-19-public-repos-skip-history-scrubs.md` says.
- This replaces the history replacement in `2026-10-23-a-clean-repo-whose-history-replaces-biomcp.md`. Pushing the merge to the public repository still needs Ian's approval at the time.
