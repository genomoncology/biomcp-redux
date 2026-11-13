# 0893 — maude-event-type-filter

Status: OPEN.

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
