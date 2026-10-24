# 0114 — Preserve the native ClinVar drug-response disease relationship

Status: OPEN.

Milestone: 2

Changes: crates/biomcp/src/source/clinvar, crates/biomcp-types, cases/variant

## Outcome

- A ClinVar drug-response assertion keeps its native disease relationship type and disease label separately from its drug-response label. Preserve source context without inferring sensitivity, resistance, treatment advice, or a new classification.
- The captured SCV000965609.1 contains a DrugResponseAndDisease relationship whose preferred name is Pancreatic Adenocarcinoma. Current output exposes Dabrafenib response in detail.conditions and drops the relationship. The approved variant slice requires the source condition and separate clinical context.
- Fix the owning assertion mapper and typed contract. Preserve independent submission domains, observation origins, versions, and statement_unmapped behavior. Apply the existing schema major rules if the contract changes.
- Proof: an outside-in synthetic ClinVar XML case reproduces the omission and verifies the native relationship type and label through CLI and MCP output. Cover a submission without the relationship and keep existing source facts unchanged. Run the relevant slice checks and routine gate through pm job. No provider calls are required.

## Witness

- Clinical review of the bounded 0110 capture confirmed the defect. The capture is kept outside the repo with the 0110 evidence. The recorded XML body SHA256 is 1b82ed1a73d1eddd3c2e796e488ba3e5008508adef2ce445dfc3a1ab816339ba. Keep the raw source body outside Git; write a synthetic case.
- Native path: ClinicalAssertion[@ID='1893254']/TraitSet[@Type='DrugResponse']/Trait/TraitRelationship[@Type='DrugResponseAndDisease']/Name/ElementValue[@Type='Preferred'].
- Contract: sdlc/planning/slices/variant.md requires source condition and separate clinical context. This bug does not invalidate the narrower real drug-response label proof.
