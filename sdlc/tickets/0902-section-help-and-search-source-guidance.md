# 0902 — section-help-and-search-source-guidance

Status: OPEN.

Milestone: 7

## Outcome

An agent that asks for help on one section gets that section's help, or a refusal whose next step works. The retired-option message for `search diagnostic` names every source search accepts.

## Why

In reg-affairs round 2 (biomed-agent-bench `personas/reg-affairs/runs/2026-11-13-r2/`, `calls.md` in `0041-persona-reg-affairs-r2` on the M5):
- `biomcp help get drug approvals` answered `usage.schema`, "use biomcp schema to list known artifact IDs";
- `biomcp help drug approvals` suggested `help help drug` as its next step.

The agent spent four help reads finding the approvals flags. Also, `biomcp search diagnostic --bogus x` answered "choose --source gtr, who-ivd, fda-cdx or fda-device-recall for search", but `--source fda-pma` and `fda-510k` search works, as the same run used (log.md:107-139).

## Proof

- Proof: help cases where:
  - `help get drug approvals` returns the approvals section's options and examples, or a refusal whose next step is a working help command;
  - `help drug approvals` gets the same answer;
  - no reply suggests `help help ...`;
  - the retired-option message for `search diagnostic` lists fda-pma and fda-510k among the search sources;
  - the retired-option message's source list is checked against the registry, so it can't drift.

## Starts from

- Starts from: `crates/biomcp/src/help.rs`, the retired-option messages in `crates/biomcp/src/retired/`, the source registry.

## Keeps

- Keeps: help sizes within their budgets.

## Defers

- Defers: nothing.

## Changes

- Changes: `crates/biomcp/src/help.rs`, `crates/biomcp/src/retired/`, `tests/cases/help/`, `docs/`.
