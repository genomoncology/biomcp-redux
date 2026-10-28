# 0370 — Attest release artifact provenance

Status: OPEN.

Milestone: 5

Depends on: 0347, 0388

## Outcome

The release workflow creates a signed build provenance attestation for every staged release artifact with GitHub's artifact attestation action, pinned by commit. The release notes and install help tell users how to verify an artifact with `gh attestation verify`. The installer keeps its SHA-256 check.

- Why: 0347 deferred signed provenance (Ian's security screening request, m10245). Ian's ruling that 1.0 ships every known fix (2026-10-27).
- Starts from: `.github/workflows/release.yml` stage job, `release/workflow.py` and the install docs in help.
- Rules: the attestation step needs only the `id-token: write` and `attestations: write` permissions on its own job, runs on standard free runners and publishes nothing else. A rehearsal run from a dispatch attests without publishing a release.
- Proof: a hosted rehearsal run shows an attestation for each staged artifact, and `gh attestation verify` passes for one downloaded artifact.
- Changes: `release.yml`, release workflow tests and install help.
- Defers: nothing.
