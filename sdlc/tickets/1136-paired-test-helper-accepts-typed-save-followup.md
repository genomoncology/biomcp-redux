# 1136 — paired-test-helper-accepts-typed-save-followup

Status: OPEN.

Milestone: 7

## Outcome

The paired CLI and MCP test helper compares a typed-mode reply whose follow-ups include `save` without failing first on schema validation, because it checks each reply against its own surface's schema.

## Why

Found by the developer of ticket 1132 (2026-11-22): typed-mode `biomcp` can return `save` as a valid follow-up, but the paired helper validates that reply against the CLI schema, whose list of follow-up kinds lacks `save`, so a paired test can fail before it compares rows.

## Proof

- A paired case with a typed reply carrying a `save` follow-up fails before the change and passes after, with its row comparison still run.

## Starts from

- Starts from: main.

## Keeps

- Keeps: CLI schema unchanged.

## Defers

- Defers: nothing.

## Changes

- Changes: the paired test helper under `crates/biomcp/tests/runner/`.
