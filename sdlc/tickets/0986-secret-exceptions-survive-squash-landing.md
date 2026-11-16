# 0986 — secret-exceptions-survive-squash-landing

Status: OPEN.

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
