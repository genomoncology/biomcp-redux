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

## Progress

- `help get drug approvals` and `help drug approvals` now answer with the get drug help narrowed to the approvals section. The description reads "get drug approvals. Sources: drugsfda."; the options are those the section's sources take (an option whose schema text or source fact says "Only with --source X" drops out when X is not a section source); `--source` choices are cut to drugsfda; examples that name neither the section nor its sources are dropped; and the per-option "Command sources" sentence is left out. PM decision in review: the five CIViC options now say "Only with --source civic", `--label-section` says "Only with --source openfda-label", and `--document-id` and `--product-number` say "Only with --source drugsfda" (description-only schema edits). Measured 3,674 bytes (j39932), against 8,305 for `help get drug`; the case cap is 4,226 (measured plus 15%). Every get command with sections takes one section word in its help.
- A section word the command lacks (`help get drug approval`) is refused with the valid sections and `help get drug` as the next step, and an extra word after `help drug approvals` lists the sections.
- No reply suggests `help help ...`: `get help` and `help get help` point to `help get`; a mistyped help topic is matched against help topics, a word past a help leaf points back to that leaf, `hepl` points to `help`, and a help step built from a path already under help is not prefixed twice.
- The retired-option message for diagnostic commands now lists gtr, fda-cdx, fda-pma, fda-510k, who-ivd and fda-device-recall for search. It is a new fixed message (the error-message enum only widens); a contract test in `crates/biomcp-types/tests/diagnostic_contracts.rs` compares both lists with the checked-in source enums, so the text cannot drift from the registry.
- Cases: `tests/cases/help/section-help.yaml`, `tests/cases/help/diagnostic-sources.yaml`.
