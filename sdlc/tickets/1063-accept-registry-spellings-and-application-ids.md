# 1063 — accept-registry-spellings-and-application-ids

Status: OPEN.

Milestone: 7

## Outcome

`search trial --phase PHASE3` works, as ClinicalTrials.gov spells it. `get drug NDA216340 label --source openfda-label` returns that application's label, as `approvals` already accepts the same ID.

## Why

Callers arrive with the spelling the source uses and the ID the last reply gave them. Both were refused.

## Proof

- Proof: the two commands above return data; both fail on the base commit with usage.

## Starts from

- Starts from: Seen on main be96d8d40 (1.0.0), Linux, 2026-11-19, in 25 agent runs that used the command line build on real tasks.
- `search trial --intervention tirzepatide --phase PHASE3` is refused; the valid values listed are `3`, `III`, `2/3` and others, while BioMCP's own trial rows print `phases: ["PHASE3"]`.
- `get drug NDA216340 approvals --source drugsfda` works. `get drug NDA216340 label --source openfda-label --label-section indications_and_usage` replies "expected a supported command with checked arguments" and suggests a search. The label search row carries `application_numbers`, so the link is held.
- PM decision, 2026-11-19: any value BioMCP prints in a reply is accepted back as input for the same field. Where one application has several labels, return the list to choose from.

## Keeps

- Keeps: the present spellings.

## Defers

- Defers: a sweep of other fields for values printed but not accepted; the developer lists what is found.

## Changes

- Changes: trial search phase parsing, drug label input resolution, help and cases.
