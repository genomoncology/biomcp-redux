# 1114 — name-the-source-and-section-limits-in-gene-get-refusals

Status: OPEN.

Milestone: 7

## Outcome

A gene get refusal names the rejected flag and teaches the working section spelling for ClinGen, GenCC or gnomAD constraint.

## Why

Found by a hand solve of research tasks, 2026-11-20. `biomcp get gene FANCA validity --source clingen` returned `unknown option: this command has no error.option`. `biomcp get gene FANCA validity --source gencc` and `biomcp get gene ATM constraint --source gnomad` give the same message even though their supported sections work without `--source`.

## Proof

- Proof: Replay the three calls and check the message names `--source` and the suggested call names the correct section, then run the suggestions on saved replies.

## Starts from

- Starts from: Gene get section parsing and the ClinGen, GenCC and constraint sections.

## Keeps

- Keeps: Gene get continues to accept named sections.

## Defers

- Defers: Constraint release selection and projection have separate tickets.

## Changes

- Changes: Gene get validation, guidance and cases.
