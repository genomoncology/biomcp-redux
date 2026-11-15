# 0968 — fda-summary-text-keeps-whole-words

Status: OPEN.

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
