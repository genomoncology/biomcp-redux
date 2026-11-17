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

## Progress

- j42119 (origin/main 35e17aabd): the tool fails both checks, as 1021 saw. Its build took 87 s from a cold test profile; the tool itself ran in under a minute.
- `paging::seer` broke at 0720-seer-answers-by-site-name (0504a7eef): an annual read now also fetches SEER's label catalog (`get_var_formats.php`), and the check's stub answered that request with the annual document, so the read failed as a bad reply. The check now serves the retained synthetic catalog for that request and expects two requests.
- `paging::study` broke at 0724-source-text-arrives-clean (1d04b4803): each returned sample list now reads its own count from `/sample-lists/{id}`, so the sample-lists second window makes a third request, which the stub answered with the list array (`sampleCount: bad-reply`). The check now answers that request with a synthetic count and expects three sample-lists requests.
- Neither check tests something no longer promised; both still prove the native window skips a refused row without shifting positions. The code they cover is right; the checks lagged it.
- New `make value-proof` runs the tool, and `check-cheap` runs it after `ratchets`.
- j42283: the tool passed inside `check-cheap`; clippy's 80-line limit then failed `lint-tools` on the two checks, so their stubs moved into helpers.
- j42285 (5081d7bc7): `make check-cheap` passes in 78 s on a warm M5 tree. `value-proof` adds about 32 s of that: 12 s to build the tools test binary after `ratchets` and 20 s to run the tool (a warm rerun took 20 s). On a cold tree (j42283) the build share was 9 s of a 177 s run.
- j42288 (gate, merged with main b7c395a36): the landing gate passed (580 s, pre-job 200 s), but docs-check's `gate-stage-check` failed because its shell-check fixture copies only the Makefile and omits each Rust goal of check-cheap; `value-proof` joined that omission list.
