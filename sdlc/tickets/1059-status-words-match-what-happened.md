# 1059 — status-words-match-what-happened

Status: OPEN.

Milestone: 7

## Outcome

A reply's top-level `status` can be trusted without reading every record. `complete` is never given when a source the command asked failed. `partial` says at the top of the reply what is missing.

## Why

Agents read the status and move on. Two replies seen this week said something different from what happened, in opposite directions.

## Proof

- Proof: a replayed article search with one of its two sources timing out replies `partial`, with the failed source named at the top; and `get drug NDA216340 approvals --source drugsfda` says at the top which record is incomplete and why. Both fail on the base commit.

## Starts from

- Starts from: Seen on main be96d8d40 (1.0.0), Linux, 2026-11-19, while agents used the command line build on real tasks.
- `biomcp search article 'AKT1 E17K'` on 2026-11-19, while Europe PMC was not answering, replied `status: complete` with the europepmc source marked failed (deadline exceeded). The nightly live check caught the failed source; the status did not.
- `get drug NDA216340 approvals --source drugsfda` replies `partial` although all four records are returned. The cause is one record whose supplement purpose could not be read from FDA's letter, and only that record says so.
- The architecture's status rules decide the exact words. If they already require this, the fault is in these two commands; if they do not, the developer proposes the rule change to the PM before coding.

## Keeps

- Keeps: exit codes, and the per-source and per-record detail already in replies.

## Defers

- Defers: nothing.

## Changes

- Changes: the status functions or the two commands, their cases, and the status paragraph in the docs.
