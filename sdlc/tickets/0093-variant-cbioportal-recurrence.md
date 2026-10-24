# 0093 — Count a variant's recurrence in a named cBioPortal cohort

Status: OPEN.

Milestone: 2

Depends on: 0087

## Outcome

`variant recurrence ID --source cbioportal --study ID --profile ID --sample-set ID` returns the exact allele's count, the cohort's denominator when the source gives it, the query bounds and the study's terms.

- Starts from: variant.md (cBioPortal row, budgets); the rulings.
- Risks the proof must cover: no default cohort; a gene-wide row is not an allele match; a missing denominator is shown missing; study-specific commercial restrictions appear in the reply.
- Proof: one captured study case with numerator, denominator and terms; a missing denominator; a wrong allele row excluded; paging; failure cases; MCP parity; ceilings. Then make landing-gate runs through pm job after current main.
