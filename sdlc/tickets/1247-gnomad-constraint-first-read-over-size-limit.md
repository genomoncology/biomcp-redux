# 1247 — gnomad-constraint-first-read-over-size-limit

Status: OPEN.

Milestone: 10

## Outcome

On a store that holds no gnomAD constraint table, `get gene SYMBOL constraint` either fetches the table and answers, or says plainly that the table must be synced first. Its `next` is a call that works, such as `sync gnomad-constraint-v4.1`. It never reports `syncing` with a 60-second retry when nothing is syncing and a retry cannot succeed.

## Why

Found in a persona evaluation on 2026-11-28 (GLM 5.3, BioMCP command and skill, a fresh store per run). The first constraint read fetched the 95,546,041-byte gnomAD v4.1 table, went over the first-read size limit, and returned `status: partial`, source state `syncing`, reason `syncing`, `retry_after_seconds: 60`, and `next: [... --fresh]`. Nothing was syncing. The `--fresh` call fails the same way. The one step that works, `biomcp sync gnomad-constraint-v4.1`, appears only in a notice's prose. The agent read 21 `syncing` replies across eight genes, then downloaded the 95 MB table with `curl` and computed the values itself.

Ticket 1138 separated a publisher's refusal from a sync. This is the other first-read failure: the size limit, and by the same code path a first read that ran out of time (`crates/biomcp-fetch/src/bundle/first.rs`, `first_failure`). The same run shape hit GenCC: a 27,143,351-byte first read ran out of time and read as `syncing`, though there `next` did offer `sync gencc`.

## Evidence

Built binary 1.0.0 at 0e329b6d1b59, fresh store:

- `biomcp get gene ATM constraint --constraint-release v4.1 --section-only` returns `"state":"syncing","reason":"syncing"`, the notices "The first-read fetch exceeded the size limit. Source size: 95546041 bytes." with `retry_after_seconds: 60` and "gnomAD v4.1 constraint table is unavailable locally. Run biomcp sync gnomad-constraint-v4.1.", and `next` `get gene ATM constraint --constraint-release v4.1 --section-only --fresh`.
- The same call with `--fresh` returns the same `syncing` reply.

## Proof

- Proof: outside-in cases on a fresh store with a recorded oversize first read and a recorded first read that runs out of time, for gnomAD constraint and GenCC: the source state is not `syncing`, the reason names the size limit or the time limit, `retry_after_seconds` is absent or reflects when a retry could succeed, and the first `next` is the sync call for that bundle, which then serves the section. A true in-progress sync still reads `syncing`.

## Starts from

- Starts from: `crates/biomcp-fetch/src/bundle/first.rs` (`first_failure`, `first_notice`) and the constraint section in `crates/biomcp-query/src/gene/sections/constraint.rs`.

## Keeps

- Keeps: the first-read size and time limits; single-flight first reads; ticket 1138's publisher-refusal reply; hosted mode never suggests an explicit sync.

## Defers

- Defers: raising the size limit or syncing large bundles in the background without being asked.
