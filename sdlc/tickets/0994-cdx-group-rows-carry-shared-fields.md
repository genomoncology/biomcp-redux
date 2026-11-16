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

## Progress

- 2026-11-16, evidence (round 4 call records, j41109, BioMCP 95a74e246): the agent read FDA's per-table cell headers, never BioMCP's shared fields. Call 46 (`--drug-application NDA208065`, 15 rows) projected `.cells["Diagnostic Name (Manufacturer)"]`, which is null on the 5 group rows, because the group table spells that header `"Diagnostic Name\n(Manufacturer)"` and its other headers differently (`"Indication(s) - Sample Type"`, `"PMA (Approval Date)"`, and a drug-group cell instead of `"Drug Trade Name (Generic) NDA / BLA"`). Call 47 opened one group row: it printed `kind`, `snapshot`, `index`, then `cells`, so the shared `diagnostic`, `indication`, `device`, `drug` and `row_date` sat after a 2 KB drug-group cell and the agent stopped reading at `head -50`. Calls 37-41 hit the same nulls over all 244 rows (call 41 failed on them).
- Already fixed on main by 0966 (landed after round 4, before this ticket was filed): rows now print `kind`, then `row_date`, `diagnostic`, `indication`, `device`, `drug`, then FDA's cells, and `fda-companion-shared-first` checks all 15 TAGRISSO rows carry those fields.
- Gap left, measured on the recorded FDA page (`tests/replies/diagnostic/cdx-group-join`): the shared `diagnostic` copies FDA's cell text, and the group table breaks names across lines (`"cobas EGFR Mutation Test v2\n(Roche Molecular Systems, Inc.)"`) while the individual table also pads with no-break spaces. TAGRISSO's 15 rows spell its 7 tests 10 ways; over all 244 rows, 88 spellings for 83 names. A script grouping on `.diagnostic` counts cobas, FoundationOne CDx and FoundationOne Liquid CDx twice.
- Decision: `diagnostic` and `indication` read as one line (whitespace runs, line breaks and no-break spaces become one space). FDA's `cells` keep their original text; `device` (number and date) and `drug` (the group cell is a list, one drug per line) are unchanged. No field added or removed: only the two fields' descriptions change, which the 1.0 rules allow. Reply size can only shrink. Release notes say it.
- Proof: `fda-companion-shared-first` now expects TAGRISSO's 15 `diagnostic` values exactly, naming the 7 tests (Guardant360 Liquid CDx, cobas, FoundationOne CDx, FoundationOne Liquid CDx, Guardant360 CDx, ONCO/Reveal, MI Cancer Seek) in one spelling each; on main the group rows' line breaks fail it.
