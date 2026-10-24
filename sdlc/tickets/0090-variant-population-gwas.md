# 0090 — Show gnomAD population counts and GWAS Catalog associations for a variant

Status: OPEN.

Milestone: 2

Depends on: 0087

## Outcome

The `population` section reads gnomAD by the source-given GRCh38 allele and returns dataset, build, allele count, allele number and filters; `variant population ID --detail` pages ancestry groups. The `gwas` section reads GWAS Catalog v2 associations by rsID.

- Starts from: variant.md (gnomAD and GWAS rows, budgets); the rulings; 0087's identity.
- Risks the proof must cover: a null genome dataset is not zero observations; no frequency is computed that the source does not give; gnomAD is asked only with an unambiguous source-given GRCh38 allele; a GWAS zero is absence from the catalog and says nothing about benignity; v1 is never called.
- Proof: the BRAF control asserts exome 2 of 1,460,618 and a null genome; ancestry detail pages; rs113488022 asserts a valid GWAS zero; an unjoinable allele reports the missing identity; failure cases; MCP parity; ceilings. Then make landing-gate runs through pm job after current main.
