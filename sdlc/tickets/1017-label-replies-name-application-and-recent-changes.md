# 1017 — label-replies-name-application-and-recent-changes

Status: OPEN.

Milestone: 7

## Outcome

A `get drug … label` reply names the label's FDA application number(s) and, when present, shows its recent major changes.

## Why

pt-committee couldn't tell which of 11 ELIQUIS labels it read (1.2, 7.1); msl took KRAZATI's current label for a stale one (4.2) (bench `personas/pt-committee/runs/2026-11-16/analysis.md`, msl's). Follows 1005.

## Proof

- Proof: a replayed case asserting both; fails on the base commit.

## Starts from

- Starts from: the analysis named in Why, and the BioMCP code it cites.

## Keeps

- Keeps: every other reply field and command form.

## Defers

- Defers: nothing.

## Changes

- Changes: the named command's code, help, cases and a release-notes line.

## Progress

- Cause: the openFDA label adapter read `openfda.application_number` only to rank labels that share a name, and a label reply served `recent_major_changes` only when asked for by `--label-section`, so the identity named just the SPL set, version and date.
- Change: `DrugHeader` gains the optional `application_numbers`, right after `effective_date`, filled from openFDA's `openfda.application_number` on label headers (search rows and cards too, as 1005 did for the date); every other drug source leaves it out. `DrugLabel` and the several-sections `DrugLabelPart` gain the optional `recent_major_changes`: the label's own section text, in every label read (bare, one section, or a missing section), once per reply in the first label part, and not beside a read of `recent_major_changes` itself. An element that cannot be admitted is a problem, not a silent drop. The `label` section help names both. Drug reply and request schemas gain only optional fields or changed descriptions.
- Evidence reused, not re-recorded: 1005's live KRAZATI replies (job j41933, `tests/replies/drug/label-effective-date/`) already carry `application_number` NDA216340 and `recent_major_changes` "Indications and Usage, KRAS G12C Mutated Locally Advanced or Metastatic Colorectal Cancer - Accelerated Approval Indication Removed 08/2026".
- Proof: `tests/cases/drug/label-application-and-changes.yaml` replays them: the bare read and an indications read show NDA216340 and the change; a two-section read shows the change in part 0 only; a `recent_major_changes` read does not repeat it. Job j42015: schemas and docs regenerated, all 85 drug, mcp-parity and help cases pass; with base cf7fb8816's crates the new case fails at step 1 (`application_numbers` missing). Its `check-cheap` failed on a long `select` and a silent `.ok()` drop; j42019 fixed the drop and failed on `select` at 81 lines; j42021 on d326db595 passes `check-cheap` and all 85 drug cases. No `max_bytes` needed raising.
- Gate j42024 on 3c87ac374 (contains origin/main 3701a52f0): `make --keep-going landing-gate docs-check schema-release-check` passed; the schema comparison accepts the added optional fields.
