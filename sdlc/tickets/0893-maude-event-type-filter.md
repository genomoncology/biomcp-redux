# 0893 — maude-event-type-filter

Status: COMPLETE.
Landed: 484d0a7798a59bbd4229e2274026034b2df5271f

Milestone: 7

## Outcome

A MAUDE search can be limited to an event type (Malfunction, Injury, Death, Other), as openFDA's `event_type` allows, so "how many malfunction reports" is one call.

## Why

In the pv-scientist persona run of 2026-11-13 (biomed-agent-bench `personas/pv-scientist/runs/2026-11-13/log.md` and `report.md`, analysis beside them; code-mode run in `runs/2026-11-13-raw/`), `--event-type` was refused and `--problem malfunction` returned 0 because `product_problems` is a coded vocabulary (log.md:125-146); the agent reported 344,695 reports of every type. The code-mode agent used openFDA's `event_type` and got 76,605 malfunctions with the split by type (raw report Q4).

## Proof

- Proof: a recorded case where `search adverse-event "infusion pump" --source maude --event-type malfunction` with a date window returns the total from the recorded openFDA reply and echoes `event_type` in `source_query`; an unknown event type is refused with the valid values.

## Starts from

- Starts from: `crates/biomcp-adapters/src/maude/`.

## Keeps

- Keeps: `--problem` on `product_problems`.

## Defers

- Defers: counts by event type in one reply (0894).

## Changes

- Changes: `crates/biomcp-adapters/src/maude/`, `crates/biomcp-types/`, `tests/cases/adverse_event/`, `docs/`

## Progress

- Live check (job j39358, openFDA device/event, dataset 2026-11-04): `count=event_type.exact` lists Malfunction 16,796,984, Injury 9,656,791, Death 232,397, Other 105,886, empty 56,494 and `No answer provided` 995. `count=event_type` without `.exact` fails with a server error. For `device.generic_name:"infusion pump"` received 2025-11-01..2026-11-01 the split is Malfunction 295,358, Injury 49,029, Death 212, Other 96, which sums to the 344,695 the persona saw. `event_type:"malfunction"` matches any case; `event_type.exact:"Malfunction"` matches only the published case and `event_type.exact:"malfunction"` finds nothing; a prefix (`"Malf"`) finds nothing.
- Change: `--event-type malfunction|injury|death|other` (any letter case) on MAUDE search adds `event_type.exact:"Malfunction"` (published spelling) to the native query and the reply's `source_query`. It counts as a MAUDE filter: refused on other sources, and enough on its own like `--problem`. An unknown value is refused with the four valid values. `--problem` is unchanged.
- Empty and `No answer provided` event types are not selectable; they stay out of every `--event-type` count.
- Recorded case `tests/cases/adverse-event/maude-event-type.yaml` (job j39399): `search adverse-event "infusion pump" --source maude --event-type malfunction --received-from 2025-11-01 --received-to 2026-11-01` returns total 295,358 with `event_type.exact:"Malfunction"` in `source_query`; `--event-type broken` is refused with the four values; `--event-type` on FAERS is refused as MAUDE-only. Schemas and `docs/reference/commands.md` regenerated in that job; `docs/cautions.md` says how `--event-type` differs from `--problem`.
- The persona's code-mode 76,605 used `device.openfda.device_name:"Pump, Infusion"` and a different window, so it does not match the `device.generic_name` count; both are openFDA's own totals for their queries.
- The new field pushed the `search_adverse_event` MCP tool to 2,112 characters, over its 2,000 budget. `--expand-drug-names` and `--name-snapshot` now have short titles ("FAERS no-op.", "FAERS name snapshot.") ahead of their unchanged full text, which brings the tool back under budget. Gate `make --keep-going landing-gate docs-check schema-release-check` passed in job j39482 at 721e70e61.
