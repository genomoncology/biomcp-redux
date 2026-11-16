# 0992 — public-records-use-generic-machine-names

Status: OPEN.

Milestone: 7

## Outcome

Files on main name no internal machine. `sdlc/` uses generic words such as "the build machine", and the repo's internal-word check covers `sdlc/` too, so new names fail at landing.

## Why

Ian, 2026-11-16 (`sdlc/decisions/2026-11-16-review-follow-ups.md`, item 3), on finding D7 of `sdlc/records/2026-11-14-adversarial-review.md`.

`scripts/check_docs.py:36` exempts `sdlc/` (with `tests/replies/` and `tests/fixtures/`) from the internal-word check by a code comment only. Public planning records and tickets name the machines that check hashes as internal. After the history append (0294), past commits become permanent public history, so the current tree must be clean before then.

## Proof

- Proof: the internal-word check runs over `sdlc/` with no exemption, and passes on main.
- Proof: a check test shows an internal machine name in an `sdlc/` file fails.

## Starts from

- Starts from: `scripts/check_docs.py` (the hashed name sets and `INTERNAL_SKIP`); the finding's hash list.

## Keeps

- Keeps: the hashes, so the check never prints the names; ticket and record meaning (replace names with roles, such as "the build machine" or "the PM's machine"); the `tests/` exemptions unless they also hold names.

## Defers

- Defers: rewriting past commits. That decision belongs to the history append (0294).

## Changes

- Changes: `sdlc/` text, `scripts/check_docs.py` and its test.
