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
