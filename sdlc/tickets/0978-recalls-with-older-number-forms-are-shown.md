# 0978 — recalls-with-older-number-forms-are-shown

Status: OPEN.

Milestone: 7

## Outcome

Drug and device recalls whose openFDA recall number uses an older published form (such as device `Z-NNNN-YY` from 2000–2006 or drug `D-NNN-YYYY`) are shown as rows and can be looked up by that number, instead of being declined.

## Why

0977's live scan of openFDA's bulk exports (2026-11-15, pm job j41437, no key): 5,539 of 59,705 device recall records, 11 of 40,481 device enforcement records and 2,596 of 18,110 drug enforcement records carry a recall number BioMCP's `biodata` types reject. Most are real recalls in an older form, not blanks. After 0975 and 0977 they are named as declined records but still cannot be shown or looked up, so about 8,000 real recalls are out of reach.

## Proof

- Proof: first list the distinct number forms in j41437's scan and decide which are published forms (older formats) and which are typos (`Z1006-2010`) or cut-short values (`Z-3011-201`); only published forms are admitted.
- Proof: routine cases with recorded pages from 0977 (`drug/recall-declined-records`, `diagnostic/recall-declined-numbers`) return the older-form records as rows, and `get` by an older-form number returns its record; both fail on the base commit.

## Starts from

- Starts from: `biodata::DeviceRecallNumber` and the drug recall number type in the pinned `biodata` crate (`Cargo.toml:40`, rev 8bcd245), their schema patterns (`drug-recall-header.json` requires `^D-[0-9]{4}-[0-9]{4}$`), and 0977's declined-record path.

## Keeps

- Keeps: 1.0.0 schema rules. Widening a value pattern needs a recorded entry in `release/schema-reviews.json` (`crates/biomcp-types/tests/schemas/compatibility.rs:102-116`); the developer writes the entry with the evidence and the reviewer checks it. Typos and blanks stay declined with 0977's problem.

## Defers

- Defers: nothing.

## Changes

- Changes: `biodata` recall number types (a new pinned rev), the recall adapters, `release/schema-reviews.json`, cases.

## Progress

- 2026-11-15: Live scan in pm job j41468 (openFDA bulk exports, no key) grouped every recall number by shape. Device recall (59,705): `Z-NNNN-YYYY` 54,166; `Z-NNNN-YY` 5,511, initiated 1997-2006 at steady volume, so a published older form; then 9 `Z-NNNN-NNN` (cut short, all 2007-2017), 7 `Z-NNN-YYYY`, 4 `Z-NNNN-N`, 3 `ZNNNN-YYYY`, and single odd spellings (`Z-NNN-YY`, `Z-650/51-03`, lowercase `z-`, `Z-NNNN--YY`). Device enforcement (40,481): all `Z-NNNN-YYYY` except 11 typos and one blank. Drug enforcement (18,110): `D-NNNN-YYYY` 15,514; `D-NNN-YYYY` 2,541, initiated 2006-2015 (mostly 2012-2014), so a published older form; then 29 `D-NNNNN-NNN` (late 2013, the recall event number plus a product suffix, not a recall sequence), 15 `DNNNN-YYYY`, 5 `D-NNNN-NNN`, one blank, one `N/A` and singles. Only `Z-NNNN-YY` and `D-NNN-YYYY` are admitted; everything else stays declined with 0977's `record.declined`. Live exact lookups of `Z-1192-03`, `Z-0019-04`, `Z-1454-06`, `D-502-2013` and `D-096-2013` each return exactly one record.
- 2026-11-15: Changed. BioData ticket 0025 (branch `ticket/0025-older-recall-number-forms`, commit 7b138af, `make gate` passed in pm job j41470) widens `DeviceRecallNumber` to `^Z-[0-9]{4}-([0-9]{4}|[0-9]{2})$` and `DrugRecallNumber` to `^D-[0-9]{3,4}-[0-9]{4}$`; BioMCP pins that commit (`Cargo.toml`, `Cargo.lock`, `deny.toml`). `release/schema-reviews.json` records the reviewed pattern widenings for the two number schemas and the five reply schemas that carry them, with the scan evidence. Exact lookups need no adapter change: they already search `.exact` and admit only the asked number. The `get diagnostic` ID help names `Z-NNNN-YY`.
- 2026-11-15: Recorded in pm job j41472 (no key, on the new pin). `drug/recall-declined-records`: the 2013-10-18 and 2013-02-27 pages are now complete (6 and 4 rows, no problems); `--firm Olympia` 2013-05-29 shows nine `D-NNN-2013` rows and declines `D-751-013`; `--firm Qualitest` 2015-03-25 declines `D00432-2015`; `get drug D-502-2013` returns its record. `diagnostic/recall-declined-numbers` adds `--manufacturer "Terumo Cardiovascular"` 2003-08-08 (five `Z-NNNN-03` rows, next page at offset 5) and `get diagnostic Z-1192-03`; its NPR and Valorem typo steps still decline. On base 94395bcad the 2013 pages declined every older-form record and older-form `get` was a usage error (0977's j41441).
- 2026-11-15: Check job j41479 regenerated schemas (pattern and description changes only; `schema-generate` and `schema-release-check` accept the reviewed widenings, including `biomcp.diagnostic-page/6`, whose path tags branches by `identity_status`) and `docs/reference/commands.md`, and passed `make check-cheap`. Its case run passed no filter list, so job j41482 ran `drug/` (83), `diagnostic/` (48) and `mcp/` (35) cases separately: all pass.
