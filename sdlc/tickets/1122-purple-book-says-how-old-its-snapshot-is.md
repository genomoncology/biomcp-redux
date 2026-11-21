# 1122 — purple-book-says-how-old-its-snapshot-is

Status: COMPLETE.
Landed: 6a6e5466d8a39b31e8f5009984bb81065a93268f

Milestone: 7

## Outcome

Every `--source purple-book` reply says how many months old its snapshot is, counted from the snapshot's release month to today. A reply that finds no product, or no biosimilar for a reference product, says that products approved or designated after the snapshot month are not included, and names `biomcp sync purple-book --file PATH` with FDA's downloads page. The release checklist has a step to replace the shipped snapshot with FDA's newest monthly file before each release.

## Why

Ticket 1094 ships FDA's October 2026 monthly file inside BioMCP. FDA's site refuses programs, so BioMCP never fetches it; a person downloads the file and either a release ships it or a user imports it. Between January and October 2026 the list gained about 120 product presentations, 32 of them biosimilar, and 11 biosimilars became interchangeable (PM comparison of FDA's January and October 2026 files, 2026-11-20). FDA's monthly change section had 11 rows in January and 14 in October. So an old snapshot quietly misses new biosimilars and new interchangeability, and today's reply only says "This is the October 2026 Purple Book snapshot", which reads as current. The governing rule from earlier tickets: only a lookup whose sources were all current may say a name was not found. Hosted servers refuse `sync`, so for them only a release brings new data.

## Proof

- Proof: a replayed case with the clock set three months after the snapshot month: the reply's notice says the snapshot is three months old.
- Proof: a replayed no-match search: the reply says later approvals are not included and names the import command; fails on the base commit.
- Proof: `release/qualification.md` names the step and the file's location in the repo (`crates/biomcp-adapters/src/purple_book/seed.csv` and its README).

## Starts from

- Starts from: `crates/biomcp-adapters/src/purple_book/read.rs` (`notice`) and `crates/biomcp-query/src/drug/purple_book.rs`, after 1094 lands.

## Keeps

- Keeps: the release month in every reply, and a failed import keeping the active release.
- Keeps: BioMCP never requests FDA's Purple Book site.

## Defers

- Defers: GenomOncology publishing monthly files for BioMCP to fetch; Ian's decision (an outward-facing service).

## Changes
