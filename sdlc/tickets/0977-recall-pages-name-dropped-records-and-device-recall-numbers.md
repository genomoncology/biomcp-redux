# 0977 — recall-pages-name-dropped-records-and-device-recall-numbers

Status: OPEN.

Milestone: 7

## Outcome

A drug recall page that drops records without a usable recall number says how many it dropped and names each by openFDA `event_id`, and a device recall page survives a record with a blank or `N/A` recall or product number the same way.

## Why

0975 review (2026-11-15):

- `crates/biomcp-adapters/src/fda_drug_recall/map.rs:37` reports a dropped record as one `value.unknown` problem with no record, and `mod.rs:133-136` deduplicates it, so two dropped records read as one and an agent cannot reach either recall. 1.0.0 allows an optional page field or a widened problem code.
- `crates/biomcp-adapters/src/fda_device_recall/raw.rs:9,24` require well-formed `product_res_number` and `recall_number`; one bad record would fail a whole device recall page, including its joined enforcement lookup (`mod.rs:173`).

## Proof

- Proof: first check live in a job whether device recall pages carry such records today, and record one if found.
- Proof: a routine case where a drug recall page drops two records shows both event IDs; a device recall case with a bad number returns the other rows as partial. Both fail on the base commit.

## Starts from

- Starts from: 0975's drug recall change; `fda_device_recall/raw.rs`, `map.rs`, `mod.rs`.

## Keeps

- Keeps: 1.0.0 schema rules (optional fields, widened enums only); `get drug D-…` and device recall lookups by number stay strict.

## Defers

- Defers: nothing.

## Changes

- Changes: drug and device recall adapters, their cases.

## Progress

- 2026-11-15: Live check in pm job j41437 (no key): scanned openFDA's bulk exports. Device recall (59,705 records, exported 2026-11-13): 5,539 have a `product_res_number` that is not `Z-NNNN-NNNN`, nearly all the older `Z-NNNN-YY` form from 2000-2006, plus a few like `Z1006-2010` and `Z-034-2010`; every malformed value is distinct, so at most one is blank. Device enforcement (40,481): 11, including one `""` (event 94520) and `Z-1548-201`-style truncations. Drug enforcement (18,110): 2,596, mostly the older `D-NNN-YYYY` form, plus `""` (event 100178) and the `N/A` 0975 found (event 99860). So device recall pages do carry such records today.
- 2026-11-15: Recorded on base 648240afe in pm job j41440 (no key). Device searches `--product-code NPR`, `--initiated-from/--initiated-to 2009-03-12`, `2009-07-21` and `2004-06` fail whole as a bad reply on the base: each page holds a record such as `Z1006-2010` (Roche Factor II kit, cfres_id 86392). Drug searches for 2013-02-27 (4 records, all declined, events 64727 x3 and 65061) and 2013-10-18 (6 records, 5 declined, all event 66568) answer `partial` with a single `recall_number` problem and no record named, so five declined records read as one.
- 2026-11-15: Changed. A declined recall record now gets its own problem with the new code `record.declined` (a widened problem-code enum; the reply shape is otherwise unchanged). The problem's `field` is the unusable identity field and its `record` names the record by an openFDA key: `event_id:NNN` for drug and device enforcement records, `cfres_id:NNN` for device recall records (the key unique to each device recall record). These problems are never merged, so their count is the number of declined records. The device recall search now reads each page row as either a record with a well-formed number or a declined row, so a malformed `product_res_number` or enforcement `recall_number` no longer fails the page or its joined lookup: the page keeps its other rows, answers `partial`, and its next page starts after every native record. Exact lookups (`get drug D-…`, `get diagnostic Z-…`) still read the strict records and accept only the asked number; a malformed number such as `Z1006-2010` is refused as a usage error before any request (j41441).
- 2026-11-15: New routine cases. `drug/recall-declined-records` (recorded j41440): the 2013-10-18 page returns `D-1161-2014` with five `record.declined` problems for event 66568; the 2013-02-27 page returns no rows and four problems, events 64727 (three) and 65061. `diagnostic/recall-declined-numbers` (recorded j41441, projected like the other device replies): `--product-code NPR` returns `Z-1652-2011` and one problem `cfres_id:86392`; `--classification "Class II" --manufacturer "Valorem Surgical"` returns four rows, one problem `event_id:77500` for `Z-3011-201`, and a next page at offset 5. Check job j41442: both cases fail with the adapter, query and type crates at base 648240afe (drug: problems name no record; device: the NPR page fails as a bad reply); all `drug/`, `diagnostic/` and `mcp-parity/` cases pass on the branch, and `schema-release-check` accepts the widened enum. Its `check-cheap` failed only on line limits and a helper name, fixed by moving the recall selector into `fda_device_recall/phrase.rs` and the declined-record helpers into `mapped/declined.rs`.
- Open: openFDA lists about 5,500 device recall records and 2,600 drug recall records under older recall-number forms (`Z-NNNN-YY`, `D-NNN-YYYY`) that the BioData recall-number types reject, so those records are declined, now by name. Accepting those forms is a BioData identifier change, outside this ticket.
