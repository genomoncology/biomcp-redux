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

## Progress

- 2026-11-14, live check: openFDA `device/recall.json?search=product_res_number:"Z-0884-2026"` sends real line breaks (JSON `\n`) in `product_description`, `code_info`, `action` and `additional_info_contact`. openFDA never sends the text `\u{000a}`.
- The literal comes from BioMCP: `mapped::field_text` admits recall text through `biodata::Text::from_source` (biodata rev 8bcd245, `crates/biodata/src/source_text.rs:301`), which rewrites every control character, `\n` included, as `\u{xxxx}`. `Text::new` refuses any control at all. The same path serves 23 adapter files, so recall is one case of a general rule.
- Blocked: every recall text field is `Reply_Text` in the 1.0.0 release-candidate baseline, whose pattern forbids `\x00-\x1f`. A JSON `\n` there breaks the published contract. Allowing it means a reply pattern widening, which `generated_schemas_preserve_release_contracts` accepts only with a `pattern_widenings` review in `release/schema-reviews.json` (none exist yet). Under the developer brief that change needs the PM's decision before any code.
