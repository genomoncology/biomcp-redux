# 0128 — Search CIViC therapies and add the drug civic section

Status: OPEN.

Milestone: 3

Depends on: 0124
Depends on: 0126
Depends on: 0125

## Outcome

`search drug TEXT --source civic` returns CIViC therapy candidates with their native IDs and totals, and `get drug 5 civic --source civic` returns that therapy's evidence and assertions with molecular-profile, disease and combination context, each collection paged on its own with `--section-limit` and `--section-offset`. Both reuse the existing CIViC adapter and molecular-profile types.

- Why: the bank's biomarker-linked therapy questions (D01, D03, D05, D17) need CIViC evidence, and drug.md keeps it. The 0116 recheck marks the exact therapy-ID get unproven.
- Starts from: `crates/biomcp-adapters/src/civic`, the gene and variant CIViC sections and their types; drug.md's CIViC rows and case C01; 0124 and 0125.
- Rules: Trastuzumab Deruxtecan 4287 and the Neratinib 52 plus Trastuzumab 84 combination stay candidates, never merged with Trastuzumab 84. No drug identity join to MyChem, ChEMBL or Open Targets. A therapy name match is not drug equivalence. `identity_level` comes from the endpoint or record type, never from a field's shape.
- Proof: outside-in cases first. One bounded keyless pm job captures a positive exact therapy-ID request for Imatinib 5, Trastuzumab 84 and Ivosidenib 454 before section code, and a known positive for the search and each collection's paging. Cases S03 for `civic` (D01, D03, D17, and D19's actual zero) and C01 (native therapy, evidence and profile IDs, separate evidence and assertion totals and continuations) replay the recorded replies with size ceilings. Existing CIViC gene and variant cases still pass. No licensed content enters the repo, and replies whose terms bar storage replay from outside the repo. Then make landing-gate runs through pm job.
