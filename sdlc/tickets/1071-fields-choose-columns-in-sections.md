# 1071 — fields-choose-columns-in-sections

Status: OPEN.

Milestone: 7

## Outcome

`--fields` works on the rows of a record's section, as it already does on search rows, so a caller can ask for only the columns it needs: for example a trial's locations with facility, city, status and distance only.

## Why

The caller is usually an agent that pays per token for everything a reply holds, and it re-reads each reply on every later turn. Measurements for ticket 1061 on 2026-11-19: a 100-row trial search falls from 61,265 bytes to 20,093 with three fields, and trial location rows would about halve. A trial with 776 sites costs about 215 KB in full. Choosing columns saves more than a higher page limit does.

## Proof

- Proof: `get trial NCT04613596 locations --fields facility,city,state,status,distance --limit 100` returns only those columns and is under half the bytes of the same page without `--fields`; and the same for one drug section and one pathway section. Each fails on the base commit, where `--fields` is refused for sections.
- Proof: a `--fields` name the section does not have is refused with the section's field names.

## Starts from

- Starts from: the search-row `--fields` handling and its "always kept" fields; ticket 1061's proposal in `sdlc/planning/1061-lists-page-one-way.md`, which has the measured sizes.
- PM decision, 2026-11-19: identifiers and anything a `next` step needs are always kept, as on search rows. The reply says which fields were asked for.
- PM rulings on the proposal (`sdlc/planning/1071-fields-choose-columns-in-sections.md`), 2026-11-19:
  1. Paths: dotted paths into objects, for sections and for searches, spelled as the reply spells them.
  2. Always kept: required and identifier keys, and for a path the required keys of each object it passes through. Next-call links are built before narrowing and carry `--fields`.
  3. Echo: an optional `fields` list on the reply; a reply without `--fields` is unchanged.
  4. Several sections in one call: a section that has none of the chosen names stays whole. A name no requested section has is refused. The refusal's `valid_values` holds only the requested sections' field names.
  5. Refusal wording as proposed, following the fixed-message design from ticket 1064.
  6. Schema: only the new optional `fields`; narrowed replies validate against today's schemas, and a general test proves it for every command switched on.
  7. Roll out trial, drug and pathway first, all in this ticket if the mechanism is schema-driven; a command that needs special handling is listed and left for a follow-up.
  8. Help: one short line per command saying `--fields` narrows rows and takes paths, within each MCP tool description's limit, and one worked example in `help examples` using a section agents use, such as trial locations with distance.
  Not in this ticket: required per-row attribution, the opentargets regulatory context, text outside the rows, GTR from the build machine.

## Keeps

- Keeps: a reply without `--fields` is unchanged. Each MCP tool description stays under its 3,500-character limit.

## Defers

- Defers: a default shorter row for agents; decide after measuring agent runs with this in place.

## Changes

- Changes: section replies for trial, drug and pathway first, then the rest; help, schemas and cases.
