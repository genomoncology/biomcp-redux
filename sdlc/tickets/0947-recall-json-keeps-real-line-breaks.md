# 0947 — recall-json-keeps-real-line-breaks

Status: OPEN.

Milestone: 7

## Outcome

Device recall text in JSON replies carries openFDA's line breaks as real newlines (`\n` in JSON), not the literal text `\u{000a}`.

## Why

dx-lab-director round 2 saw a literal `\u{000a}` 11 times in recall text, e.g. Z-0884-2026. Evidence: bench branch `ticket/0048-dx-r2-evals`, `personas/dx-lab-director/runs/2026-11-14-r2/` (pm job j40752, BioMCP 2d08cab05).

## Proof

- Proof: a recorded recall reply with a line break shows `\n` in JSON output and no `\u{000a}` text.

## Starts from

- Starts from: the fda-device-recall adapter's text cleaning; search the codebase for `u{000a}` or escape_debug-like formatting.

## Keeps

- Keeps: other text fields' cleaning.

## Defers

- Defers: nothing.

## Changes

- Changes: the recall text mapping, a case.
