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

## Progress

- Live check (job j41131, openFDA device/event): `search=event_type.exact:""` returns total 56,494 and `search=event_type.exact:"No answer provided"` returns 995, matching 0893's whole-database counts, so openFDA can select both groups.
- Change: the `event_type` doc in `crates/biomcp-types/src/adverse_event/query.rs` gains, on its second line, "Reports with an empty or "No answer provided" event type match no value and fall outside every event-type count." It reaches `biomcp help search adverse-event`, the `event_type` schema description and `docs/reference/commands.md` (regenerated in j41131). The typed MCP `adverse_event` tool description stays 396 characters and does not carry the sentence.
- Help case: `tests/cases/adverse-event/help.yaml` (the existing adverse-event help case, rather than a new file under `tests/cases/help/`) asserts the full `--event-type` help description; its byte cap moves 8,024 to 8,156, measured size plus 15%.
- Gate `make --keep-going landing-gate docs-check schema-release-check` passed in job j41139 at 70695e75d.
- 0893's Progress line now says the groups match no `--event-type` value and that openFDA can select them; its Defers line no longer points counts by event type at 0894.
