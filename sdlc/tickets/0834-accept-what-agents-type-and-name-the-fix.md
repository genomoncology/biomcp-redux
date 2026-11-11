# 0834 — accept-what-agents-type-and-name-the-fix

Status: OPEN.

Milestone: 7

## Outcome

The commands agents actually typed in the baseline either run or fail with an error that names the option at fault and the exact form to use: short flags `-q`, `-c` and `-i`; a section given as a positional word or as `--section`; retired command names, which answer with the replacement command written out for this request; and no usage error leaves the option unnamed.

## Why

In `biomed-agent-bench/results/baseline-1.0.md` (build c2511ee90, which is current main) Sonnet hit a usage error naming no option 40 times, wrong section forms 43 times, short flags 22 times and retired commands 20 times. Each is a wasted turn and a reason to leave for curl.

## Proof

- Proof: routine cases under `tests/cases/grammar/` replay each failing call text from the baseline's failed-call table and assert it runs or returns the named fix; a check fails if any `usage.arguments` reply lacks an option or argument name.

## Starts from

- Starts from: `crates/biomcp/src/grammar.rs`, `crates/biomcp-types/src/response/error.rs`, the retired-command table, and the failed-call tables at the end of the baseline report.

## Keeps

- Keeps: strict checking of values; nothing is guessed silently: an accepted alias is echoed in the reply's query as the canonical form.

## Defers

- Defers: fuzzy matching of unknown option names waits for evidence that it is needed.

## Changes

- Changes: `crates/biomcp/src/grammar.rs`, `crates/biomcp-types/src/response/`, `tests/cases/grammar/`
