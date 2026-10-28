# 0364 — List candidate alleles when variant population gets an rsID or ClinVar ID

Status: COMPLETE.
Landed: b359506517c52a505fa38bbb4dac590455ef5d87

Milestone: 5

Depends on: 0387

## Outcome

`variant population rs7903146` and `variant population clinvar:7413` return the candidate alleles the source gives for that ID, with a next step for each, instead of `failed` with an empty `next` and no reason. An ID with one allele returns its population data.

- Why: the docs lead's second-pass product findings (2026-10-27), item G4. Ian's ruling that 1.0 ships every known fix (2026-10-27).
- Starts from: the variant population command, the MyVariant lookup and its next step builders.
- Rules: candidates come from the source reply only; BioMCP does not pick an allele.
- Proof: replay cases for rs7903146 and clinvar:7413 show candidates and next steps; a single-allele ID returns data.
- Changes: the variant population query, next steps and cases.
- Defers: nothing.
