# 0867 — trial-fields-name-what-the-source-lacks

Status: COMPLETE.
Landed: 9c802816dae7b38da1a543c713fb4ae558fcc5b3

Milestone: 7

## Outcome

`search trial --fields` never silently drops a chosen field: a field the selected source's rows cannot carry (today `phase`, `matching_sites`, `matched_site` on ClinicalTrials.gov) is refused as a usage error or named in the reply, and `help search trial` says which fields belong to which source.

## Why

Bench 0040's rerun (2026-11-12) found four personas losing columns silently (failure #2 in `sdlc/records/2026-11-12-persona-rerun-failures.md`): those fields exist only on NCI rows (`crates/biomcp-types/src/response/nci_trial.rs:20-26`), but `--fields` accepts any name any trial reply can carry. The gap came with 0838.

## Proof

- Proof: a routine case runs `search trial --sponsor ... --fields nct_id,phase` on ClinicalTrials.gov and shows the refusal or the named missing field; the same command with `phases` returns the field; NCI rows still return `phase`.

## Starts from

- Starts from: `crates/biomcp/src/fields.rs` (`check`), 0838's field choice, the trial row types.
- PM decision: 2026-11-12. Prefer a usage error naming the field and the source's nearest field (`phase` to `phases`) when the source is known at parse time; name the field in the reply only when it is not.

## Keeps

- Keeps: fields valid for the source unchanged; NCI rows unchanged.

## Defers

- Defers: other entities' `--fields` (checked by the same rule if the developer finds the same gap; otherwise nothing waits).

## Changes

- Changes: `crates/biomcp/src/`, `crates/biomcp-query/src/trial/`, `tests/cases/trial/`
