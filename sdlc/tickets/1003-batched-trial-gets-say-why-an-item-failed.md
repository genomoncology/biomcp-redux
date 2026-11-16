# 1003 — batched-trial-gets-say-why-an-item-failed

Status: OPEN.

Milestone: 7

## Outcome

When a batched `get trial` answers `partial` because one or more items failed, each failed item says why: a source, a reason and a problem, as a whole failed reply would. The same trials, asked again on their own, still succeed, and the reply's `next` still offers that retry. If the cause is BioMCP's own (a deadline, a reply-size cap or a page limit across the batch), the item says so and the cause is fixed or made visible, not left as a bare `"status":"failed"`.

## Why

The trial-ops persona hit it twice, on two builds:

- 2026-11-12 on 58d83b6da: a batch of 20 `get trial … locations` gave `partial` with four items' `locations` null, and the same four came back fine one at a time (bench `personas/trial-ops/runs/2026-11-12/log.md:46`, review "not ticketed").
- 2026-11-16 on 3521c503a, the hosting candidate (bench ticket 0057, job j41866): `biomcp get trial` with 20 NCT ids and `--section locations --limit 100` gave `status partial`, `problems: null`, and the last two items were only `{"input": "NCT07689539", "status": "failed"}` and `{"input": "NCT07262567", "status": "failed"}`. Asked again as a pair, both were `complete`. The run's record is `calls.md` turns 37-38 in the bench's records folder `0057-persona-trial-ops-pair/biomcp-cli`.

An agent can't tell a source failure from a BioMCP limit, so it retries blind. The persona's site-load table depends on this batch.

## Proof

- Proof: a routine case over recorded replies with a 20-id batch where one item fails (a synthetic failing recording, or whatever reproduces the real cause). The failed item carries a reason and a problem. The case fails on the base commit.
- Proof: Progress names the real cause of the 2026-11-16 failure, found by replaying or reproducing it.

## Starts from

- Starts from: the batched trial-card path (`get trial` with several inputs), how it sets an item's status, and the reply envelope's per-item failure fields used elsewhere.

## Keeps

- Keeps: the 20-id cap, `next`'s fresh retry, and every reply field for complete items.

## Defers

- Defers: nothing.

## Changes

- Changes: the trial batch path, one case and its recordings, and a release-notes line if a reply field changes.
