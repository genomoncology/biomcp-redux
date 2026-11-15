# 0975 — drug-recall-status-search-pages-parse

Status: OPEN.

Milestone: 7

## Outcome

`search drug --source fda-drug-recall` with `--status` or `--reason` returns its page at the default limit, as it does at `--limit 1`, instead of a bad-reply failure.

## Why

0841's live reproduction (2026-11-15, pm job j41358 on d61945035): `search drug --source fda-drug-recall --status Ongoing` and `--reason contamination` fail as a bad reply at the default page size of 10, with plain values, and work at `--limit 1`. Some record on the larger page probably holds a field shape the recall parser does not expect.

## Proof

- Proof: first record the failing default-size page live in a job and find the record and field that fail to parse.
- Proof: a routine case with that recorded page returns its rows; it fails on the base commit.

## Starts from

- Starts from: the openFDA drug recall adapter under `crates/biomcp-adapters/src/` and its reply parsing.

## Keeps

- Keeps: the reply schema (1.0.0 rules: optional fields only); other recall filters' results.

## Defers

- Defers: nothing.

## Changes

- Changes: the drug recall reply parsing, `tests/cases/`.

## Progress

- 2026-11-15: Recorded live on base 8ac4f1674 in pm job j41420 (no key, bodies unchanged; table in `tests/replies/drug/recall-status-pages/README.md`). Both default pages fail as a bad reply on the base, and `--status Ongoing --limit 1` answers. Cause: openFDA publishes some enforcement records with no usable recall number. On the `--status Ongoing` page the third record (Trifluent Pharma lidocaine patch, event 100178) has `"recall_number": ""`; on the `--reason contamination` page the ninth record (Centric Compounding Tri-Immune Boost, event 99860) has `"recall_number": "N/A"`. The parser required a well-formed `D-NNNN-YYYY` recall number on every record, so one such record failed the whole page.
- 2026-11-15: A record without a well-formed recall number is now declined instead of failing its page: the page keeps its other rows, reports a `recall_number` problem (value unknown, no record id), and its status is `partial`. The next page starts after all native records on the page, declined ones included, so no record is skipped or repeated. Exact `get drug D-…` is unchanged: it still admits only the record carrying the asked number. The reply schema is unchanged. New routine case `drug/recall-status-pages` replays both recorded pages; on the base commit both steps fail as a bad reply.
- 2026-11-15: Cheap job j41421 regenerated schemas and docs with no change and passed `make check-cheap`; replaying the recorded pages gives `partial`, 9 of 10 rows, total 2,857 (`Ongoing`) and 2,073 (`contamination`), one `recall_number` problem, and a next page at offset 10. Its only failure was the case's expected argument order for the next step, now fixed.
