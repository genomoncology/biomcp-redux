# 1069 — pdf-reader-decodes-fonts-without-character-maps

The PDF reader reads the character maps lopdf refuses over their header. The fonts that read as gibberish do carry a character map (`/ToUnicode`); lopdf rejects the whole map because its PostScript header puts `def` inside `/CIDSystemInfo`, and then decodes each two-byte glyph ID as a Latin-1 byte.

Status: OPEN.

Milestone: 7

## Outcome

BioMCP's PDF reader returns correct text for FDA PDFs whose `Identity-H` or `Identity-V` fonts carry a character map lopdf refuses. On 2026-11-19 that was 145 maps in 16 documents reported garbled (six Xeljanz, one Olumiant and four Rinvoq labels, Lipitor 2012, Tagrisso 2020 and two more Xeljanz labels, and the Tagrisso S-21 letter), and stretches of gibberish in 50 documents the garbled check let through.

## Why

Until these read, label change history answers "on or before" a date for Olumiant, Xeljanz and Rinvoq, because the version just before the change cannot be read, and searches miss text in the mixed documents.

## Proof

- Proof: the reader's words agree with a reference reading (poppler's `pdftotext`) on all 16 documents; the repo asserts it on two small real fixtures (the Tagrisso S-21 letter and page 1 of the Olumiant 2020-07-08 label) together with their section headings and searched sentences. They are reported garbled on the base commit.
- Proof: a small synthetic PDF whose glyphs have no character map is still reported garbled, and a hostile refused map (a range over every four-byte code and thousands of sections) reads within the rebuild budget and is refused beyond it.

## Starts from

- Starts from: the 2026-11-19 investigation: every failing map parses once its code ranges and mappings are copied under a plain header.
- PM decisions, 2026-11-19:
  1. Method as proposed: keep lopdf's answer when it reads the map; only when it refuses the map of an Identity-H or Identity-V font, rebuild the map from its own code ranges and mappings under a plain header and use it only if it parses. No guessed offset, no new library. The garbled check still runs on the result.
  2. Proof: word-level agreement with a reference reading on all 16 documents, asserted in the repo on small fixtures (the Tagrisso letter and one trimmed Xeljanz or Olumiant label page), plus the searched sentences and headings. Record in the report the before and after for all 162 documents, including the 50 that gained text.
  3. Keep a real test for the garbled state with a small synthetic PDF, since no real document may now trigger it. State in docs/cautions.md what "garbled" still means and when it can arise.
  4. Find the document whose read time went from 0.2 s to 1.0 s and say why; make sure a hostile map (huge ranges, many sections) cannot make the reader slow or use much memory, since the hosted server reads PDFs fetched from the web.
  5. Words run together ("servicefood") are a separate, smaller fault: note it with examples in Defers; do not fix it here unless it is a few lines.
  6. The 50 documents with gibberish stretches that the garbled check missed mean the check is weaker than thought on mixed documents. After this fix, say whether any mixed case can remain, and if so whether the check should work per font or per page rather than per document.
- The ticket that makes the reader flag garbled text (1068) lands first.

## Keeps

- Keeps: no new PDF library without the PM's ruling and the review that new dependencies need.
- Keeps: documents that read correctly read the same, and reading stays fast; each font is read once per document.

## Defers

- Defers: scanned labels with no text at all.
- Defers: words run together where poppler puts a space. Old FDA letters set in one font read "servicefood", "administrationrockville" and "laboratoriesattention" (Humira 2007-09-05 letter, BLA 125057 supplements 89, 94 and 114), and one Livalo label reads "saf ely" and "ectively" (2019-05-16). The word assembler places these breaks; the character maps are not involved.

## Changes

- Changes: the PDF reader, its cases and fixtures, `scripts/pdf_font_fixtures.py`, and the PDF section of `docs/cautions.md`.
