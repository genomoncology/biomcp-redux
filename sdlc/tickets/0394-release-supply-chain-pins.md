# 0394 — Pin release images and bootstrap tools

Status: COMPLETE.
Landed: 195795eb66a14200978832dfc932ec361b20d1ee

Milestone: 5

## Outcome

The manylinux build images and the runtime base image are pinned by digest in `release/toolchain.json`, and `toolchain.py check` rejects a floating image reference. The runtime image installs pinned package versions. The rustup-init checksum is pinned in the manifest instead of fetched from the same server. Tag builds restore no build cache. actionlint checks the workflows in the landing gate.

- Why: pre-release review `sdlc/records/2026-10-28-pre-release-review.md`, release findings 6 and 12; the release rules require one pinned manifest.
- Starts from: `release/toolchain.json`, `release/toolchain.py`, `release/workflow.py`, `release/rehearse.py`, `release/matrix.sh`, `release/Dockerfile` and `.github/workflows/release.yml`.
- Rules: reuse 0391's manifest and check; no new framework.
- Proof: the check fails on a tag-only image reference and on a rustup digest mismatch; actionlint passes in the gate; the 0336 hosted run passes.
- Changes: the files above and their tests.
- Defers: nothing.
