# 0199 — Read the 0.9 key environment names as fallbacks

Status: OPEN.

Milestone: 1

Depends on: none

## Outcome

BioMCP reads `S2_API_KEY` when `SEMANTIC_SCHOLAR_API_KEY` is unset and `ONCOKB_TOKEN` when `ONCOKB_API_KEY` is unset, per the 2026-10-25 legacy key names decision. Users who upgrade from 0.9 keep their settings.

- Starts from: `crates/biomcp-fetch/src/secret.rs`, whose `environment()` returns one name per key.
- Keeps: config-file precedence, credential containment, key reload and registration behavior, and the rule that a key never appears in a request key, recording, trace, error or envelope.
- Changes: `crates/biomcp-fetch/src/**`, `crates/biomcp/src/**` help text for `help key`, `crates/biomcp/tests/runner/key_*.rs`, `crates/biomcp/tests/runner/scholar.rs`, `crates/biomcp/tests/runner/credential_containment.rs`, help cases under `tests/cases/**`, `sdlc/planning/upgrade.md`, `sdlc/planning/architecture.md`, `sdlc/planning/slices/article.md`.
- Proof: runner tests with placeholder keys show, for Semantic Scholar and OncoKB, that the 0.9 name alone sends the key, that the 1.0 name wins when both are set, and that the config file wins over both. The containment test covers a key supplied by the 0.9 name. `help key semantic-scholar` and `help key oncokb` name both variables within measured ceilings. Offline landing gate passes.
- Defers: keys for sources not yet built; each of those follows the decision when its slice lands.

After this lands, 0061 no longer needs its name mapping in the job command, and either form works.
