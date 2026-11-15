# 0949 — gtr-gene-with-condition-refusal-names-the-combination

Status: COMPLETE.
Landed: 4e439b69d9bac25cc88bf091e96190bad9d59427

Milestone: 7

## Outcome

When `search diagnostic --source gtr --gene X --condition-id Y` is refused, the message says which combination isn't supported (or the combination works), not that the gene is invalid.

## Why

dx-lab-director round 2 call 14: the combination was refused as an invalid gene. Evidence: bench branch `ticket/0048-dx-r2-evals`, `personas/dx-lab-director/runs/2026-11-14-r2/` (pm job j40752, BioMCP 2d08cab05).

## Proof

- Proof: a case with that combination shows the accurate message or the joined result.

## Starts from

- Starts from: GTR search validation in `crates/biomcp-types/src/query/diagnostic/search_validation.rs` and the GTR adapter.

## Keeps

- Keeps: refusal shapes.

## Defers

- Defers: nothing.

## Changes

- Changes: validation or the GTR filter, a case.

## Progress

Live check, 2026-11-14 (job j40930, GTR files from NCBI the same day).

- GTR can answer the combination. `test_condition_gene.txt` (Last-Modified 2026-11-05) has a gene row and a condition row per test, so a test that lists both can be found by joining on its accession. EGFR has 185 tests, MedGen:C0007131 (non-small cell lung cancer) has 56, and 19 list both. So the joined result is the better outcome, not a better refusal.

Build.

- Validation: `--gene` or `--gene-id` with `--condition-id` is no longer refused. `--gene` with `--gene-id` is still refused, now naming only those two fields. The `--condition-id` description says that with a gene it keeps tests that list both.
- GTR adapter: with a gene and a condition, the search reads the gene's relation rows as before, reads the condition's rows from the same snapshot, and keeps the gene rows whose test also lists the condition. Each row still shows the gene relation it matched; the total counts distinct tests. `--type` and `--name` still narrow the joined set. A refused source row on either side is disclosed, since it could hide a joined test.
- Case `diagnostic/gtr-gene-condition.yaml` replays the live files cut to the 185 EGFR gene rows and 56 C0007131 condition rows (values unchanged; versions and XML cut to those 222 tests, lab elements kept to certifications as in the EGFR replay): EGFR + NSCLC gives 19 by symbol and by GeneID, and each filter alone still gives 185 and 56.
- Case caps are the measured sizes plus 15% (5,127, 3,888, 1,267 and 1,405 bytes measured in j40949). The `--condition-id` description change regenerated the checked-in schemas and the command reference; the release schema check passes (descriptions only).
- Gate: `make --keep-going landing-gate docs-check schema-release-check` passed on the branch merged with origin/main (j40975).
