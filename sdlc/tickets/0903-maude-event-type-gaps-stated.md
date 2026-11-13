# 0903 — maude-event-type-gaps-stated

Status: OPEN.

Milestone: 7

## Outcome

A drug-safety scientist who counts device reports by event type learns from BioMCP's own help that some reports have no event type and fall outside every `--event-type` count. The 0893 ticket's record is accurate.

## Why

The independent review of 0893 (MAUDE event-type filter, 2026-11-13) found three gaps.
- The warning that reports with an empty or "No answer provided" event type match no value is only in `docs/cautions.md:98`, which no reply or help links to. The whole database has 56,494 empty and 995 "No answer provided". Recent years have almost none, but there were 11,335 in 2013 and 8,890 in 2014.
- The 0893 Progress note says these reports "are not selectable". openFDA can select them with `event_type.exact:""`; only BioMCP can't.
- 0893's Defers line points counts by event type at 0894, which is counts by received date.

## Proof

- Proof:
  - `biomcp help search adverse-event` (and the schema description of `event_type`) says reports with an empty or "No answer provided" event type match no value;
  - a help case asserts the sentence;
  - the MCP `search_adverse_event` tool stays under its 2,000-character limit.

## Starts from

- Starts from: `crates/biomcp-types/src/adverse_event/query.rs:71`, the `event_type` doc. Put the sentence on its second line, which reaches CLI help and the schema but not the MCP tool's first-sentence summary.

## Keeps

- Keeps: the four event-type values; the MCP tool size.

## Defers

- Defers: counts by event type, which should include the empty and "No answer provided" groups so the split adds up to the total. It waits for 0894's count reply to land, then becomes its own ticket.

## Changes

- Changes:
  - `crates/biomcp-types/src/adverse_event/query.rs`;
  - the generated schemas and `docs/reference/commands.md`;
  - `tests/cases/help/`;
  - a correction to `sdlc/tickets/0893-maude-event-type-filter.md`'s Progress and Defers lines.
