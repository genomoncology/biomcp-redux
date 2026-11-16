# 1005 — drug-label-replies-give-the-label-effective-date

Status: COMPLETE.
Landed: c005565a61c727e2f9d15057ec3482a711f3f1ba

Milestone: 7

## Outcome

A `get drug … label` reply from openFDA's label source gives the label's own effective date, as openFDA's `effective_time` states it, next to its SPL set id and version, in both the default and the `--label-section` replies. An agent can tell a current label from an old one without guessing.

## Why

The msl persona on 3521c503a (bench ticket 0058, job j41885, records `0058-persona-msl-pair/biomcp-cli/calls.md` turns 38-60) read KRAZATI's label: SPL set 0b8bf078, version 14, indications listing NSCLC only. KRAZATI's colorectal indication was removed in 2026, so the label was right. But the reply named only the version, so the agent took it for a stale openFDA snapshot and reported a tool lag. Both blind graders called that claim wrong (bench `personas/_eval/grading-2026-11-16-p58/notes.md`). A date on the label would have shown it was current.

## Proof

- Proof: a replayed case for `get drug KRAZATI label` and `--label-section indications_and_usage` asserts the effective date from the recording. The case fails on the base commit.

## Starts from

- Starts from: the openFDA label adapter's record (`crates/biomcp-adapters/src/openfda_label/`), the drug label reply's identity, and openFDA's `effective_time` field.

## Keeps

- Keeps: label choice, sections and their text unchanged.

## Defers

- Defers: Drugs@FDA supplement purposes, which the same run could not read ("No complete supplement-purpose statement could be read from the linked FDA letter").

## Changes

- Changes: the label adapter and reply, the schema (one added field), a case and its recording, and one release-notes line.

## Progress

- Cause: the openFDA label adapter already read `effective_time` and put it in the card's `updated` and in a section's nested `label.revised`, but the reply's `identity`, where the SPL set ID and `version` sit, had no date, and the bare `get drug NAME label` reply (section names only) carried no date at all.
- Change: `DrugHeader` gains the optional `effective_date`, serialized right after `version`. The openFDA label header fills it from `effective_time`; every other drug source leaves it out. Because the header is shared, openFDA label search rows and cards also carry it, which keeps a label's identity the same in every reply (a several-sections reply compares identities). Label choice, sections, hints and text are unchanged. Thirteen drug reply schemas gain the one optional field; the 1.0 baseline allows added optional fields.
- Measured live in pm job j41933 (2026-11-16): KRAZATI matches one openFDA label, SPL set 0b8bf078-34c2-4f45-9012-38a8ac082b01, version 14, `effective_time` 20260831, so the label the msl persona read was current as of 2026-08-31. Both replies are kept whole in `tests/replies/drug/label-effective-date/`.
- Proof: `tests/cases/drug/label-effective-date.yaml` reads `get drug KRAZATI label` and `--label-section indications_and_usage` and expects `/result/identity/effective_date` 2026-08-31 beside id and version 14. Cheap job j41936: schemas regenerated, `make check-cheap` and all 84 drug cases pass; with base 41ac1325f's crates the new case fails at step 1 (`effective_date` missing). No drug case's `max_bytes` needed raising.
- Recording jobs j41926 and j41930 failed on recording setup (one recording directory reused, then a missing parent directory); j41933 recorded.
- Gate j41937 on 3b923e65d (contains origin/main 41ac1325f; main had not moved): `make --keep-going landing-gate docs-check schema-release-check` passed; all 1009 cases pass and the schema comparison accepts the added optional field.
