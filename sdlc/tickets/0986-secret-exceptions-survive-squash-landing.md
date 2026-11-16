# 0986 — secret-exceptions-survive-squash-landing

Status: COMPLETE.
Landed: 08710171f214f13a31458d84f9c699abc4b34838

Milestone: 7

## Outcome

A reviewed secret-scan history exception written on a ticket branch still holds after `pm ticket land` publishes the ticket as one new commit, so no lander has to rebind rows by hand and `make secrets` on main never fails with `history-ignore-unreachable-commit` because of a landing.

## Why

Root cause of a problem repeated across tickets on 2026-11-15:

- 0831's rows in `release/secrets.ignore.jsonl` named branch commits; after landing, `make secrets` on main would fail (`release/secrets.py:315-316`), and its lander rebound them by hand (main 5c56dad9a).
- 0981's first landing gate (j41510) failed the same way on the merged tree, and its lander copied the same rebinding.
- 0982 adds new rows bound to its branch commits, which landing will make unreachable again.

`history_ignores` (`release/secrets.py:295-335`) requires `row["commit"]` to be reachable and to hold `row["blob"]` at `row["file"]`. `pm ticket land` publishes a new single-parent commit (keeping the tip only under `refs/pm/tips/…`), so every branch-bound row breaks. The landing gate does not run `make secrets` (`sdlc/planning/handover-2026-11-11.md:64`), so the break shows only later.

## Proof

- Proof: a test in the secrets check's own tests binds a row to a commit that is then rewritten (new commit, same blob at the same path) and shows the check still accepts it, while a row whose blob is not reachable at that path, or a changed line, still fails.
- Proof: `make secrets` passes on main after 0982 lands with its rows as written.

## Starts from

- Starts from: `release/secrets.py` `history_ignores` and `scanner_config`; the fixture-form rules it already applies outside history mode.

## Keeps

- Keeps: exceptions stay exact to blob, file, line and rule (an immutable reviewed binding); no pattern exemption in history mode wider than today's fixture forms; the scan still covers full history.

## Defers

- Defers: nothing.

## Changes

- Changes: `release/secrets.py`, its tests, `release/secrets.ignore.jsonl` if rows simplify.

## Progress

- 2026-11-15: Chose the fallback over dropping the `commit` field. When a row's commit is reachable, the check is unchanged: that commit must hold the row's blob at the row's file. When it is not (a squash landing published a new commit), `release/secrets.py` `held` accepts the row only if some reachable commit (same `--single-worktree --all HEAD` scope) holds the same blob at the same file; otherwise it still fails with `history-ignore-unreachable-commit`. Rows stay exact to blob, file, line and rule, no row in `release/secrets.ignore.jsonl` changes, and rows that 0982 and 0849 add on their branches keep working after they land. Dropping the field would have rewritten all 226 rows and conflicted with those branches for no extra safety, because the blob and path already identify the reviewed content.
- Tests (`release/test_security.py`): new `test_history_exceptions_survive_a_squash_landing` rewrites the branch as one new commit on main with the same tree and deletes the branch, then shows the rows still pass, while a wrong line (`history-ignore-wrong-line`), another rule, another path, another blob, a later changed blob, and a reachable commit without the blob (`history-ignore-wrong-blob`) still fail. The existing exact-binding test's detached-sibling row now passes, because the reachable base commit holds the same blob at the same path.
