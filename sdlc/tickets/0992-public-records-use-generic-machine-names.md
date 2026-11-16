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

## Progress

- 2026-11-16 scan: the check's hashed sets caught 209 lines in 95 files, all under `sdlc/`; `tests/replies/` and `tests/fixtures/` held none, so their exemption stays. Names are counted by hash only here.
- Machines became roles: the coordination machine is "the PM's machine", the shared test machine is "the speed-test machine", and a build host's account name became "the build machine". Machine words the check treats as common words outside an SSH path stay as they were, as elsewhere in public files.
- Paths keep their shape with placeholders, explained in `sdlc/README.md`: `{workspace}` for a machine's workspace root, `{home}` for its home folder, `{repo}` for this repository's working name.
- `scripts/check_docs.py` now checks `sdlc/`. The retired-tool-name check still skips `sdlc/`, as before, since records cite retired names on purpose. `scripts/test_docs.py` plants a workspace path and a made-up hashed machine name in an `sdlc/` record and shows each fails.
- Left open: `sdlc/data/` stays exempt. It is the registry `pm disk` reads, and each Location line must name a real machine and folder for pm to place it, so a role word there would leave this team's data unregistered. Clearing it needs those entries moved to a private registry, which belongs to the pm and rules owners.
