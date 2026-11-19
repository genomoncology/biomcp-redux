# 1070 — first-read-fetch-backs-off-and-says-what-happened

Status: OPEN.

Milestone: 7

## Outcome

After a first-read fetch of a stored source fails or times out, BioMCP waits a short time before trying that fetch again, so repeated calls do not download the same file again and again. The reply to the caller says a fetch was tried, why it did not finish, and the source's size, beside the `sync` step it already offers.

## Why

Ticket 1055 made a missing stored source fetch itself on first read. Its reviewer found two gaps that did not block it. On a hosted server, a missing multi-file source whose fetch keeps failing is downloaded again on every call, up to 32 MiB each (GenCC is 27 MB with an 8-second read limit), with no wait beyond the source's request quota; the genetic test registry may download its first file before being refused for size. And a caller whose first read timed out gets the old `syncing` reply, with nothing saying a fetch was tried.

## Proof

- Proof: with a stub that always fails the fetch, three reads in a row make one fetch attempt, not three, and the second and third replies say when the next attempt is allowed; it fails on the base commit.
- Proof: a read refused for size or time says so in the reply, with the size; it fails on the base commit.

## Starts from

- Starts from: `crates/biomcp-fetch/src/bundle/read.rs` and `set.rs` from ticket 1055; multi-file sources take no lock in `publish_set`, so two first readers both download.
- The size is known before download for some sources and not others; the developer says which, and what the reply gives when it is not known.
- A new field in the reply changes its schema; follow the repo's schema rules.

## Keeps

- Keeps: offline and replay never fetch. A source the user disabled is never fetched. An explicit `biomcp sync` is never delayed by the wait.

## Defers

- Defers: raising the read time limit for the two largest sources.

## Changes

- Changes: the stored-source read path, the syncing reply, cases and docs.
