# 0395 — Ship the license, third-party notices and registry marker

Status: COMPLETE.
Landed: e3cb10cd7543f95e5743140af771c9b43f75c0e9

Milestone: 5

## Outcome

The repository root has the MIT LICENSE text that public 0.9.3 ships, so the public merge keeps it. Every artifact carries LICENSE and a generated third-party notices file: wheels, archives, desktop bundles, the container image and the plugin zip. The wheel METADATA has a description containing `mcp-name: io.github.genomoncology/biomcp`, plus `License` and `Project-URL`, as the 0.9.3 wheel did. Artifact inspection and staging fail when any of these is missing.

- Why: pre-release review `sdlc/records/2026-10-28-pre-release-review.md`, release findings 2 and 5, confirmed against the published 0.9.3 wheel and repository. The MCP registry verifies PyPI ownership through that marker, and PyPI metadata cannot change after upload. MIT, Apache, BSD and ISC dependencies require their notices in binary copies.
- Starts from: `release/artifacts.py`, the staging code, `release/Dockerfile`, `deny.toml` and the 0.9.3 wheel METADATA.
- Rules: notices are generated from the locked dependency set by a pinned tool, not written by hand. No licensed data in fixtures.
- Proof: `inspect()` and staging cases fail on an artifact without LICENSE, notices or the marker; the 0336 hosted run passes with them present.
- Changes: LICENSE, the notices generator, packaging and their tests.
- Defers: nothing.
