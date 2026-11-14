# 0947 — recall-json-keeps-real-line-breaks

Status: OPEN.

Milestone: 7

## Outcome

Device recall free text in replies reads as one line: each run of openFDA line breaks (LF, CR or a CR-LF pair) becomes a single space, so no literal `\u{000a}` text reaches a reply. The 1.0.0 `Reply_Text` contract stays unchanged.

## Why

dx-lab-director round 2 saw a literal `\u{000a}` 11 times in recall text, e.g. Z-0884-2026. Evidence: bench branch `ticket/0048-dx-r2-evals`, `personas/dx-lab-director/runs/2026-11-14-r2/` (pm job j40752, BioMCP 2d08cab05).

## Proof

- Proof: a recorded recall reply with line breaks shows the text joined by single spaces and no `\u{000a}` text.

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
- Why: every recall text field is `Reply_Text` in the 1.0.0 release-candidate baseline, whose pattern forbids `\x00-\x1f`. A JSON `\n` there breaks the published contract. Allowing it means a reply pattern widening, which `generated_schemas_preserve_release_contracts` accepts only with a `pattern_widenings` review in `release/schema-reviews.json` (none exist yet). Widening `Reply_Text`'s pattern would change the frozen 1.0.0 contract.
- PM decision 2026-11-14: option B. Keep the 1.0.0 contract and collapse line breaks to a single space in the device recall mapping. `Text::from_source` stays unchanged for other sources here.
- Changed: `fda_device_recall/map.rs` joins the non-empty trimmed lines of each free-text field (reason, root cause, product description, distribution pattern, initial firm notification, product quantity) with one space before admission. Code fields keep strict admission.
- Follow-up: the other adapters that admit text through `field_text` or `Text::from_source` still turn source line breaks into `\u{000a}`; they need the same decision source by source.
