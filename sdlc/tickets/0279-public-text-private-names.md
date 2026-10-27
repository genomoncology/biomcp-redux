# 0279 — Remove private infrastructure names from public repo text

Status: COMPLETE.
Landed: ac3fb6647ed401749b460b72872b3e1802b4bf07

Milestone: 5

## Outcome

README.md, the runner README, release scripts, tracked scripts and docs outside `sdlc/` name no private project, team role, machine or workspace path, so the repository history can replace the public BioMCP repository. Capture scripts take their host and evidence path from arguments or environment instead of hard-coding them, or leave the repository when they served one ticket.

- Why: the milestone 4 review found `pm job`, Jarvis, the workspace machine registry, `ssh yellow` and `/home/ian/workspace/data` paths in tracked public files, , while the workspace rules say public repos name no private project.
- Starts from: README.md lines 18, 26 and 50, `crates/biomcp/tests/runner/README.md` lines 73 to 75, `release/matrix.sh`, the tracked scripts that hard-code a host, workspace path, machines.json or ticket number (about 20, including `scripts/nightly_live_check.py` lines 270 to 290, `capture_author_witnesses.py`, `gwas_get_capture.sh`, `study_files_proof.sh`, `protein_*_native.sh` and the `author_*_plan.json` files), `sdlc/planning/release.md`, architecture.md's MCP compatibility text and `2026-10-19-public-repos-skip-history-scrubs.md`.
- Rules: public products and channels may be named. `sdlc/` records may keep internal names as the workspace decision allows.
- Proof: a reviewer reads every changed file and the list of tracked files outside `sdlc/` for private names, machines and workspace paths, and capture and release scripts still run with explicit arguments in their existing tests. No name check, linter or guard is added, as Ian's 2026-10-19 decision says.
- Changes: the named docs and scripts, and the docs check.
- Defers: nothing.
- PM ruling 2026-10-26 (m8126 ask 15): the repo's own AGENTS.md is this repository's file, so the developer rewrites it as self-contained public instructions with no private skill paths or notes. Distribution platform names such as PyPI, Homebrew, Docker, GHCR, Windows and the MCP registry are product facts and stay; the workspace rule on vendor names concerns agent harnesses and models. Per Jarvis (m8340) and Ian's 2026-10-13 ruling, public products, including AI vendors' products and the plugin marketplace, may be named plainly; only private projects, customers, internal infrastructure and tokens are removed. Release scripts drop the private job labels and accept generic extra container arguments, and the private job invocation supplies the labels.
