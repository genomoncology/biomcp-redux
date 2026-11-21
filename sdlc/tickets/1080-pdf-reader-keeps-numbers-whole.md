# 1080 — pdf-reader-keeps-numbers-whole

Status: COMPLETE.
Landed: e1857a08073907fc361af416a2fe5f9249e272ac

Milestone: 7

## Outcome

Numbers read from an FDA PDF come out as they stand on the page: a footnote superscript is not joined onto the number before it, and a number is not split at its decimal point or inside a phone number.

## Why

The ticket 1069 review (2026-11-20) compared numbers with the reference reader `pdftotext`: 83 of 20,890 differ in the 16 formerly garbled documents and 178 of 33,835 in the others. Most differ only in spacing. Some read wrong: "Week 262" and "Week 244" where a footnote superscript joined a week number (this also happens on main in a Rinvoq label), Lipitor 2009 "p=0.0005" split into "0 0005" and "p= .", and "(301) 796- 5073". Words also run together about once per 10,000 ("servicefood"). These come from how words are assembled from glyph positions, not from character maps. A wrong number in a drug label matters more than a wrong word, and a phrase search over label versions misses a split or joined token.

## Proof

- Proof: on the documents the review names, the listed numbers read as on the page, and number agreement with `pdftotext` rises; the ticket reports the before and after counts.
- Proof: no document that read correctly loses a correct token.

## Starts from

- Starts from: the review's outputs; the reader's word assembly step (superscript detection by baseline and font size; gap thresholds).

## Keeps

- Keeps: the limits from tickets 1068 and 1075. Where the reader cannot tell, it keeps the glyphs apart with a space rather than joining them.

## Defers

- Defers: paragraph order.

## Changes

- Changes: the PDF reader's word assembly, its tests and fixtures.
