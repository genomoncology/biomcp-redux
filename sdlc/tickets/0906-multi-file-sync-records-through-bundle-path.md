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

## Progress

- Measured first: the Why's diagnosis does not hold on main. `part_body` reads every set part with `Read::bytes()`, so `record_wire` stores each part as a binary sidecar (`Body::Bytes`, `record_sidecar.rs`), not through `Body::capture`; the record file holds only the sidecar name, and the 16 MiB file limit never applies to a part. The 0889 Orphadata fixtures are hand-written `text` bodies, not a failed capture. Job j41301 (d96485fbe) ran a two-part synthetic set whose second part is 17 MiB of quoted CSV: it recorded and replayed on main. The same set with a 51 MiB part failed to record with `Recording(Limit)`: the real ceiling is the 50 MiB binary sidecar cap, which an ordinary binary reply also has.
- Changed: a set part's read is marked `set_part`, and its sidecar is recorded and replayed under the largest part a set accepts (`MAX_STREAMED_BYTES`, 512 MiB), still bounded by the part's own byte limit. Ordinary binary replies and gRPC messages keep the 50 MiB cap, and JSON and text replies keep the 16 MiB record file limit. Sync atomicity is unchanged: recording happens inside the part's read, before any row is published.
- Proof: `set_record::multi_file_sync_records_and_replays_a_part_over_sixteen_mib` (passes on main and after) and `set_record::multi_file_sync_records_and_replays_a_part_over_the_binary_reply_limit` (51 MiB; failed on main in j41301 with `Recording(Limit)`, passes after in j41303) in `crates/biomcp-fetch/tests/bundle/set_record.rs`.
- Gate j41318 passed on acefe904d (`make --keep-going landing-gate docs-check schema-release-check`; branch level with origin/main). The 0889 Orphadata and GenCC fixtures stay hand-cut, as the Defers says.
