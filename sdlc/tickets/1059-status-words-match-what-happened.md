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

## Progress

- Built on top of 1072 (merged), which removes the default Europe PMC request from article search.
- Change: `crates/biomcp/src/incomplete.rs` is the one owner of the final status words. It runs on every command reply, multi-part drug gets and sync replies. A failed, stale or syncing source row turns `complete` into `partial`. A `partial` reply gets `incomplete` right after `status`, with one `{at, source, reason}` entry for each failed source row, each result node marked `unavailable`, `no-text`, `too-large`, `skipped` or failed, and each entry a command names with `status::missing`. Two fallbacks follow: refused records in `problems` (one entry per source and code), then the source's last notice. `Envelope` gains the optional `incomplete` field, and `Reason` gains `not-stated`, which Drugs@FDA now gives when a letter states no supplement purpose (`not-found` when no letter is linked). Multi-part drug gets combine the parts' settled statuses. The variant shorthand names its unresolved candidates. Exit codes are unchanged.
- Check: `crates/biomcp/tests/runner/status_words.rs` runs on every replayed case reply. `complete` never sits beside a failed, stale or syncing row or an entry, and every `partial` reply has an entry. Its first run on the branch found 49 case replies and 54 runner replies that were `partial` with nothing at the top, in about 30 reply kinds; each is now covered by one of the rules above.
- Cases: the new `drug/approvals-incomplete-top` (`get drug NDA216340 approvals --source drugsfda`, recorded live) expects `partial` and `incomplete` `[{at: /result/approvals/data/records/2/supplement_purpose, source: drugsfda, reason: not-stated}]`. `article/search/pubmed-access-timeout` now expects `partial` with `[{at: /sources/1, source: europepmc, reason: deadline-exceeded}]`. `drug/source-aliases-review-evidence` expects `partial` with MyChem and ChEMBL named. The research fetch plain-text view leads with the Incomplete table. Ceilings moved to measured plus 15 percent for `article/preprint/all` and GTR steps 1 and 10, and the stale drug-identity runner budget moved to 907 comparable bytes. The 142 generated schemas were regenerated (additive).
- Docs: the architecture section 6 paragraph and rule row, the `Status` doc comments, `docs/cautions.md` (partial replies and drug-name lookups), and release notes.
