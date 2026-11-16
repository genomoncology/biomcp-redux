# 1005 — drug-label-replies-give-the-label-effective-date

Status: OPEN.

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
