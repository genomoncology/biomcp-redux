# 0302 — Declare Variant parity plans for 0.9.3

Status: COMPLETE.
Landed: 9071351337ed999c3f230d4fab76577f86cc71dd

Milestone: 5

Depends on: 0298

## Outcome

Every direct variant case in the 0298 inventory has a reviewed 0.9.3 command, pointer and query declaration, and the dry inventory shows no pending gap for this family.

- Why: the 0298 source analysis found 13 direct variant cases that each need reviewed declarations, and one writer serializing all families leaves machines idle. PM ruling 2026-10-27 (m8126 ask 40) splits each family into its own ticket for parallel writers, landed together in one accepted batch.
- Starts from: the landed 0298 endpoint catalog, inventory, projection guards and witnesses, the read-only analysts' source-grounded drafts, the 0.9.3 release source and binary, and the harvested cases under `tests/cases/`.
- Rules: released 0.9.3 is the baseline. Declarations use only endpoints the 0.9.3 release calls and the 0298 strict query selectors. A question 0.9.3 cannot answer is reported with its reason. No live calls and no new orchestrator.
- Proof: the dry inventory reports zero pending variant gaps and no unsafe plan, a recorded run over a sample of the family produces both verdicts, and a reviewer checks a sample of declarations against 0.9.3 source.
- Changes: variant declarations only.
- Defers: the whole run, which is 0297.
