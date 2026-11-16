# 0982 — saved-search-snapshots-and-changed

Status: COMPLETE.
Landed: 264f3455ff81703d56cc8574d57a39ba256e0291

Milestone: 9

## Outcome

A user runs `saved changed [NAME]` and learns which IDs a saved search gained, lost or changed since its last check, with the states `first`, `unchanged`, `changed`, `not_due`, `rebaselined` and `incomplete` that `sdlc/planning/saved-work.md` (Change detection, Records: `biomcp.saved-snapshot/1`, `biomcp.change-report/1`) defines.

## Why

Step 3 of the saved-work design's Order; 0831 built step 1 (landed 2026-11-15).

## Proof

- Proof: two recorded days of one trial search with hand-counted adds, removals and updates; the report equals the hand count.
- Proof: the same replay under two row schemas reports `rebaselined`, never every row as changed.
- Proof: inside the once-an-hour floor `saved changed` answers `not_due` and runs nothing; a re-check reads the source, not the reply cache, through its own reviewed path (hosted refuses `--fresh`).

## Starts from

- Starts from: 0831's store (`crates/biomcp-query/src/store/saved/`), its migration rule and compatibility key; the design's Change detection section.

## Keeps

- Keeps: 0831's compatibility rules (additive migrations only); two newest snapshots per search; no source content stored, only IDs and fingerprints.

## Defers

- Defers: nothing.

## Changes

- Changes: the saved-work code and its cases.

## Progress

- Decided (within the outcome): `saved changed [NAME]` replies with the one saved-work body, `biomcp.saved/1`, which gains two optional lists: `changes` (one `biomcp.change-report/1` per checked search) and `snapshots` (only `saved export` fills it, so `saved show` stays short). Both record formats are registered schema contracts. Without NAME every saved search is checked; a list's name is unknown to `changed`.
- Store: three additive migrations (`saved_snapshot`, its index, `saved_snapshot_row`), so `COMPATIBILITY` stays 1 and `migrations_only_add` passes. A row holds entity, ID, source and fingerprint only. Each check keeps its snapshot, drops all but the two newest, and sets `checked`, in one transaction; `saved delete` removes a search's snapshots.
- Re-check path: `crates/biomcp/src/command/recheck.rs` re-parses the stored words through the grammar (a retired or unknown filter answers its usage error and runs nothing), inherits the caller's process options, and runs every page with `fresh` set, so the reply cache is never read. It follows the reply's own page step until the source has no more pages. Only the command line reaches it; MCP and hosted are unchanged, and hosted still refuses `--fresh`. `saved changed --offline` is refused when a search is due, because offline reads only the cache; a not-due answer needs no source and works offline.
- Floor: `checked` plus 3,600 seconds. Inside it the search answers `not_due` with `next_due` and the newest snapshot, and runs nothing (the runner trial points it at an empty replay folder).
- Cap: a snapshot holds up to 500 IDs (`SNAPSHOT_ROWS`). A search whose first page states more, a walk that hits the cap, or a row with no exact get target gives `complete: false`, and two such snapshots report `incomplete` with both totals. The cap is a constant, not a setting, because no caller asked to change it.
- IDs and fingerprints: a row's ID is the exact get command the research projection already derives (`mcp/research/records.rs`, now `pub(crate)`), split into entity, ID and `--source`. The fingerprint is SHA-256 of the row's JSON. `row_schema` is the reply schema ID plus the first 8 bytes of SHA-256 of that schema's checked-in definition, so any row format change, even an additive one that keeps the schema ID, gives `rebaselined` instead of every row as changed.
- Order of states: `first` (no earlier snapshot), then `rebaselined` (row schemas differ), then `incomplete`, then `changed` or `unchanged`. A report's `search` is the saved search's name; `from` and `to` carry the snapshot ID, time and total.
- A check that reads a failed or partial page keeps nothing, leaves `checked` alone, and answers that page's reply.
- Live (job j41517): five trial searches were saved and checked against ClinicalTrials.gov; each walked its pages (11 to 41 IDs, up to 5 pages), answered `first`, then `not_due`, then after moving `checked` back an hour `unchanged`; every snapshot was complete.
- Proof data: `tests/replies/saved/changed/day1` is the real recording of `search trial --condition "uveal melanoma" --phase 3` (22 trials, three pages). The second run in the same job got the same studies, so `day2` is an authored variant of it with hand-counted changes (1 new, 2 gone, 2 changed), described in its README and `hand-count.json`. The runner trial `saved-work/changed-two-recorded-days` checks the report equals the hand count, `not_due` reads nothing, a snapshot under another row schema gives `rebaselined` with no rows listed and the next check `unchanged`, two snapshots are kept, and a stored search that no longer parses answers a usage error. `tests/cases/saved/changed.yaml` covers first, not_due, export and an unknown name.
- Fixture: `tests/fixtures/saved/1.0.0/` was rewritten by this build (job j41517) and now holds a snapshot (melanoma, 3,780 matches, so incomplete with its first page). New page-token lines and the fixture's request key have reviewed bindings in `release/secrets.ignore.jsonl`.
- Checks: job j41519 passed fmt, schema and docs regeneration (no change), the store tests, `saved-work/` (6), the saved, help and mcp case folders, and `make check-cheap`. Gate j41523 passed on 8a3587ea8 (merged with origin/main ec489dc1d): `make --keep-going landing-gate docs-check schema-release-check`.
- Review fixes (2026-11-16): `saved changed` without NAME first sorts searches into not due and due, refuses `--offline` before anything runs, and then checks each due search; one whose check fails (source failure, or words that no longer parse) reports the new state `failed` (`next_due` now, nothing kept) and the loop goes on, so the others' reports are shown. With NAME the failing reply is still the answer. A rebaseline now lists `new` and `gone` by ID and only leaves out `changed`. `row_schema`'s digest covers only the row definition and the definitions it reaches, with descriptions, titles and examples removed, so wording and other reply parts no longer rebaseline. 0831's original file and export are kept byte for byte in `tests/fixtures/saved/1.0.0-0831/` (from 054509929), so the upgrade from 0831's file to this build's is tested beside this build's own `1.0.0` file. The two-day trial also checks the day-two total (21), checks a rebaseline back to day one lists the hand count's new and gone the other way round, and saves a second search whose words no longer parse: checked with the others it reports `failed` while `watched` reports `changed`.
- Checks: job j41529 passed fmt, schema and docs regeneration (schemas regenerated for the `failed` state), the store tests, the schema tests, `saved-work/` (6) and `make check-cheap`. Gate j41531 passed on 30cb548f1 (merged with origin/main c769752f4): `make --keep-going landing-gate docs-check schema-release-check`.
