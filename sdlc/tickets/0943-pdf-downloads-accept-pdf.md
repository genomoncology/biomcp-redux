# 0943 — pdf-downloads-accept-pdf

Status: OPEN.

Milestone: 7

## Outcome

BioMCP's PDF downloads (FDA PMA safety summaries, Drugs@FDA documents, article PDFs) ask for a PDF in a way FDA's servers accept from every machine we run on, so `get diagnostic P150047 summary` works live from the M5. When a source still refuses, the message says what was refused and why, not "intermittent, retry later" unless that is what the source said.

## Why

0934's review (2026-11-14) found a production bug: BioMCP sends `Accept: application/octet-stream` for PDF downloads (`crates/biomcp-fetch/src/read.rs:240`, via `Read::bytes()`), with the HTTP library's default user agent `ureq/3.5.0`. From the M5, FDA answers 406 to that Accept header for P150047B.pdf (jobs j40615, j40668), and 200 to `application/pdf` or `*/*`, even with no browser user agent. So live safety-summary reads fail there today, and the 0868 notice in `crates/biomcp-adapters/src/pdf/text.rs` tells the user it is intermittent and to retry. The persona rounds run BioMCP on the M5, so the dx-lab-director's round 2 depends on this.

## Proof

- Proof: a job on the M5 runs `get diagnostic P150047 summary --source fda-pma` live and gets text; a unit test asserts the PDF download's Accept header; a recorded 406 case shows the corrected message.

## Starts from

- Starts from: `crates/biomcp-adapters/src/pdf/download.rs` (`from_source`), `crates/biomcp-fetch/src/read.rs:240`, `DocumentMedia` (no PDF variant yet), the 0868 notice in `pdf/text.rs`. Check live from the M5 which Accept values FDA, Drugs@FDA and the article PDF hosts accept before choosing one.

## Keeps

- Keeps: cached recordings and their keys (if the request key includes headers, say how old recordings stay valid); the 1.0.0 schema rules.

## Defers

- Defers: a BioMCP user agent string (decide separately if any source needs one).

## Changes

- Changes: the PDF download request, its error message, tests.
