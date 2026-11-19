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

## PM decisions

- PM decision, 2026-11-19: adopt the rule in `sdlc/planning/status-words-proposal.md`. A reply is `complete` only when every source row answered or was skipped on purpose. Any failed, stale or syncing row makes an answered reply `partial`. `absent` and `failed` are unchanged. This deliberately reverses ticket 1046's choice: a status word a caller cannot trust is worse than an honest `partial`.
- PM decision, 2026-11-19: every `partial` reply carries the `incomplete` list right after `status`. Each entry names the source and reason, points to the detail already in the reply, and stays one short line, because the caller is an agent paying per token. Add the `not-stated` reason code.
- PM decision, 2026-11-19: enforce both rules in the one place where the reply is built. Add a check across every replayed reply that `complete` never sits beside a failed source and that every `partial` reply has an entry.
- PM decision, 2026-11-19: ticket 1072 (article open-access check) is investigated and lands first, so the new rule does not turn ordinary article searches `partial` on the day it ships. Build 1059 on top of 1072 if they touch the same code.
- PM decision, 2026-11-19: drug search name lookups (MyChem, ChEMBL) that fail make the reply `partial` under the new rule. That is correct; keep it and say so in `docs/cautions.md`.
