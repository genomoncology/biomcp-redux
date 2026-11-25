# 1185 — configuration-guide-grant-search-and-size

Status: OPEN.

Milestone: 7

## Outcome

`docs/configuration.md` lists the `grant_search` article action and states the typed catalog size the first-read check measures on main (193,600 bytes at 2cfede31b), or states it in a way a check keeps current.

## Why

Found by the 1180 developer on 2026-11-25: the guide's article action list omits `grant_search` and its catalog size says 190,876 bytes.

## Proof

- Proof: The docs check or first-read check fails when the guide's action list or size differs from the code; it fails on main.

## Starts from

- Starts from: `docs/configuration.md`, `scripts/check_docs.py`, `scripts/check_front_map.py`.

## Keeps

- Keeps: Every other statement in the guide.

## Defers

- Defers: Nothing.

## Changes

- Changes: docs/configuration.md and the docs check.
