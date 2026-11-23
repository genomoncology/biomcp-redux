# 1149 — label-sections-accept-numbers

Status: OPEN.

Milestone: 7

## Outcome

A label read by section number (such as `1` or `14`) returns that section, or refuses and names the matching section name, instead of an empty complete reply.

## Why

Found 2026-11-22 by scouts exploring BioMCP for multi-entity questions (about 760 CLI calls on build b7def4651). `biomcp get drug NAME label --source openfda-label --label-section 1,14` returned `complete` with empty parts for every drug tried, though the labels have nonempty Indications and Clinical Studies sections (`indications_and_usage`, `clinical_studies`). An empty complete reply reads as "the label has no such section".

## Changes

- Map standard prescribing-information section numbers to their openFDA field names, or refuse unknown section values with the list of valid names.
- Never report `complete` for a section the request named but the reader could not interpret.

## Proof

- `--label-section 1,14` on a replayed label returns the two sections (or a refusal naming `indications_and_usage,clinical_studies`); a truly missing section is reported as missing.


## Amendment, 2026-11-22

A round 2 scout found the same empty reply for a numbered subsection: the targeted `14.3` selector on Bizengri's label returned nothing although the full label contains section 14.3. Subsection numbers are in scope.
