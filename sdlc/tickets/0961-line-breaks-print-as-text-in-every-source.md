# 0961 — line-breaks-print-as-text-in-every-source

Status: OPEN.

Milestone: 7

## Outcome

Source text with line breaks reads as text in every reply: no adapter prints a source line break as the literal `\u{000a}`. Each adapter that admits text through `mapped::field_text` or `biodata::Text::from_source` either keeps a real line break or collapses it to a space, chosen source by source as 0947 did for device recalls, within the 1.0.0 schema.

## Why

0947 (2026-11-14): `Text::from_source` rewrites every control character as `\u{xxxx}`, and the same path serves 23 adapter files, so the recall fix was one case of a general rule; agents read the escapes as noise or quote them.

## Proof

- Proof: a case per changed source (or one shared case where the mapping is shared) with a recorded reply that holds a line break, asserting no `\u{000a}` in the reply; a grep or ratchet over `tests/cases` replies showing no remaining escaped line breaks from source text.

## Starts from

- Starts from: 0947's ticket and its device recall mapping; `mapped::field_text`; biodata `crates/biodata/src/source_text.rs:301` (`Text::from_source`); `grep -rln 'field_text\|Text::from_source' crates/`.

## Keeps

- Keeps: the 1.0.0 reply schema; 0947's recall behavior; refusal of other control characters.

## Defers

- Defers: nothing.

## Changes

- Changes: the adapters' text mapping, possibly one shared helper, their cases.
