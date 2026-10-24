# 0091 — Show CIViC evidence and Cancer Hotspots counts for a variant

Status: COMPLETE.
Landed: 7669361438bcb8772d2cd5337fa46d6cf6106024

Milestone: 2

Depends on: 0087

## Outcome

The `civic` section returns CIViC evidence and assertions for the variant's exact molecular profiles, keeping compound profiles labeled. The `hotspots` section returns Cancer Hotspots residue and alternate amino-acid counts with the source transcript.

- Starts from: variant.md (CIViC and Cancer Hotspots rows, budgets); the rulings; the gene slice's CIViC adapter.
- Risks the proof must cover: a CIViC name match is a candidate until exact variant membership is joined; a compound OR profile is shown as compound; residue counts never read as allele frequency or actionability.
- Proof: the shared fixture's CIViC Statement row runs through BioMCP's `Statement`; BRAF V600E asserts its exact profile evidence apart from the compound profile; a hotspots case with residue and alternate counts and transcript; failure cases; MCP parity; ceilings. Cancer Hotspots captures run from outside the repo until its terms are checked. Then make landing-gate runs through pm job after current main.
