# 1180 — docs-examples-and-release-notes

Status: OPEN.

Milestone: 7

## Outcome

Every tool example in `docs/configuration.md` runs as written on current main, and the release notes no longer say trial `--offset` needs `--next-page`.

## Why

The 2026-11-24 adversarial review ran the `disease` example at `docs/configuration.md:41` and got `usage.retired`, and found the release notes claim about trial `--offset`, which the code does not require.

## Proof

- Proof: A check (existing docs-example test or a new case) runs each tool example in docs/configuration.md and fails on main.

## Starts from

- Starts from: `docs/configuration.md`, the release notes in `CHANGELOG.md` or `docs/`.

## Keeps

- Keeps: The documented behaviour of each example; only the call shape changes.

## Defers

- Defers: Nothing.

## Changes

- Changes: docs/configuration.md, release notes, the docs-example check.
