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
