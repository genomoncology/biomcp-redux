# 1034 — replayed-name-lookups-finish-as-they-did-live

Status: OPEN.

Milestone: 7

## Outcome

A call recorded live and then replayed gives the same answer: name lookups that ran side by side live do not run out of time in replay. `search drug "dabigatran etexilate" --source fda-recall` (or the `get` form 1033 tried) replays as it ran live.

## Why

1033 (2026-11-17) dropped an FDA recall replay step: in replay the name lookup came back "name lookup unfinished" (`crates/biomcp-query/src/drug/search_names.rs:162`) and retried "Pradaxa", which was not recorded. Its review suspects replay charges each recorded request's delay one after another against one deadline (`crates/biomcp-fetch/src/request.rs:467-473`), so lookups that ran in parallel live add up past it. If so, any source that fans out name lookups is affected, and replay cases can't cover them.

## Proof

- Proof: 1033's dropped recall step, recorded live and replayed, matches the live answer; it fails on the base commit.

## Starts from

- Starts from: the replay delay accounting and the drug name fan-out.

## Keeps

- Keeps: live deadlines and pacing.

## Defers

- Defers: nothing.

## Changes

- Changes: replay timing, the recall case, its recording.
