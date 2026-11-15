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

## Progress

- 2026-11-15 probe from the M5 (pm jobs j41234 and j41236, curl with user agent `ureq/3.5.0` and `Accept: application/pdf`, no browser user agent):
  - Europe PMC refuses automated PDF requests on every route. The `fulltextRepo` link that Europe PMC's own REST core record lists for PPR1064174 answered 403 `application/json` `{"error":"PDF link has expired or is invalid"}` (the same 403 0943 saw). The PMC render routes (`/articles/PMC…?pdf=render`, `/articles/pmc…/pdf`, `/backend/ptpmcrender.fcgi`) and the preprint page answered 403 with `cf-mitigated: challenge`, a Cloudflare browser check. BioMCP does not get past either. Europe PMC's open alternatives, its REST full text XML and PMC OA, are already what `get article ID fulltext` and `article text` read.
  - bioRxiv serves PDFs to automated clients. Existing-version links (`…v1.full.pdf`, `…v3.full.pdf`) and the `early/…full.pdf` links Semantic Scholar gives answered 200 `application/pdf`; the unversioned link redirected twice to the latest version and then 200. 0943's 302 was its `v4` link: that paper has only v1-v3, and bioRxiv redirects a missing version to `/node/.full-text.pdf`, which answers 403. So bioRxiv never needed a different request; 0943's probe did not follow redirects. medRxiv `early/` links also served PDFs (j41255).
  - Semantic Scholar gave no Europe PMC PDF links among 450 open-access PubMed papers (j41255); Europe PMC PDF links reach BioMCP through a Europe PMC preprint ID (`article pdf PPR…`), which the Semantic Scholar adapter refuses (`paper_id`), so the reply gave no reason.
- Changed: `article pdf PPR…` no longer asks Semantic Scholar. It answers without a request: file unavailable from `europepmc` with reason `rejected`, the source `skipped`, a notice saying Europe PMC does not give PDFs to automated clients and BioMCP does not get around that, and a next step `get article PPR… fulltext` for Europe PMC's full text.
- Changed: when a publisher refuses a PDF download with HTTP 4xx, the Semantic Scholar source carries a notice naming the host, the URL and the status, saying no file was saved and that the refusal does not show the paper is closed (for a Europe PMC link: that Europe PMC does not give PDFs to automated clients). When the link names a PMC or PPR ID, or the caller gave a PMID, PMCID or PPR ID, a next step offers `get article ID fulltext`. The FDA refusal and this notice share `pdf::refused`.
- Proof: `tests/cases/article/pdf/biorxiv` saves a bioRxiv preprint from replies BioMCP recorded live in j41240 (3,578,936-byte PDF; the body is replaced by BioMCP's synthetic PDF because bioRxiv lists it cc_no). `tests/cases/article/pdf/europepmc-refused` asserts the plain refusal and the full-text next step for PPR1295425. `tests/cases/article/pdf/publisher-refused` replays a live recording (j41270) where Cell answered a Semantic Scholar PDF link for PMID 29625055 with a 403 Cloudflare browser check; before, that reply said only `rejected` (j41261). Other publishers' refusals get the same plain notice, but making their PDFs download stays deferred.
