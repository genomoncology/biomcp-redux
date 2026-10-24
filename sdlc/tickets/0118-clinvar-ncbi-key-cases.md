# 0118 — Prove ClinVar's optional NCBI key with and without a key

Status: COMPLETE.
Landed: 1d068e8f2c976a2d53758d54b4e24996a21aee72

Milestone: 2

## Outcome

Cases prove that ClinVar requests run at the public limit without `ncbi_api_key`, and send the key and use the higher limit with it, read from the config file or the environment, with the key never recorded or printed. This ticks the roadmap's ClinVar NCBI key box.

- Why: the ClinVar adapter declares the optional NCBI key, but no case proves the keyed request. The Semantic Scholar key tests in `crates/biomcp/tests/runner/scholar.rs` are the pattern.
- Proof: stub-backed cases for keyless, config-key and environment-key runs assert the same reply, the key reaching only the stub, no key in any recording, and each rate interval. No provider call. Then make landing-gate runs through pm job.
