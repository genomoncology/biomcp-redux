# 0309 — Give each zero-coverage family a few comparable parity facts

Status: COMPLETE.
Landed: 314b4d23b4ab0551899b11fbf62e81b22a65bb85

Milestone: 5

## Outcome

Diagnostic, disease, GWAS, PGx, phenotype and drug each have at least three comparable facts executed against 0.9.3 and 1.0 from harvested cases and existing recordings, or a report naming why the family cannot be compared without a new capture.

- Why: whole pass j16465 executed 165 rows across 141 cases with 109 comparable facts and zero regressions, but diagnostic, disease, GWAS, PGx and phenotype had no executed declarations and drug had only isolation and input gaps. Under the 0308 stop rule, a family with no comparable coverage gets bounded work. PM ruling 2026-10-27 (m8126 ask 48).
- Starts from: the j16465 report and gap table, the landed runner, matcher and declarations, the 0.9.3 release source and binary, and existing recordings.
- Rules: direct facts only, chosen in the order the harvested cases list them, using the 0308 value-binding matcher. No new capture and no live call; a family that needs one is reported with the exact missing input and held. Released 0.9.3 is the baseline, and each regression becomes its own bug ticket.
- Proof: a recorded run reports at least three comparable facts per family with both verdicts, or a named hold, and a reviewer checks a sample against raw outputs.
- Changes: declarations for these families only.
- Diagnostic: the existing six-step WHO sequence runs whole to keep its catalog sync state, and only its four source-reviewed empty-result assertions at steps 4 and 5 are compared. PGx counts its first three policy byte bounds as error-output bounds only. Disease, GWAS, phenotype and drug are named holds with their exact missing native requests (PM ruling 2026-10-27, m8126 ask 49).
- Defers: wider coverage of these families and every held gap in the j16465 table.
