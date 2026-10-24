# 0093 — Count a variant's recurrence in a named cBioPortal cohort

Status: COMPLETE.
Landed: 473531a46dd1e942ae5ac785b8836e040f920c64

Milestone: 2

Depends on: 0087

## Outcome

`variant recurrence ID --source cbioportal --study ID --profile ID --sample-set ID` returns the exact allele's count, the cohort's denominator when the source gives it, the query bounds and the study's terms.

- Starts from: variant.md (cBioPortal row, budgets); the rulings.
- Risks the proof must cover: no default cohort; a gene-wide row is not an allele match; a missing denominator is shown missing; study-specific commercial restrictions appear in the reply.
- PM ruling 2026-10-24 (m4587): the result is narrowed. `--gene SYMBOL` resolves through HGNC to the Entrez ID the mutations endpoint needs; the command fetches the selected sample list's mutation rows by gene and pages, keeps rows whose source genomic allele and build equal the variant's, and reports the observed count of distinct matched samples with whether every page was read, the sample list's size as `selected_sample_count`, no denominator, and the study's terms marked unverified with the source FAQ's restrictions.
- Proof: one captured study case with numerator, denominator and terms; a missing denominator; a wrong allele row excluded; paging; failure cases; MCP parity; ceilings. Then make landing-gate runs through pm job after current main.
