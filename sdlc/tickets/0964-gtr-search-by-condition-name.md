# 0964 — gtr-search-by-condition-name

Status: OPEN.

Milestone: 7

## Outcome

A GTR test search can match a condition by name text, with the same case, hyphen and space folding as `--name`, so "non-small cell lung cancer" finds the tests that name it only among their conditions.

## Why

0935 (PM idea, 2026-11-14): in NCBI's `test_version.gz`, 56 current tests (19 with EGFR) hold "Non-small cell" only in `condition_identifiers`, which `--name` does not read and `--condition-id` (a MedGen ID) cannot reach by text; the dx-lab-director's NSCLC slice of EGFR tests came back 0.

## Proof

- Proof: a case on the synced GTR catalog (recorded real rows) where a condition-name search finds a test whose name lacks the phrase; the persona's NSCLC + EGFR query returns the tests 0935 counted.

## Starts from

- Starts from: 0935's ticket and its folding; the GTR catalog sync and search code (`git log origin/main --oneline -- '*gtr*'`).

## Keeps

- Keeps: `--name` and `--condition-id` behavior; reply size caps.

## Defers

- Defers: nothing.

## Changes

- Changes: GTR search filter, catalog columns if needed, cases, help.

## Progress

- Measured (2026-11-15): NCBI's `test_version.gz` is still the 2026-11-05 file (live Last-Modified), the same file 0935 recorded. Its `condition_identifiers` cell holds the test's condition names, pipe-separated (for example GTR000500500.2 "EGFR-related lung cancer|Non-small cell lung carcinoma"), not identifiers. Under 0935's folding, "non-small cell" matches 56 current tests, 19 of them EGFR tests, and none of the 56 has the phrase in `lab_test_name`, which confirms 0935's count. "non-small cell lung cancer" matches 12 of the 19 EGFR tests; the rest say "carcinoma".
- Changed: new optional request field `condition` (`--condition`, GTR only), matched against `condition_identifiers` on the version rows already synced, with the same rule as `--name` (letter case and name hyphens ignored; spaces and hyphens in the text optional). No catalog column change. It combines with `--gene`, `--gene-id`, `--condition-id`, `--type` and `--name` as AND; alone it pages the whole version report. Other sources refuse it, and text without a letter or digit is refused. A refused version row is disclosed only when its condition cell is unreadable or matches. A GTR search with `--condition` carries a request-guidance notice saying how it matched and that NSCLC is not expanded. `--name` and `--condition-id` behave as before.
- Proof: `tests/cases/diagnostic/gtr-condition-name.yaml` replays 0935's recorded EGFR slice (`tests/replies/diagnostic/gtr-egfr/`): `--gene EGFR --condition "non-small cell"` returns 19, first GTR000500500.2 "EGFR targeted variants" and GTR000511179.13 "Clinical Exome", names without the phrase (CLI and MCP); "Non-small cell lung cancer" returns 12; `--condition "nonsmall cell"` alone returns 19. Schema contract test adds the refusals.
- Job j41225 measured the case at sync 197, 5,357, 5,476 and 4,099 bytes; caps 227, 6,161, 6,298 and 4,714 (measured plus 15%). `make schema-generate docs-generate` left the tree clean and `make check-cheap` passed.
- Open (PM): the typed MCP tool `search_diagnostic` measures 2,432 characters with the new field, over the 2,400 budget (`mcp-parity/compact-typed-schemas` and `mcp-parity/typed-hosted-catalog` fail). The field adds about 67 characters even with its one-line description "Condition name text."; per the brief no other field's wording was trimmed. Raising the budget or a search_diagnostic exception is the PM's call.
