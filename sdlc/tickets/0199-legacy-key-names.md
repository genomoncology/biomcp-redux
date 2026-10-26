# 0199 — Restore the 0.9 key environment names

Status: COMPLETE.
Landed: 1f05b8ca57754fe48646b9edf10c3b77ccdb6209

Milestone: 1

Depends on: none

## Outcome

BioMCP reads the Semantic Scholar key from `S2_API_KEY` and the OncoKB key from `ONCOKB_TOKEN`, per the 2026-10-25 key names decision. `SEMANTIC_SCHOLAR_API_KEY` and `ONCOKB_API_KEY` are removed everywhere, with no fallback. Config field names stay as they are.

- Starts from: `crates/biomcp-fetch/src/secret.rs`, whose `environment()` returns the renamed variables.
- Keeps: config-file precedence, credential containment, key reload and registration behavior, and the rule that a key never appears in a request key, recording, trace, error or envelope.
- Changes: `crates/biomcp-fetch/src/**`, `crates/biomcp/src/**` help text for `help key`, `crates/biomcp/tests/runner/key_*.rs`, `crates/biomcp/tests/runner/scholar.rs`, `crates/biomcp/tests/runner/credential_containment.rs`, help cases under `tests/cases/**`, `sdlc/planning/upgrade.md`, `sdlc/planning/architecture.md`, `sdlc/planning/slices/*.md`.
- Proof: runner tests with placeholder keys show that `S2_API_KEY` and `ONCOKB_TOKEN` send their keys and that the config file wins over each. A repository search finds no `SEMANTIC_SCHOLAR_API_KEY` or `ONCOKB_API_KEY` in code, tests or plans. `help key semantic-scholar` and `help key oncokb` name the 0.9 variables within measured ceilings. Offline landing gate passes.
- Defers: keys for sources not yet built; each keeps its 0.9 name when its slice lands.

After this lands, 0061 runs with the inherited `S2_API_KEY` directly and drops its name mapping.
