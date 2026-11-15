# 0965 — article-pdfs-from-europepmc-and-biorxiv

Status: OPEN.

Milestone: 7

## Outcome

Asking BioMCP for an article PDF from Europe PMC or bioRxiv either returns the PDF or says plainly why not and where to read it, instead of a bare failure.

## Why

0943 probe (2026-11-14, pm jobs j40674 and j40677): Europe PMC's fulltextRepo PDF answered 403 JSON and bioRxiv answered 302 to every Accept value, so neither depends on Accept and 0943's fix doesn't reach them.

## Proof

- Proof: a live probe in a job showing what each source needs (redirect target, alternative URL such as Europe PMC's render endpoint or PMC OA); cases with recorded replies for a success or the plain refusal for each.

## Starts from

- Starts from: 0943's ticket, its Progress and `pdf::download`; the article PDF path for Europe PMC and bioRxiv.

## Keeps

- Keeps: licensing: only fetch PDFs the sources serve openly; 0943's Accept handling.

## Defers

- Defers: publisher PDFs other than these two.

## Changes

- Changes: the article PDF download code, its cases.
