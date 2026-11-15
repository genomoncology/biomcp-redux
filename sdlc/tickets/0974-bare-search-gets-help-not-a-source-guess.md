# 0974 — bare-search-gets-help-not-a-source-guess

Status: OPEN.

Milestone: 7

## Outcome

A search refused for missing input whose fix would be one of several sources (bare `search drug`) gets the help step first, not a suggested call to whichever source the parser tried first. A source is suggested only when exactly one source makes the call valid.

## Why

0904 review (2026-11-15): bare `search drug` on main suggests `search drug --source fda-drug-recall` ahead of help. Nothing in the call points at recalls; the suggestion is the first source in declaration order that makes the call parse. An agent that runs the first next step searches recalls by accident. 0904 kept main's behaviour (the guess shown with help beside it) because changing it was outside that ticket.

## Proof

- Proof: `tests/cases/grammar/usage-missing-option.yaml` asserts bare `search drug` gets help as its first next step and no `--source` guess; it fails on the base commit.
- Proof: a case where exactly one source works (as `get disease MONDO:0013267 genes` needs `--source monarch`) still gets that call as its first step.

## Starts from

- Starts from: the older missing-option correction path in `crates/biomcp/src/grammar/recovery.rs` that 0904 left in place (0904's Progress names it), after 0904 lands.

## Keeps

- Keeps: certain corrections (the field's default, or the only valid value) replacing help as 0904 sets; the refusal schema; MCP typed refusals matching the CLI.

## Defers

- Defers: nothing.

## Changes

- Changes: the missing-option correction path, its cases.

## Progress

### Built, 2026-11-15

- Measured first (j41422): bare `search drug` offered `--source fda-drug-recall` because recalls are the only drug source that runs with no name, not because the call asked for them. `variant normalize --source all --hgvs ...` without `--build` offered whichever build was declared first.
- `grammar/recovery.rs::corrected` now offers a listed value only when it is certain: the declared default, or the only value that passes when the call already names something. A bare command, or several values that each pass, get help alone. Dropping a refused filter is still offered as a guess with help beside it, and the source swap (`source_swap`) is unchanged.
- Cases: `grammar/usage-missing-option.yaml` expects help as the only next step for bare `search drug`; `grammar/errors-name-the-real-problem.yaml` expects help alone for the missing build; `grammar/source-filter-errors.yaml` adds `search adverse-event pump --event-type malfunction`, where no source was named and FAERS (the default) refuses the filter, which gets the MAUDE call as its only next step (0904's certain swap path); `mcp/typed-refusals.yaml` adds the typed `drug` search with no arguments, which gets help alone as the CLI does. The three `--source hpo` refusal caps in `phenotype/admission.yaml` are 458 (measured 398 plus 15%).
- j41419 ran format, schema and docs regeneration (nothing changed) and `check-cheap`; j41423 ran every case (969 passed).
