# 1223 — purple-book-search-biosimilar-names

Status: COMPLETE.
Landed: 9f38f8606017a2b76324117eb9fddfc3f7507966

Milestone: 10

## Outcome

`search drug <name> --source purple-book` finds a licensed biologic by any product's proprietary name, proper name or BLA number in the Purple Book file, including biosimilars and reference products that have no biosimilar.

## Why

A biosimilar's own name returns zero rows today with a notice that reads as if the product were newer than the file.

## Evidence

Checked 2026-11-27 against the live source and the 1.0.0 build at 4efd1dcb2426. Full tables: [source reach census](../records/2026-11-27-source-reach-census.md).

- Failing case: `biomcp search drug Inflectra --source purple-book` returns `status: complete`, `total: 0`; so do its proper name and BLA. Inflectra (BLA125544, infliximab-dyyb) is the first item of `biomcp search drug infliximab --source purple-book`. `Zirabev`, approved in 2019, behaves the same.
- Keytruda, Kymriah, Enhertu and Recombivax return 0: reference products with no biosimilar are not matched either.
- Cause: the search matches reference-product names only. Ticket 1094 defined the roster that way; the reply never says so.

## Proof

- Proof: an offline gate test: every proprietary name, proper name and BLA number in the shipped Purple Book file resolves to its own row. It runs in seconds.
- Proof: outside-in cases: Inflectra returns its row and its reference product; Keytruda returns its licence row.

## Starts from

- Starts from: `crates/biomcp-adapters/src/purple_book/decode.rs`.

## Keeps

- Keeps: a reference-product search still lists its biosimilars and interchangeable products.

## Defers

- Defers: nothing.

## Changes

- Changes: `crates/biomcp-adapters/src/purple_book/`, drug cases.
