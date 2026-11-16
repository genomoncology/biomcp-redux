# 0995 — trial-results-come-whole-in-one-reply

Status: OPEN.

Milestone: 7

## Outcome

An agent reading a trial's posted results gets every outcome measure in one or a few calls, not one record per page. Pages are sized by what fits the reply, not one record at a time.

## Why

Bench ticket 0052's patient-navigator round (2026-11-16, BioMCP b54b4e8; bench `personas/patient-navigator/runs/2026-11-16/analysis.md:19,31`):

- ELI-002 (NCT04853017)'s posted results came one record a page: 93 records, fetched by the agent's own paging script, so 93 BioMCP calls that the call count doesn't show.
- ClinicalTrials.gov returns all 5 outcome measures in one 65,676-byte reply (`/api/v2/studies/NCT04853017?fields=ResultsSection`).
- Code mode got the same numbers from one `curl`.

This is that round's biggest BioMCP finding, and it got worse since 2026-11-12.

## Proof

- Proof: first find, in Progress, why a results page holds one record. Is it the page size, a per-record byte cap, or the 0823/0824 section shape?
- Proof: a routine case over a recorded NCT04853017 results reply returns all 5 outcome measures in at most 2 calls. It fails on the base commit.
- Proof: replies stay within the existing caps. Any schema change follows the 1.0 rules, and the release notes say what changed.

## Starts from

- Starts from: the trial results section (0823, 0824), its continuation and size limits, the bench analysis above.

## Keeps

- Keeps: 1.0 schema rules; reply caps; whole records (no record cut mid-way).

## Defers

- Defers: distance on site rows, printable export, and telling new trials from edited ones in the watch (the same round's other gaps). Those are separate tickets once the next personas confirm them.

## Changes

- Changes: the trial results section or its paging, one case and its recording, the notes.

## Progress

- Cause, 2026-11-16: the page size. `crates/biomcp-query/src/trial/get/page/results.rs` paged posted results with an 8,000-byte record budget. A page always takes its first record whole and stops before the next one that would pass the budget. ELI-002's first three outcome measures are each about 5 KB (5,058, 4,936 and 4,838 bytes in the source), so pages 1-3 each held one outcome; the remaining 90 records came about 8 KB at a time. It was not a per-record cap and not the 0823/0824 section shape.
- The 93 calls in the bench log are the agent's reading, not BioMCP's paging: after the first page held one record, the agent wrote "one outcome record per page" and looped on the continuation. Its first loop got 26 records in 5 calls (`biomcp-cli/calls.md` turn 27), so the whole section was about a dozen pages. Still, the outcome measures, which is what the agent wanted, took three calls and the first reply looked like one record per page.
- Fix: the results page budget is now 40,000 bytes (`RESULTS_BUDGET`), which also sizes `flow` and `baseline`. Records stay whole and `--limit` still caps the count. 40,000 keeps `get trial ID all` under the 150,000-character MCP reply cap (`crates/biomcp/src/mcp/protocol.rs`): the other sections' budgets add to 91,000 bytes (locations 60,000, outcomes 8,000, publications 8,000, eligibility 6,000, arms 4,500, contacts 4,500). No schema change: same fields, only more records a page. Release notes say results pages hold up to 40,000 bytes.
- Recording: NCT04853017 recorded live in j41645 (base adf01aabb); its `resultsSection` equals the same day's direct download (5 outcome measures). Case `tests/cases/trial/results-outcomes-together.yaml`: `get trial NCT04853017 results` returns all 5 outcomes, whole, in one call. On the base code (j41651, results.rs reverted) it fails: item 1 is missing.
- `tests/cases/trial/results-flow.yaml` (NCT02987543) now finds outcomes, flow and baseline in one `--limit 20` page with continuation at 20; its ceiling is the measured 18,691 bytes plus 15 percent (j41651). The CheckMate 067 paging runner still pages its hundreds of records whole and passed unchanged.
- j41651 (136166ad4): schema and docs regenerate with no diff, `make check-cheap` passes, trial cases pass except the old results-flow expectations, fixed above.
- Gate j41654 passed on e234bf393 (branch merged with origin/main 0bffbf7ba): `make --keep-going landing-gate docs-check schema-release-check`.
- Measured, j41662 (a34e08488), through the case runner's own output sizes (the bytes the other cases' ceilings use, source-terms text excluded), all at `--limit 100`. The MCP reply cap is 150,000 characters of the JSON value (`crates/biomcp/src/mcp/protocol.rs`); characters never exceed bytes.

  | Trial (recorded results size) | `all` plain JSON | `all` MCP single tool | `all` typed `trial` tool | `results flow baseline` JSON / typed |
  | --- | --- | --- | --- | --- |
  | NCT01844505 (301 KB results, 150 sites) | 69,384 | 69,384 | 69,533 | 41,212 / 41,425 |
  | NCT02578680 (217 KB, 0 sites) | 61,564 | 61,564 | 61,637 | 52,624 / 52,837 |
  | NCT02296125 (202 KB, 170 sites) | 76,885 | 76,885 | 77,072 | 54,713 / 54,926 |
  | NCT03734029 (92 KB, 208 sites) | 78,424 | 78,424 | 78,573 | 50,445 / 50,658 |

  The largest is 78,573, about half the cap, so 40,000 stays. `results-outcomes-together` measured 23,965 bytes in CLI and MCP; its ceiling is now 27,560 (plus 15 percent).
