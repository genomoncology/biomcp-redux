# 0392 — Prove AlphaGenome's local TLS path from the installed binary

Status: OPEN.

Milestone: 5

## Outcome

The release smoke and container checks run the existing `variant/alphagenome-native-groups-local-tls` trial against the installed candidate binary without a source checkout, and each report has an explicit entry for it. The runner gains an installed AlphaGenome proof mode beside the existing `--installed-http-proof`, using the same local TLS fixture.

- Why: the coordinator found (m12221, 2026-10-28) that `release/smoke.py` passes `--cases` with copied fixtures, which leaves out the protocol trials, and that dropping `--cases` needs a source checkout. The installed release channel has no AlphaGenome gRPC proof, which 0198 requires for 1.0.
- Starts from: `--installed-http-proof` in `cases.rs` and `runner/mod.rs`, `release/smoke.py`, `release/container_check.py` and the AlphaGenome local TLS trial.
- Rules: reuse the existing trial and fixture; no provider request, capture or key; no product behavior change; no new framework.
- Proof: the installed mode passes against a built binary in an isolated directory with no checkout, and fails when the binary's AlphaGenome transport is broken; the smoke and container reports list the entry. The final 0293 rehearsal runs it on the five native targets and both Linux containers.
- Changes: the runner entry point, the two release scripts, the report entry and their cases.
- Defers: nothing.
