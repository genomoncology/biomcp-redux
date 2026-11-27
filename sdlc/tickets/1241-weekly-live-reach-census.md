# 1241 — weekly-live-reach-census

Status: OPEN.

Milestone: 10

Depends on: 1240

## Outcome

`make weekly-reach-census` runs the live census: counts and a random sample per record class for each source, through the BioMCP binary, compared with a recorded floor. A drop below a floor fails the job and names the source and class. pm schedules it weekly.

## Why

The offline check (1240) sees only pinned slices. Sources change their releases and record classes, and the census showed reach drifting silently.

## Evidence

- PM ruling 2026-11-27: the weekly live job is approved as proposed, at about three hours of polite wall time, a few thousand requests and no paid calls.
- Full method: [source reach census](../records/2026-11-27-source-reach-census.md).

## Proof

- Proof: one full run through `pm job` records each source's floor; a second run reports equal or better reach; a deliberately lowered floor fails with the source named. FDA sources are sampled only through BioMCP, never fetched directly.

## Starts from

- Starts from: the census helpers' scripts, `scripts/nightly_live_check.py` for the job shape.

## Keeps

- Keeps: polite request rates; no source keys printed; no direct FDA fetches.

## Defers

- Defers: sources that need a license we do not hold.

## Changes

- Changes: `scripts/`, `Makefile`. The schedule entry lives in pm's `config/jobs.json`, so the PM asks pm's owner to add it beside `nightly-live-check`.
