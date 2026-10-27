# 0306 — Extend shared parity projections and selectors for the family batches

Status: COMPLETE.
Landed: 613b3f58c8ef906238909f4d26a005c51a364956

Milestone: 5

Depends on: 0298

## Outcome

One writer lands the small closed projection and selector extensions every family branch needs, so 0300 to 0305 build on one shared base.

- Why: seven lanes started after 0298 landed at 91e2d8a80, and each hit the same small gaps in shared projections and helpers. PM ruling 2026-10-27 (m8126 ask 43).
- Starts from: the landed 0298 projections, selectors and shared proof, and the 0.9.3 release source.
- Rules: the extensions are closed and source-grounded. They are the ERepo source enum mapping from `clingen_erepo` to `erepo`, the ClinGen provider spelling and its object provider-member projection, section state spellings only where the actual applicability matches, exact namespace prefix mapping of author IDs with numeric suffix validation, effective default-limit binding taken from types and query rather than from the expected value (adverse event 5), removal of the hardcoded six-pending-owner expectation in shared proof, and the exact ORCID origin in the shared fixture-origin hook under the released policy. No orchestrator and no clinical transforms. Source delivery JSON enablement stays with each family after its containment review.
- Proof: a fixed-shape and negative guard for each extension, plus recorded witnesses where a comparable 0.9.3 reply exists. The ClinGen empty reply, which lacks a released MIR21 confirmation, and the ERepo retained page 3, which differs from the old page 25, are recorded execution gaps held for 0297; no invented data and no live capture (PM ruling 2026-10-27, m8126 ask 44).
- Changes: shared projections, selectors and proof helpers.
- Defers: family declarations, which are 0300 to 0305.
