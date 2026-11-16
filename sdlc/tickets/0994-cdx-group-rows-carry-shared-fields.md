# 0994 — cdx-group-rows-carry-shared-fields

Status: OPEN.

Milestone: 7

## Outcome

An agent reading a companion-diagnostic group row sees the device names and other shared fields on the row itself, without having to read a help line that explains where they are.

## Why

In dx-lab-director round 4 (bench, `personas/dx-lab-director/runs/2026-11-15-r4/analysis.md`), Q4 named 5 of 7 TAGRISSO devices. FDA's group-table rows come back null when keyed on the individual table's header. 0959 added help that names the shared fields, and that help was in the agent's call 6 reply, but the agent didn't use it. The streamlining note's anti-pattern 7 applies: no wording fixes a reply shape that puts the wrong thing in front of the agent.

## Proof

- Proof: first, in Progress, show from the recorded round-4 replies which fields are null on the group rows and where the shared values sit.
- Proof: a routine case over a recorded TAGRISSO search shows all seven devices named on its rows; it fails on the base commit.
- Proof: any reply schema change follows the 1.0 rules (an optional field, or a reviewed entry in `release/schema-reviews.json`), and the notes say what changed.

## Starts from

- Starts from: 0933 (dated CDx rows and one row shape), 0959 and 0962 (the shared-field help), and the fda-cdx adapter.

## Keeps

- Keeps: 1.0 schema rules; reply size within the existing caps.

## Defers

- Defers: nothing.

## Changes

- Changes: the fda-cdx rows, their case, possibly the help line 0959 added, the notes.
