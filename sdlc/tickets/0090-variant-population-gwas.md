# 0090 — Show gnomAD population counts and GWAS Catalog associations for a variant

Status: COMPLETE.
Landed: ec9d6a4b57c21422d552a36696b43a614ddb63b5

Milestone: 2

Depends on: 0087

## Outcome

The `population` section reads gnomAD by the source-given GRCh38 allele and returns dataset, build, allele count, allele number and filters; `variant population ID --detail` pages ancestry groups. The `gwas` section reads GWAS Catalog v2 associations by rsID.

- Starts from: variant.md (gnomAD and GWAS rows, budgets); the rulings; 0087's identity.
- Risks the proof must cover: a null genome dataset is not zero observations; no frequency is computed that the source does not give; gnomAD is asked only with an unambiguous source-given GRCh38 allele; a GWAS zero is absence from the catalog and says nothing about benignity; v1 is never called.
- Shape, amended 2026-10-24: each gnomAD count is one `PopulationFrequency` per stratum with the fields in the plan's section 3.7: dataset, dataset version, data type, ancestry group, sex, subset, counts, hemizygote count and faf95 as the source states them.
- Proof: the BRAF control asserts exome 2 of 1,460,618 and a null genome; ancestry detail pages; rs113488022 asserts a valid GWAS zero; an unjoinable allele reports the missing identity; failure cases; MCP parity; ceilings. Then make landing-gate runs through pm job after current main.
