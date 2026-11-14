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

## Progress

- 2026-11-14 probe from the M5 (pm jobs j40674 and j40677, curl with user agent `ureq/3.5.0`, each Accept value with and without `Accept-Encoding: gzip`): P150047B.pdf, the Drugs@FDA letter 021588Orig1s056,s057ltr.pdf, the ClinicalTrials.gov large-docs SAP_001.pdf and arXiv 1706.03762 all answered 200 `application/pdf` to `application/octet-stream`, `application/pdf`, `*/*` and `application/pdf, */*;q=0.1`. Europe PMC's fulltextRepo PDF answered 403 JSON to all four, and bioRxiv answered 302 to all four, so neither depends on Accept. FDA's 406 is therefore not steady: j40668 saw 406 for octet-stream and 200 for `application/pdf` and `*/*` within one run, and j40674/j40677 saw 200 for all an hour later. `application/pdf` is the value that worked in FDA's 406 window and every host accepts it.
- Changed: PDF downloads (FDA PMA, Drugs@FDA and publisher article PDFs through `pdf::download`, and ClinicalTrials.gov documents) now send `Accept: application/pdf` through a new `DocumentMedia::Pdf`. Other byte reads keep `application/octet-stream`. Recording keys don't include Accept (j40689 recorded P150047 under the same key j40615 used with octet-stream), so existing recordings stay valid; a cached entry stored as octet-stream is simply a miss.
- Changed: the FDA refusal notice no longer says "intermittent, retry later". It names the URL, the HTTP status and its meaning (e.g. "HTTP 406 Not Acceptable"), says FDA has refused PDFs that open in a browser, so it does not show the document is missing, and says to open the URL.
- Proof: pm job j40689 ran `get diagnostic P150047 summary --source fda-pma` live three times from the M5; each returned text (53 pages). Unit tests `pdf_downloads_ask_for_application_pdf` (adapter) and `pdf_document_media_asks_for_a_pdf` (wire header) pass (j40680). `tests/cases/diagnostic/fda-document-refused` has a new P150047 step replaying a synthetic 406 body over the live-recorded request key, and the 404 steps carry the new message.
