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
