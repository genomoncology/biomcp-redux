# 0289 — Carry the 0.9 release pipeline over to 1.0

Status: COMPLETE.
Landed: 37e79ee6e734acda7ce4ad25dcb92a7140cd9bc8

Milestone: 5

## Outcome

One GitHub Actions release workflow, triggered only by a release tag, builds every artifact in `sdlc/planning/release.md` through the existing `release/` producers, runs the install gate on each platform the release promises, and stages the results for the release operators. It publishes nothing to PyPI, GHCR, Homebrew, the MCP registry, the desktop extension or the plugin marketplace until Ian approves the release, and that approval step is explicit in the workflow.

- Why: ROADMAP.md's release pipeline box is open, and 1.0 must keep every 0.9 distribution name.
- Starts from: 0.9.3's `.github/workflows/release.yml`, `install.sh`, `Formula/biomcp.rb`, `Dockerfile`, `manifest.json` and `server.json`; this repo's `release/` producers and `make release-artifacts`; `sdlc/planning/release.md`; and `skills/gh-actions.md`, which limits Actions to public releases.
- Rules: everyday builds stay on pm job. The workflow pins actions by commit, holds no long-lived secret beyond what each publisher requires, and every publish job waits on a protected environment that only the release operator approves after Ian's approval.
- Proof: local producer, installed and staged plugin checks pass with no Actions run, tag, dispatch or publication, and a reviewer checks each channel against release.md. The workflow run that builds every artifact on all five platforms and both containers with publish jobs skipped moves to 0293 (PM ruling 2026-10-26, m8126 ask 36).
- Changes: `.github/workflows/release.yml` and any producer gaps it exposes.
- Defers: publication itself, which needs Ian.
