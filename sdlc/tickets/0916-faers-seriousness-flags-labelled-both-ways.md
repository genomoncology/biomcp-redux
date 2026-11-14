# 0916 — faers-seriousness-flags-labelled-both-ways

Status: OPEN.

Milestone: 7

## Outcome

Every FAERS seriousness flag on a card carries a label, whether openFDA sends "1" or "2", so an importer never needs openFDA's 1/2 rule.

## Why

The pharmacovigilance persona's round 2 (bench `personas/pv-scientist/runs/2026-11-13-r2/`, log.md:80, report Q7) scanned 25 fatal cards. Flags set to "1" carry a label, such as `death_label`. Flags set to "2" keep the bare code with no label, as in report 27316597's `lifethreatening:"2"`. The persona wrote that "a "2" flag needs the importer to know 1/2 semantics (undocumented in the reply)".

## Proof

- Proof: a recorded FAERS card case where a "2" flag carries a label saying the report did not record that outcome, and a "1" flag keeps today's label.

## Starts from

- Starts from: 0896's FAERS code labels.

## Keeps

- Keeps: the raw codes beside the labels.

## Defers

- Defers: nothing.

## Changes

- Changes: the FAERS card mapping, `tests/cases/adverse-event/`, regenerated schema descriptions if a field description changes.
