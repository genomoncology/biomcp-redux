# 0968 — fda-summary-text-keeps-whole-words

Status: COMPLETE.
Landed: be8c247b9b9fb420941c012aa24c3469fe184b3a

Milestone: 7

## Outcome

Text BioMCP extracts from FDA summary PDFs keeps words whole (no "contai ned", "va lidated", "mutati ons"), so `--find` matches the words a reader searches for.

## Why

dx-lab-director round 4 (2026-11-15): the P120019 summary read back with words split by stray spaces (calls 80, 83), while `pdftotext` on the same file gives whole words; a `--find` for a split word misses. Round 3's agent had also noticed extra spaces. Evidence: bench `personas/dx-lab-director/runs/2026-11-15-r4/` (job j41109, BioMCP 95a74e246, bench 0051).

## Proof

- Proof: a case on a recorded FDA summary PDF (P120019 or P150047, already recorded) whose extracted text holds the whole words above and whose `--find` for one of them returns its passage.

## Starts from

- Starts from: the PDF text extraction used by `get diagnostic ... summary` and `pdf::download` callers; 0943 and the P150047 recording.

## Keeps

- Keeps: page and fragment numbering; the `--find` behavior; text caps.

## Defers

- Defers: other PDF sources unless the same extractor serves them, in which case they benefit too.

## Changes

- Changes: the PDF text extraction, cases.

## Progress

- Root cause, measured on the recorded P150047 summary (the P120019 summary is not recorded; it was made the same way): lopdf 0.47's `extract_text` adds a space after every `TJ` array (`parser_aux.rs`, `collect_text`) and a line break at every `ET`. Word-made FDA summaries break one word across several text operations whenever the character spacing changes (`(P)Tj 0.56 0 Td [(ag)6(e )]TJ`), and draw table-cell text in separate text objects, so lopdf split words inside a line ("plasmid sta ndard", "teste d", "cp/m L") and across text objects ("Mutatio\nns"). Word writes real word spaces as space glyphs, so the added space also doubled them. pdftotext 26.04, which places breaks by glyph position, keeps the words whole. On main (j41159), BioMCP's text of P150047 had 156 words split within a line that pdftotext has whole, and 3,145 doubled spaces.
- Changed: `pdf/words.rs` (with `pdf/fonts.rs` for glyph widths and `pdf/state.rs` for the matrices) reads each page's content with lopdf's own decoding and fonts, tracks the text and graphics matrices and glyph widths (`Widths`/`FirstChar` for simple fonts, `W`/`DW` for composite fonts), and adds a space between operations only where the next glyph starts more than a tenth of the font size (pdftotext's word gap, and lopdf's own `TJ` kerning rule) or a line away from where the last ended. lopdf's line breaks at `T*`, `'` and `"` stay; its `ET` break stays unless the next text object runs on in the same word on the same baseline. A font without widths (Type3, or no `Widths`) keeps lopdf's behaviour. Page count, page and text caps, the deadline checks, fragment size and `--find` are unchanged; the text is shorter, so P150047 now reads in 20 fragments instead of 22.
- Measured with the fix (j41165): P150047 and P170019 each have no word split within or across lines that pdftotext keeps whole, and no word longer than six letters that pdftotext lacks (so no words glued together); doubled spaces fell to 358 and 306, which are FDA's own space glyphs. Table cells keep their line breaks, and trade names now read "Tarceva® (erlotinib)" instead of "Tarceva \n®\n \n(erlotinib)".
- Cases: `pma-summary-find.yaml` gains a find for "plasmid standard curve", which returns the Table 15 passage in fragment 8 (it found nothing before, because the text read "sta ndard"); its LoD passage and `fda-documents.yaml`'s P170019 first fragment are re-taken from the fixed extraction, and P150047's fragment total is 20.
- The adapter value ratchet refuses `.ok()` drops, so font dictionaries are read by plain lookups: a missing key, a value that is not a number, or a reference to an object the file lacks (null by PDF 32000-1 §7.3.10) means the font has no widths and keeps lopdf's breaks. Cases and pdftotext comparison unchanged after that (j41176).
- Gate j41183 passed on c5aeebb5b (branch merged with origin/main 2026-11-15). rustfmt ran in j41176; schema and docs regeneration there left no changes.
