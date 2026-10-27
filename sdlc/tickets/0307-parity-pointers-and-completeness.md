# 0307 — Fill five parity projection pointers and fix six assertion completeness errors

Status: COMPLETE.
Landed: d93929c5fcc56c13e89bd028728f301243a1c8ca

Milestone: 5

## Outcome

The article indexing case gains its five missing projection pointers, covering six assertion rows, and the six assertion-completeness errors are diagnosed and fixed in the runner or the cases, so the dry inventory reports neither.

- Why: landed-main preflight j16434 over all 635 cases reported 5 projection pointer gaps and 6 completeness errors in `article/semantic-scholar/synthetic-minimal.yaml`, `author/papers-bad-id.yaml`, `author/papers-wrong-offset.yaml`, `orcid/wrong-works.yaml`, `protein/wrong-accession.yaml` and `protein/wrong-taxon.yaml`. PM ruling 2026-10-27 (m8126 ask 46).
- Starts from: the j16434 inventory archived with 0297, the landed runner and declarations, and the 0.9.3 release source.
- Rules: a case changes only when its assertion is wrong or incomplete, and the reason is recorded. No live calls.
- Proof: the dry inventory reports zero projection pointer gaps and zero completeness errors, and a reviewer checks each of the six diagnoses.
- Changes: declarations, runner checks or the six cases.
- Defers: sequence step shapes, which are 0308, and native recording gaps, which wait for the 0297 partial run.
