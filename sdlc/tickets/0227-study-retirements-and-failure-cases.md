# 0227 — Retire old study commands and chart options and prove study failure cases

Status: COMPLETE.
Landed: a3e010de543b628c3da438beee1fb601f770dd4e

Milestone: 4

Depends on: 0213
Depends on: 0238
Depends on: 0221

## Outcome

Every 0.9 study leaf, option, alias and chart option in study.md's accounting returns a fixed usage error that names the supported route or the ruling, and makes zero requests. The catalog sync, the card and the `profiles`, `sample-lists` and `files` sections each report a failed outcome, never an empty one, on a 429, 500, timeout, broken JSON or HTTP 200 error object, and a mismatched returned identity is dropped with a problem and never enters the cache.

- Why: the entity boundary ruling and the 2026-10-26 study rulings retire 0.9's calculation leaves and chart options with fixed messages, and the slice rules require failure and identity cases for every adapter.
- Starts from: study.md's 0.9 accounting, legacy output sections and failure rows of the case map; the existing retirement table in `crates/biomcp/src/retired.rs` and its usage errors; the adapters from 0213, 0238 and 0221.
- Rules: `study list` names `search study`. `study download` and its `--list` name `sync cbioportal-studies` for metadata and `get study ID files` for source file references, and say whole-study analytical installs are retired. `study query`, `study top-mutated`, `study filter`, `study cohort`, `study survival`, `study compare` and `study co-occurrence` each say the local calculation is dropped by the entity boundary ruling and name `get study ID` and the source study page, with every command-specific option and alias study.md lists, including the `--type`, `--endpoint` and `--target` values. The shared chart options `--chart`, `--terminal`, `--output` and `-o`, `--title`, `--theme`, `--palette`, `--cols`, `--rows`, `--width`, `--height`, `--scale` and the hidden `--mcp-inline` return one fixed message that charts are retired. A set `BIOMCP_STUDY_DIR` never selects a directory, and the reply names `sync cbioportal-studies` instead of scanning it. The old `not_in_local_cohorts` state becomes a missing snapshot or a source absence, never a zero frequency. No retirement rewrites the query, calls an analysis route or installs a tool.
- Proof: outside-in cases first. A case per retired leaf, option, alias, enumerated value and chart option asserts its message with zero provider requests, and a case asserts the `BIOMCP_STUDY_DIR` guidance with no directory read. Synthetic 429, 500, timeout, broken JSON and HTTP 200 error replies for the catalog sync, `profiles`, `sample-lists` and `files`, labeled synthetic, assert a failed outcome with source and terms, and a failed sync keeps the last accepted copy. A synthetic wrong study identity in the bundle, a profile and a sample list asserts the drop, the problem and no cache admission. A successful empty search, a missing bundle, a stale bundle, an absent study and a disabled source each assert their own distinct status. Each case has a size ceiling. This ticket adds no filter or exact-ID get and makes no provider call; the filters and exact-ID gets it relies on have known-positive captures through one bounded keyless pm job in 0207, 0213, 0238 and 0221. No licensed content enters the repo, and replies whose terms bar storage replay from outside the repo. Then make landing-gate runs through pm job.
- Defers: the `counts` section's failure and identity cases stay in 0223, so this ticket lands whether or not the gene-count section is built.
