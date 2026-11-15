# 0873 — remaining-pages-report-totals-honestly

Status: OPEN.

Milestone: 7

## Outcome

The pages 0859's survey left behind report a total honestly: the source's own count, a stated "the source gives no total", or a stated filtered count. None sums two sources' totals. A cut-short article search says in plain words that rows past those read can't be paged.

## Why

0859's survey (its Progress section) and its review on 2026-11-12 found these gaps, all instances of 0859's Outcome that it left for a follow-up:

- Pathway search across Reactome and WikiPathways adds their totals (`crates/biomcp-query/src/pathway/union.rs:260`), which 0859's Keeps forbid.
- GWAS search with a p-value cut reports the count before BioMCP's own filter.
- Article recommendations and author papers: the source gives no total and the reply doesn't say so.
- Cell-line search reports no total; whether Cellosaurus offers one is unchecked.
- Article search with one source listed twice loses that source's count (`crates/biomcp-query/src/article/sources.rs:209-210`).
- A cut-short Europe PMC search says so only through the `truncated` flag; past the 100 rows read there is no step and no sentence saying why.
- Drug approvals showed 11 of 25 records with no next step in a persona run (unchecked; the drug files were busy with the label tickets).

## Proof

- Proof: first check each item live and record which still holds; then a routine case for each item that holds shows the honest total or the "no total" statement.

## Starts from

- Starts from: 0859's survey table and its changes to `crates/biomcp-types/src/response/page.rs` and `crates/biomcp-query/src/article/page.rs`.

## Keeps

- Keeps: never invent or sum totals; paging contracts; 0859's behaviour.

## Defers

- Defers: paging Europe PMC beyond BioMCP's read pool.

## Changes

- Changes: `crates/biomcp-query/src/`, `crates/biomcp-adapters/src/`, `tests/cases/`

## Progress

### Live check, 2026-11-15 (pm job j41438, main 648240afe)

| Item | Still holds? | What the live run showed |
| --- | --- | --- |
| Pathway search across Reactome and WikiPathways | Yes | `search pathway MAPK --source reactome --source wikipathways --limit 5` gave total 117: Reactome's own 110 plus WikiPathways' 7. |
| GWAS search with a p-value cut | Yes | `search gwas --pubmed 17529967 --p-value 1e-10 --limit 3` gave total 6 (GWAS Catalog's count before the cut) with no word that it is; `--gene BRCA2 --p-value 5e-8` gave 143 the same way. |
| Article recommendations | Yes | Total 5, the papers returned. Semantic Scholar's reply holds only `recommendedPapers`, no count, and BioMCP didn't say so. |
| Author papers (Semantic Scholar) | Yes | No page and no word why. Semantic Scholar's reply holds `offset`, `next` and `data`, no count. |
| Cell-line search | Yes | No total. Cellosaurus's JSON search reply holds only `cell-line-list`; its `format=txt` header states `response.numFound` (2,331 for `id:HeLa`), which BioMCP doesn't read. |
| Article search with one source listed twice | No | `search article tebentafusp --source europepmc --source europepmc` is now refused as a usage error, so the dropped count can't happen that way. A source still repeats when drug names make one request per name (Semantic Scholar, LitSense); that path is unchanged. |
| Cut-short Europe PMC search | Yes, and worse | At `--offset 97 --limit 3` there was no page step and no sentence. Also, with a drug name, MyChem and ChEMBL joined the source list, so the total fell back to the 100 rows read instead of Europe PMC's 1,178. |
| Drug approvals | No | `get drug apixaban approvals --source drugsfda` showed 10 of 25 with a page step; `--section-limit 5` 5 of 25 with a step; `--section-limit 30` all 25. |

### Changes

- Pathway search over several sources gives each source's own count in `sources[].count` (Reactome's `found`, WikiPathways' matching catalog rows). The page total is the rows read so far, with the new total kind `read`; it is no longer the sum.
- GWAS search with `--p-value`: while the walk runs, the GWAS source says its count is from before the cut; once the walk ends, `page.total` is the count at or below the cut (the same as `qualifying_total`).
- Article recommendations: the `scope` sentence says Semantic Scholar gives no total and the page total counts the papers returned.
- Author papers from Semantic Scholar: a source notice says the source gives no total and a page step follows while it has more.
- Cell-line search: a full page's Cellosaurus notice says the search reply gives no total, beside the next page step.
- Article search: only sources that retrieved a window count toward "one source", so name and title lookups no longer hide Europe PMC's count. At the end of the read window, each cut-short source says how many it matched, how many BioMCP read, and that rows past those can't be paged.
- Cases: `pathway/union-totals.yaml`, `gwas/threshold-total.yaml`, `cell-line/search-total.yaml`, `article/search/europepmc-window.yaml` and `author/papers-no-total.yaml`; the recommendation cases check the new scope sentence. Replies are j41438's live recordings; the WikiPathways catalogs keep only the 24 rows whose title contains MAPK. Recommendations and author papers use the existing synthetic Semantic Scholar replays.
- Author paper caps follow the new measured size plus 15%: `author/papers.yaml` and `papers-disabled-pool.yaml` 864 to 1,060 (measured 921), `papers-details.yaml` 1,359 to 1,698 (measured 1,476).
- Checks: j41443 regenerated the four pathway schemas (the `read` kind widens the enum) and found two ratchet hits, a real Semantic Scholar ID in a case's note and `.ok()` drops in the pathway union; j41444 ran the pathway, GWAS, cell-line, article, author and mcp-parity cases (all new cases passed; only the author caps above failed).
- Gate `make --keep-going landing-gate docs-check schema-release-check` passed in pm job j41447 at f04408bb5 (978 cases).

### Review fixes, 2026-11-15

- GWAS: a walk that ends with rows dropped (no exact p-value, or refused) now says the count at or below the cut is unknown, instead of saying paging will reveal it. Case `gwas/threshold-dropped.yaml` replays an authored copy of j41438's page with `pvalue_mantissa` removed from the second row; its README says so.
- Cell line: the notice now says the Cellosaurus JSON search reply BioMCP reads gives no total (its txt format states `numFound`; reading it is a separate ticket).
- `tests/replies/pathway/union-totals/README.md` states the trim (2,218 to 24 rows, "MAPK" in the title without regard to case) and the recording job.
- Pathway type descriptions mention `read`. The union's `read` total now counts rows actually handed out (the previous page's total less rows still queued), so an empty page's `next_offset` step of one no longer adds a row never read.
- Review fixes checked in j41452 (pathway, GWAS and cell-line cases, ratchets, clippy). Gate passed on the branch merged with origin/main in pm job j41453 at 7776bb908 (979 cases).
- Re-review additions: `pathway/union-totals.yaml` follows the page step. After page 1 (Reactome, WikiPathways, Reactome, WikiPathways, Reactome) Reactome has 2 rows queued and WikiPathways 5, so page 2 hands out W, R, W, R, W with no new request: offset 5, 5 returned, `read` 12 (5 handed before, 5 now, 2 queued), next offset 10. The union snapshot key is now `pathway-union/2`, so pools saved with the old summed total come back expired. The GWAS note for a finished walk with unchecked rows now says "some rows could not be checked against the cut", which also covers a stale or syncing source.
- Re-review checks: j41457 at 0c121c3cc ran fmt (no change), check-cheap and the pathway and GWAS cases; the page-2 numbers held as derived. Gate passed on the branch merged with origin/main (57cf6f6e3) in pm job j41458 at 0c121c3cc.
