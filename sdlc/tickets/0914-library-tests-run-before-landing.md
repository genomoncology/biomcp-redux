# 0914 — library-tests-run-before-landing

Status: OPEN.

Milestone: 7

## Outcome

A ticket's own library tests run before it lands, so a new test can't pass review without running.

## Why

The landing gate (`scripts/landing_gate.py:9`) runs rights, audit, secrets, pre-job, the first-read check, workflow lint and cases. Library tests (`library-tests`) run only in `make full-suite`. On 2026-11-13, 0909's new `crates/biomcp-fetch/tests/record/command_budget.rs` was compiled by clippy, but its gate never ran it. The reviewer had to run it by hand (j39853). This was also flagged earlier in the day: adapter unit tests don't run in the gate.

## Proof

- Proof: a test that changes under a ticket runs before landing, and the gate log names it. Either:
  - the gate runs the library tests of the crates the branch changed; or
  - `pm ticket land` runs `make full-suite`;

  whichever costs less on the M5. Measure both and record the times.

## Starts from

- Starts from: `scripts/landing_gate.py`, `Makefile` (`landing-gate`, `full-suite`, `library-tests`).

## Keeps

- Keeps: the landing gate's speed, within what the measurement shows is worth it.

## Defers

- Defers: nothing.

## Changes

- Changes: `scripts/landing_gate.py` and/or `Makefile`.

## Progress

Chosen: the landing gate runs the library tests of the crates the branch changed. It adds a final `changed-library-tests` stage (`scripts/changed_library_tests.py`, `make changed-library-tests`). The stage diffs `HEAD` against its merge base with `origin/main`, which pm fetches into every job checkout. A change under `crates/NAME/` selects that crate. A change to `Cargo.toml`, `Cargo.lock`, the toolchain file or `.cargo/` selects every crate. If no base is found, every crate runs. The log names the base, the crates and each cargo command, and cargo names every test binary and test. The stage uses `--no-fail-fast`, so one gate names every failing test. For `biomcp` it runs only `--bin biomcp`, because the gate already runs that crate's cases and ratchets. `LIBRARY_TESTS_BASE` overrides the base, which the measurements below used. `scripts/test_gate_stages.py` covers the crate selection against real Git, with a stub cargo.

Times on the M5 (seconds, from the gate's stage lines; each job shared the machine with other jobs):

| Job | What ran | Total | Notes |
|---|---|---|---|
| j39966 at 28d703eb1 (main) | landing gate alone | 1,205 | pre-job 334, cases 850 |
| j39968 at 771d586bb | landing gate + all six crates' library tests (base set before 0909, which touched every crate) | 1,419 | library stage 213, stopped at the red test below; 0909's `command_budget` test ran and passed |
| j40033 at 90b02bdd9 | report gate (`landing-gate docs-check schema-release-check`); this branch changes only `biomcp-types` | 1,220 for the landing gate | library stage 61, crates=biomcp-types; passed |
| j39967 at 28d703eb1 (main) | `make full-suite` | 910 to the failure | secrets-full 40, pre-job 317, docs-check 329, tensor-proto-check 18, library-tests 205 to the red test; coverage never ran |
| j40034 at 90b02bdd9 | `make full-suite` | pending | started 2026-11-13; its time was not recorded before the stop |

Running the changed crates costs 1 to 4 minutes on top of a 20-minute gate. The full suite repeats pre-job and adds docs-check, the whole library-tests run and coverage, so it costs more than the changed-crate stage.

Red test on main, fixed here: `request_boundaries_reject_missing_text_null_limits_invalid_dates_and_removed_options` (`crates/biomcp-types/tests/adverse_event_contracts.rs:311`). 0894 deliberately made a bare year a valid receipt bound (its Progress: the dates "now also accept a bare year (`2021`)"), so `received_to:"2024"` and MAUDE `received_from:"2020"` moved to the accepted list with a comment naming 0894. j40005 at 17876272a then ran every crate's library tests with `--no-fail-fast`, and all passed, so no other library test is red on main.

Pick up: read j40034's total from `pm job list 0914` and record it in the table above. Then land.
