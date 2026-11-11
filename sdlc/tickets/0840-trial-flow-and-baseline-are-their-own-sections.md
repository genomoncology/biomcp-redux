# 0840 — trial-flow-and-baseline-are-their-own-sections

Status: OPEN.

Milestone: 8

## Outcome

`get trial ID flow` and `get trial ID baseline` return participant flow and baseline characteristics directly, without paging through the posted outcomes first.

## Why

The review of the participant-flow ticket found that with outcomes first in `results`, NCT01844505's flow starts on page 12, after about 152 KB, with no way to jump there. An agent asking how many dropped out should pay for that answer only.

## Proof

- Proof: a routine case under `tests/cases/trial/` asserts that the first record of `get trial NCT01844505 flow` is a flow record and that the reply is under a measured byte ceiling; it fails on the base commit.
- Proof: the existing trial search cases' byte ceilings are re-measured against rows that carry the new sponsor, date and enrollment fields.

## Starts from

- Starts from: the results section's record ordering and paging in the trial adapter and query crates, as landed by the participant-flow ticket.

## Keeps

- Keeps: `results` unchanged, with outcomes first, then flow, baseline and adverse events; schema ID rule; source attribution; no new source, key or dependency.

## Defers

- Defers: nothing.

## Changes

- Changes: the trial section list, `crates/biomcp-adapters/src/ctgov/`, `tests/cases/trial/`, the command reference
