# 1074 — hosted-server-caps-pdfs-read-at-once

Status: OPEN.

Milestone: 7

## Outcome

The hosted server reads at most a set number of PDFs at the same time, so many callers each asking for a large PDF cannot exhaust its memory. A caller past the cap waits briefly or is told to retry; no caller gets a wrong or empty answer.

## Why

Ticket 1068 bounds what one PDF can cost: measured on 2026-11-19, about 0.3 seconds and 170 MB at worst for a hostile document, and 160 MB for the largest real FDA label read. That bound cannot cheaply go lower, because the PDF library loads the whole document. So total memory is the per-document bound times the number read at once, and nothing bounds that number. The hosted server reads PDFs fetched from the web on behalf of anyone who calls it.

## Proof

- Proof: a test starts more PDF reads than the cap at once and shows that no more than the cap run together, that the rest finish or are refused with a retry message, and that peak memory stays under cap times the per-document bound. It fails on the base commit.

## Starts from

- Starts from: the PDF reader's limits as ticket 1068 leaves them; the hosted server's request handling and its existing quota rules.
- PM decision, 2026-11-19: the cap is configurable with a default chosen from the server's memory; the command line on a user's own machine needs no cap beyond one read per call.

## Keeps

- Keeps: a single PDF read behaves as today. Replies do not change shape.

## Defers

- Defers: a PDF reader that loads objects only as pages need them.

## Changes

- Changes: the hosted server's PDF path, its config and docs.
