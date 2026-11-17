# 1046 — pubmed-search-rows-say-open-access

Status: OPEN.

Milestone: 7

## Outcome

`search article` rows from PubMed fill the same `open_access` field Europe PMC rows fill since 0827, and the licence where the source states one, so a reader can tell which papers may be shared without opening each one. A row whose status the source does not state says it is unknown rather than leaving the field empty.

## Why

The msl persona needed sharing rights across a list of papers in its 2026-11-10, 2026-11-12 and 2026-11-16 runs. PubMed rows came back with the field empty, so the agent treated having a PMCID as "shareable", which is wrong (biomed-agent-bench `personas/msl/runs/2026-11-12/log.md:108,121`; `personas/msl/runs/2026-11-16/analysis.md`, fact 9.2: "Search `open_access` is filled only by Europe PMC rows"; `records/2026-11-12-persona-review-findings.md`, Waiting). 1016 explained the licence codes but did not fill PubMed rows. Ian's `decisions/2026-10-27-1-0-ships-every-known-fix.md` puts every known gap in 1.0.

## Proof

- Proof: a replayed PubMed `search article` page with one PMC open-access paper and one paper without PMC full text expects `open_access` true with its licence on the first row and false on the second. It fails on the base commit.

## Starts from

- Starts from: the PubMed search mapping in `crates/biomcp-adapters/src/pubmed/` and the Europe PMC row field in `crates/biomcp-adapters/src/europepmc/mod.rs:165-168`. The developer checks which source states open-access status for a page of PMIDs in one batched request (PubMed's own PMC links, the PMC OA service, or a Europe PMC ID lookup) and picks the cheapest one that states it, recording the choice and its live check in Progress. Any new source call follows the source's rate limit and the article rights policy in `crates/biomcp-adapters/src/article_rights/`.

## Keeps

- Keeps: Europe PMC rows, the `--open-access` filter, and 1016's licence meanings.

## Defers

- Defers: nothing.

## Changes

- Changes: the PubMed search adapter and mapping, a case and its recording, and one release-notes line.

## Progress

- Source choice, checked live 2026-11-17 with curl. PubMed's ESummary states a PMCID but no open-access status or licence (PMIDs 42640561, 42852808, 42162665 all show only `Has Abstract`). NCBI's `"pubmed pmc open access"[filter]` would state status in one ESearch but no licence, and the PMC OA service and the PMC OA bucket answer one paper per request. A single Europe PMC `search?query=SRC:MED AND (EXT_ID:… OR …)&resultType=core` request states both `isOpenAccess` and `license` for the whole page. For the first 20 PubMed `tebentafusp` PMIDs it returned all 20 rows: 36970111 Y `cc by-nc`, 31336704 Y `cc by`, 37870955 N with PMCID PMC11188986 (a PMCID is not open access), 42162665 N with publisher licence `cc by-nc-nd`, 37643277 Y with no licence. Europe PMC was chosen. The `lite` result type is 18 KB for 20 rows but states no licence; `core` is 288 KB, so a body filter keeps only `hitCount` and each row's `source`, `pmid`, `isOpenAccess` and `license` before the reply reaches the cache or a recording, and no abstract text is kept.
- Change: `search article` pages read the open-access flag and licence for rows that have a PMID and no flag, in one Europe PMC request per page, bounded to 3 seconds like the article licence lookup. Found values are saved with the stored page. A row still without a flag carries `open_access_unknown: true`. A failed lookup settles nothing and does not change the page status. `ArticleItem` gains two optional fields, `open_access_unknown` and `license`. Citation, reference and recommendation pages are unchanged.
