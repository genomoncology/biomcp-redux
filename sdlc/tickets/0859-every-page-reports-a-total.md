# 0859 — every-page-reports-a-total

Status: COMPLETE.
Landed: 22bf5a43686fe355be314fe2dbe0056d2b9e0d38

Milestone: 7

## Outcome

Every search and section page reports the source's total, or says the source gives none, and a page cut short by a limit says so with its next step; trial locations report their total and accept a larger page.

## Why

From the bench 0039 persona review (2026-11-12, runs on BioMCP 58d83b6da), `personas/<slug>/runs/2026-11-12/review.md` and `log.md`: pt-committee log.md:13, rare-disease log.md:21, msl log.md:15 and 42, comp-bio log.md:47-50: capped pages looked complete; patient-navigator log.md:155 and 450: 230 sites took 24 calls at 10 per page.

## Proof

- Proof: routine cases for an article search, a CIViC section and trial locations show the total and the truncation next step; locations accept a page of 100.

## Starts from

- Starts from: the shared page type and the adapters that drop the native total.

## Keeps

- Keeps: never invent or sum totals; paging contracts.

## Defers

- Defers: distance sort for locations (BioMCP 0083).

## Amendment, 2026-11-12

From bench 0040's rerun, reproduced on main 59150d16d (`sdlc/records/2026-11-12-persona-rerun-failures.md`):

- Trial locations: `--limit 50` returns 20 sites, and an offset past the end is a usage error instead of an empty page (failure #16); Europe PMC reports the retrieved pool (100) as the total where the source says 188 (failure #20).

## Changes

- Changes: `crates/biomcp-query/src/`, `crates/biomcp-adapters/src/`, `tests/cases/`

## Progress

### Survey, 2026-11-12

Read from the code on main efe47e466. Paths are under `crates/`. "Total" means `page.total` or a section's own `total`.

| Page | Source | Source gives a total? | What BioMCP reports now |
| --- | --- | --- | --- |
| `search article` (one source, pooled) | Europe PMC, PubMed, PubTator, Semantic Scholar, LitVar | Yes: `hitCount`, `count`, `total`, `pmids_count`. Each read is a window (Europe PMC 100 rows, PubMed 9,999). | The pool size (`biomcp-query/src/article/page.rs`, `finish`): Europe PMC 188 hits read as 100. Source counts only in `sources[].count`. **Fixed here.** |
| `search article` (several sources) | same | Yes, per source | The deduplicated pool size, with `truncated` when any source matched more. Summing would double-count, so this stays; per-source counts stay in `sources[].count`. |
| `search article` served live when no pool could be written | same | Yes | The retrieved count, no `truncated` (`article/page.rs`, `fetched`). **Fixed here.** |
| `search article` with one source listed twice | same | Yes | `count` and `retrieved_count` dropped for that source (`article/sources.rs:209-210`), so `truncated` cannot fire. Follow-up. |
| `article recommendations` | Semantic Scholar | No | The returned count, without saying the source gives none. Follow-up. |
| `article citations`/`references --source semantic-scholar` | Semantic Scholar graph | No | The read window, with a `graph-truncated` notice and no source count. OK. |
| `get trial … locations` | ClinicalTrials.gov record | Whole record | Correct total, but a 4,500-byte budget cut `--limit 50` to 20 sites, `--limit` stopped at 50, and an offset past the end was a usage error. **Fixed here.** |
| `get trial …` other sections | ClinicalTrials.gov record | Whole record | Correct total; byte budgets cut pages short, with a continuation. OK. |
| `get variant … civic` | CIViC GraphQL | Yes, `totalCount` | CIViC's `totalCount` per evidence and assertion stream, with a page step (`civic/variant_map.rs`, `claim_page`). OK; the routine case now shows it. |
| `get drug … civic`, `get gene … civic` | CIViC | Yes | CIViC's `totalCount`. OK. |
| `search gwas --p-value` | GWAS Catalog | Yes, before BioMCP's p-value filter | The unfiltered source total, `truncated` while the walk runs; the filtered count only in `qualifying_total` (`gwas/threshold.rs:239-247`). Follow-up. |
| `search pathway` over Reactome and WikiPathways | Reactome `found`; WikiPathways local catalog | Reactome yes | The sum of both (`pathway/union.rs:260`), no per-source count. Follow-up: the ticket keeps "never sum". |
| `search cell-line` | Cellosaurus | Unknown: the adapter parses no count | No page at all; a next page only when the page is full (`cell_line/mod.rs`). Follow-up: check for a count, else say none. |
| `author papers` (Semantic Scholar) | Semantic Scholar | No | No page, without saying the source gives none (`author/semantic_scholar.rs:160`). Follow-up. |
| `variant recurrence` | cBioPortal | Available but not asked for | No total; `all_pages_read` says so. OK by design. |
| `get drug NAME approvals` | Drugs@FDA | Whole application | The persona saw 11 of 25 records with no next step (pt-committee log.md:13). Code reads the whole application (`drug/drugsfda/approvals.rs`); not checked further because drug files are held by tickets 0871 and 0872. Follow-up. |
| Every other search and section | (trial, drug, gene, variant, disease, phenotype, PGx, GWAS, protein, adverse event, diagnostic, study, organization, biomarker, author search) | Yes, or a whole record or local bundle | The source total or the whole list. OK. |

Latent: several pages read `window.total().unwrap_or(0)` (`drug/native/sections.rs:170`, `drug/openfda_label.rs:74,215`, `study/get.rs:150,185`, `study/get/files.rs:62`, `study/get/rows.rs:112`, `cell_line/chembl.rs:44`, `disease/seer.rs:31,221`), so a missing source total would read as 0. Every window feeding them carries a total today.

### Changes

- Trial locations fill the page asked for, up to `--limit 100` (the trial get limit is now 1-100), state the registry's site total, and give the next page until the last site. An offset past the end of any record section returns an empty page with the total and no next step; an eligibility character offset past the text is still refused.
- Article search reports one source's own count as `page.total` when it matched more than BioMCP read (Europe PMC 188, not 100). Page steps stay inside the read window; `truncated` stays true. Several sources keep the deduplicated pool size. The live path without a pool does the same and now sets `truncated`.
- Cases: `trial/sites/locations-page.yaml`, `article/search/europepmc-total.yaml`, the next page step in `variant/civic-section-limit.yaml`, and the runner trial `trial-sites/sites/page-past-end-is-empty`.
