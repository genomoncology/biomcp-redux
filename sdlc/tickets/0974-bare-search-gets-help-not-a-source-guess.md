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
