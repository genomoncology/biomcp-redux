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

## Progress

- Cause, reproduced live on the M5 against ClinicalTrials.gov with the ticket's 20 ids and `--section locations --limit 100` on base af4763f2b (job j41882, fresh cache): the batch took 16 s and answered `partial`; the first 15 trials came back, the last five (NCT06959615, NCT06973564, NCT07483983, NCT07689539, NCT07262567) were bare `{"input", "status":"failed"}`, and `sources[0]` said `failed` / `deadline-exceeded`. Asked again with a warm cache, the same batch was `complete` in 5 s. The cause is BioMCP's own: `trial::get` reads each trial as its own request, `quotas.rs` paces ClinicalTrials.gov at one request a second, and the whole batch shared one ClinicalTrials.gov leg capped at the source's 15-second deadline (`ctgov::SPEC.deadline`) inside the 20-second command budget. A cold batch can read about 15 trials before the leg ends; the rest fail with `deadline-exceeded`. In the 2026-11-16 persona run the first 18 trials were already cached from earlier turns, so only the last two ran out of time. The reason was only in `sources[0].reason` and in a `request-guidance` notice (0858), not on the item, and `problems` was null, so the agent saw nothing.
- Fix: a batched trial get now gets the time its own pacing needs. `Fetch::paced_budget` and `Fetch::paced_leg` add one pacing interval per trial after the first (19 s for 20 trials at the declared rate) to the command budget and to the ClinicalTrials.gov leg's deadline; a hosted call's end lengthens to match, as for a declared source budget (0912). A single get gets no extra time, so its behaviour is unchanged. The pacing comes from the resolved source pacing and rate share, so an operator's slower pacing gives a longer budget.
- Visible: each failed item now carries an optional `failure` with `source`, `reason` (absent only for a local storage error) and `problem`, plain words that say whether the cause is BioMCP's own time limit or pacing, a source rate limit, or the source not returning the trial, and what to do. The existing notice and the `fresh` retry first in `next` stay. Adding an optional field fits the 1.0 schema baseline.
- Decisions: kept the 20-id cap and the per-trial requests; ClinicalTrials.gov's multi-id search would cut the batch to one request, but it changes the cache and every recording, so it is not this ticket. Trials that were read stay `partial` when the source leg failed for a sibling; that is unchanged here.
