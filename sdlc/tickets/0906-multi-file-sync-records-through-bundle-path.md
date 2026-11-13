# 0906 — multi-file-sync-records-through-bundle-path

Status: OPEN.

Milestone: 7

## Outcome

`biomcp --record` can record a multi-file sync (such as `sync orphadata`), and its replay works, when one file is larger than the ordinary 16 MiB recording limit. Developers then no longer have to cut those test fixtures by hand.

## Why

Found in the review of 0889 (Orphanet prevalence, 2026-11-13), by reading the code. `sync_set` (`crates/biomcp-fetch/src/bundle/set.rs:89-104`) fetches each file through the ordinary `leg.call`. That path records with `Body::capture` and `append`, which have the ordinary 16 MiB limit (`record.rs:21`), not the larger bundle limit that a single-file sync uses (`append_bundle`, `download.rs:290`). Orphadata's `en_product9_prev.xml` is 16,178,169 bytes, and 17,053,377 bytes once stored as JSON text, so recording fails. 0889 and GenCC both cut their fixtures by hand.

## Proof

- Proof: a test that records and replays a two-file sync where one file is over 16 MiB once encoded (synthetic is fine). It fails on main and passes after the change.

## Starts from

- Starts from: `crates/biomcp-fetch/src/bundle/set.rs`, `crates/biomcp-fetch/src/bundle/download.rs` (`append_bundle`), `crates/biomcp-fetch/src/record.rs`.

## Keeps

- Keeps: the ordinary 16 MiB limit for non-bundle replies; sync atomicity.

## Defers

- Defers: re-recording the hand-cut fixtures, which stay as they are unless a later ticket needs them whole.

## Changes

- Changes: `crates/biomcp-fetch/src/bundle/`, `crates/biomcp-fetch/src/record.rs`, `crates/biomcp-fetch/tests/` or `tests/cases/`.
