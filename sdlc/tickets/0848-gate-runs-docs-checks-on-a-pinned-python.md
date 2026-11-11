# 0848 — gate-runs-docs-checks-on-a-pinned-python

Status: OPEN.

Milestone: 7

## Outcome

The Makefile's Python steps run on a pinned Python of 3.11 or newer on every machine, and the landing gate runs the first-read check (`scripts/check_front_map.py`, at least its examples part), so a wrong documented example fails before it lands.

## Why

0836's review (2026-11-11): since ticket 0395 `release/notices.py` needs `tomllib`, but the build machine's default Python is 3.9, so `make docs-check` fails there for every branch unless run with 3.13 on the path; and the landing gate never runs the docs check, so the examples' proof never runs at landing.

## Proof

- Proof: `make docs-check` passes on the build machine with no PATH change; the landing gate's output shows the first-read check running.

## Starts from

- Starts from: `Makefile`, `scripts/check_front_map.py`, `release/notices.py`.

## Keeps

- Keeps: the landing gate's time within a minute of today's.

## Defers

- Defers: nothing.

## Changes

- Changes: `Makefile`, `scripts/`
