# 1079 — pdf-symbol-font-characters-read-as-standard-characters

Status: COMPLETE.
Landed: b46c4b6d5c654de37e07c74b467477aef5c4236f

Milestone: 7

## Outcome

Text read from an FDA PDF holds the standard character for each Symbol-font glyph (arrows, ≥, ≤, Greek letters, %, digits, bullets), not a private-use code.

## Why

The ticket 1069 review (2026-11-20, 52 FDA PDFs fetched from accessdata.fda.gov plus the repo's fixtures) found 957 private-use characters in 3.2 million, in 19 documents. 618 are Symbol-font bullets. About 340, in the Lipitor labels and one Tagrisso label, are arrows, ≥, ≤, Greek letters, % and 86 digits: "↑ 8.7 fold" comes out as private-use codes. They come from the fonts' own maps, and the reference reader `pdftotext` gives the same codes. A caller searching a label for a number or a sign misses these, and an agent cannot read them.

## Proof

- Proof: on the Lipitor and Tagrisso documents the review names, the reply holds no private-use character where the font is Symbol and the code has a standard character, and the digits and signs match the page.
- Proof: a private-use character that has no known standard character is kept, never dropped, and the read says so.

## Starts from

- Starts from: the review's outputs and its count by document; the Adobe Symbol encoding table.
- PM decision, 2026-11-20: never drop these characters; dropping deletes ≥ and digits.

## Keeps

- Keeps: documents without Symbol-font text read byte-for-byte the same. The limits from tickets 1068 and 1075 hold.

## Defers

- Defers: other symbolic fonts (Wingdings, ZapfDingbats) unless the same table approach covers them at no risk.

## Changes

- Changes: the PDF reader's text step, its tests and fixtures.
