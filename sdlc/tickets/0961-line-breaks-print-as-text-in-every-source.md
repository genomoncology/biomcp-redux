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

## Progress

- 2026-11-15, scope: `Text::from_source` reached replies from 16 adapter files directly, from `mapped::field_text`, `mapped::source_text` and `Problems::text`, and from every `NativeText` field (about 40 more raw types), so the 23-file list undercounted.
- Evidence: a scan of the JSON source replies under `tests/replies` (2,299 files) found real line breaks in ctgov, nci, reactome, openfda-label, maude, fda-pma and fda-device-recall. ctgov descriptions and eligibility, nci descriptions, Reactome summations and label sections are `Markdown` fields, which already keep real line breaks; the openfda-label, maude and fda-pma ones are synthetic. Only device recall text reached a `Text` field. Live probes found none in OLS MONDO descriptions, Semantic Scholar abstracts, MyDisease definitions or 765 openFDA drug enforcement records from 2025. Reactome marks paragraphs with `<br><br>`, so its paragraphs survive in text too.
- Rule chosen: one shared rule. Every `Text` field forbids controls in the 1.0.0 `Reply_Text` pattern, so no `Text` field can keep a real line break without widening the frozen contract; prose where paragraphs matter is already `Markdown`. So every source text admission joins the non-empty trimmed lines with one space, exactly as 0947 did for recalls. Other controls keep their visible escapes, and strict admission (`Text::new`) still refuses them.
- Changed: `native_text::admit` holds the rule; `NativeText`, `mapped::field_text`, `mapped::source_text`, `Problems::text` and the 16 direct callers use it instead of `Text::from_source`. The recall mapping's own `one_line` copy is gone; its case is unchanged. biodata is unchanged.
- Proof: the case runner now refuses any CLI, MCP or named-tool reply holding the text `\u{000a}` or `\u{000d}`; pm job j41237 ran all 953 cases with it and only a draft Reactome case failed, because Reactome summations already keep real line breaks (dropped). `diagnostic/recall-line-breaks` passes through the shared helper. The `markdown-view/source-fields-are-data` trial, whose PubMed title holds line breaks, now expects spaces where it expected `\u{000a}`.
- Open: strict fields (`Text::new`, `field_code_text`) still refuse a source line break as a problem; the ticket keeps that refusal. NCI trial site records join several phones or emails with `\r`, but BioMCP does not map those site fields.
