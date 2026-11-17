# 1025 — value-proof-seer-and-study-paging-pass

Status: OPEN.

Milestone: 7

## Outcome

`tools --other-source-value-proof` passes its `paging::seer` and `paging::study` checks on main, or those checks are removed with the reason, and the tool runs in a gate or check target so it cannot rot unseen again.

## Why

1021's job j42103 (2026-11-16) found both checks failing (`crates/biomcp/tests/runner/other_source_value_proof/paging.rs:6`, `:122`). 1021's review showed the branch changes nothing they use, so they fail on main too. Nothing runs the tool automatically.

## Proof

- Proof: the tool passes in a pm job on main, and the target that runs it is in `make check-cheap` or the landing gate.

## Starts from

- Starts from: the two checks, their SEER reply and cBioPortal test data, and the commit that last changed SEER or study paging.

## Keeps

- Keeps: every other value-proof check.

## Defers

- Defers: nothing.

## Changes

- Changes: the two checks or the code they cover, and the Makefile target.
