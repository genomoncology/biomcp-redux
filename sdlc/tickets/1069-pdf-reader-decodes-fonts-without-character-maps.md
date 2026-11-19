# 1069 — pdf-reader-decodes-fonts-without-character-maps

Status: OPEN.

Milestone: 7

## Outcome

BioMCP's PDF reader returns correct text for FDA PDFs whose fonts it now reports as undecodable, starting with the seven label versions found on 2026-11-19 (one Olumiant and six Xeljanz).

## Why

Until these read, label change history answers "on or before" a date for those drugs, because the version just before the change cannot be read.

## Proof

- Proof: the seven named PDFs yield text that matches a reference reading on the label's section headings and on the searched sentences; they are reported unread on the base commit.

## Starts from

- Starts from: this is an investigation first, with a limit of one working session: find why these fonts decode wrongly (the first guess is fonts with no character map), say whether a bounded fix exists in the present reader, and report to the PM before building. If no bounded fix exists, say so and stop.
- The ticket that makes the reader flag garbled text lands first.

## Keeps

- Keeps: no new PDF library without the PM's ruling and the review that new dependencies need.

## Defers

- Defers: scanned labels with no text at all.

## Changes

- Changes: the PDF reader and its cases.
