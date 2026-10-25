# 0168 — Retire old pharmacogenomics options and prove pharmacogenomics failure and pairing cases

Status: OPEN.

Milestone: 4

Depends on: 0166
Depends on: 0167

## Outcome

Every 0.9 pharmacogenomics command, option and section that pharmacogenomics.md drops, renames or defers returns a fixed usage error that names the supported route or the reason, and makes zero requests. Every source and section pairing outside 0153 recheck amendment 1 fails before I/O. CPIC and ClinPGx each report a failed outcome, never an empty one, on a 429, 500, timeout, broken JSON or HTTP 200 error body, and a mismatched returned identity is dropped with a problem and never enters the cache.

- Why: pharmacogenomics.md's accounting retires or defers these options, the recheck amendments fix the source and section pairings, and the slice rules require failure and identity cases for every adapter.
- Starts from: pharmacogenomics.md's 0.9 accounting, deferred selectors and failure rows of the case map; the existing retirement table and usage errors; 0165's `pharmgkb` retirement; the adapters from 0166 and 0167.
- Rules: positional search names `--gene`. `-d` and `--drug` and `--evidence` say drug-text and guideline-text search are dropped. `--pgx-testing`, `--population`, `--lookup-key` and `--drug-id` say the selector is not yet supported, except a `--drug-id` that 0163 proved on `recommendations`. `get pgx QUERY` with a bare word names `--gene` and the named sections. `interactions` names `search pgx --gene` and the gene card. `frequencies` says frequency pages are deferred because no harvested question needs them. `all`, `--full` and several sections in one request say to name one section. Get `-l` names `--section-limit`, get `--offset` names `--section-offset`, and search `-l` names `--limit`. `-j` and `--json` name `--format json`. The pairing failures are: an omitted source; `--source clinpgx` with search, the card, `recommendations`, `guidelines` or `diplotype`; `--source cpic` with `annotations`; and `get pgx --gene X --source clinpgx` with no section. No retirement rewrites the query, and no command accepts unphased variants to choose a diplotype.
- Proof: outside-in cases first. A case per retired spelling and per pairing failure asserts its message with zero provider requests. Synthetic 429, 500, timeout, broken JSON and HTTP 200 error replies for `cpic` and `clinpgx`, labeled synthetic, assert a failed outcome with source and terms; a CPIC 206 partial content reply stays a page and never a failure. A synthetic wrong returned gene or accession for each adapter asserts the drop, the problem and no cache admission. Q33 asserts a successful zero distinct from each failure, Q16 asserts a missing second gene never reads as normal, and Q31 and Q32 assert that no phase is resolved. Each case has a size ceiling. This ticket adds no filter or exact-ID get and makes no provider call; the filters and exact-ID gets it relies on have known-positive captures through one bounded keyless pm job in 0163, 0166 and 0167. No licensed content enters the repo, and replies whose terms bar storage replay from outside the repo. Then make landing-gate runs through pm job.
