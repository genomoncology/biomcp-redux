# 1068 — pdf-reader-opens-form-objects-and-flags-garbled-text

Status: OPEN.

Milestone: 7

## Outcome

BioMCP's PDF reader returns the text of a PDF whose body sits in embedded form objects, and it reports a PDF whose text decodes to gibberish as unread, with that reason, instead of returning the gibberish as text.

## Why

The design work for label change history (ticket 1066) ran BioMCP's reader over 80 real FDA label PDFs on 2026-11-19. Seven lost their body text because the reader never opens form objects, and seven came out as gibberish because their fonts decode wrongly. A garbled document read as text makes a search answer "no match" when the truth is "could not read". The same reader reads FDA approval letters and device documents today, so the gap likely reaches those: it may be why some approval records come back with no supplement purpose and the reply is `partial`.

## Proof

- Proof: cases on a few small real FDA PDFs: one whose text is in form objects now yields its text; one with a font that decodes wrongly yields "unread: text could not be decoded", never text. Both fail on the base commit.
- Proof: the developer reports, from a run over the 80 label PDFs and a sample of at least 30 approval letters, how many documents gain text, how many are flagged garbled, and whether any document that read correctly before reads differently now.

## Starts from

- Starts from: the reader in `crates/` that reads FDA letters and device documents with `lopdf`; its `words.rs` is at 296 of 300 allowed lines and must be split first. The reader comparison numbers are in `sdlc/planning/label-section-change-history.md` on branch `ticket/1066-label-section-change-history`.
- PM decision, 2026-11-19: fix BioMCP's own reader; add no new PDF library. A second library recovered the same seven documents and none of the garbled ones.
- PM ruling, 2026-11-19: keep strict loading. A damaged PDF is refused, and the refusal must say the file is damaged (not too large, not garbled), so a caller knows to fetch it another way.
- PM decision, 2026-11-19: the garbled check must be conservative in one direction: it may let odd text through, but it must never mark readable English text as garbled. Say how it decides and show its result on all 80 labels.

## Keeps

- Keeps: every document that reads correctly today reads the same. Reading stays well inside the per-call time limit.

## Defers

- Defers: decoding the garbled fonts properly, which has its own ticket.

## Changes

- Changes: the PDF reader, its cases, and any letter or device case whose recorded result improves.
