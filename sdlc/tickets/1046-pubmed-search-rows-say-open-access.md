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
- Change: `search article` pages read the open-access flag and licence for rows that have a PMID and no flag, in one Europe PMC request per page, bounded to 8 seconds: Europe PMC's core search took 2.2 seconds for two papers in j42460 and ran past a first 3-second bound on other tries, leaving every row unknown. Found values are saved with the stored page. A row still without a flag carries `open_access_unknown: true`. A failed lookup settles nothing and does not change the page status. `ArticleItem` gains two optional fields, `open_access_unknown` and `license`. Citation, reference and recommendation pages are unchanged.
- Proof: `tests/cases/article/search/pubmed-access-rights.yaml` replays `search article tebentafusp --exact-term --offset 7 --limit 2`, recorded live with `--fresh --record` in j42460: 36970111 (PMC10031621) is `open_access` true with `cc_by_nc`, and 42162665 (no PMC copy) is false with the publisher's `cc_by_nc_nd`. The case fails on base 876387841's crates (`open_access` missing) and passes on the branch (j42461).
- Other cases: the AKT1 E17K page (`article/search/live`) gains a Europe PMC access reply captured with curl of BioMCP's exact request (body trimmed to the kept fields), and its ceilings in `readme/article`, `article/filters/include-retracted` and the LitVar 30-row step move to the measured size plus 15 percent. The LitVar replay has no access reply, so its rows say `open_access_unknown`. The PubMed-only store-contention test disables Europe PMC because its stub serves PubMed alone. A failed lookup is a reviewed absence in the adapter value audit. In j42461, all article, readme, search and mcp-parity cases pass, and schemas regenerated with only the two optional fields in three article schemas.
- Gate j42468 on de23db9a8 (contains origin/main 876387841): `make --keep-going landing-gate docs-check schema-release-check` passed.
- Review fixes at 2e83bcc. A reviewer timed Europe PMC at 11 to 19 seconds for 7 of 8 calls, so: the lookup now runs beside the PubMed title read (`article/access.rs`, through `fan_out`), not after it, on its own Europe PMC leg capped at 3 seconds that honours `--fresh`. A failed or timed-out lookup names Europe PMC in `sources` with its state and reason, and adds a fresh retry step; the page status is unchanged, and a search that also asked Europe PMC records the failure on that source's row. `--no-access-lookup` (MCP `no_access_lookup`) skips the lookup for one search; config `article_access_lookup: false` makes skipping the default, documented in `docs/configuration.md` (default on). The reviewed-absence entry is gone, because the failure is now named.
- New cases: `pubmed-access-rights` gains a `--no-access-lookup` step (rows unknown, PubMed the only source). `pubmed-access-timeout` replays the same page with the Europe PMC reply's delay raised to 12 seconds (synthetic, README in its replay folder): status complete, rows unknown, `europepmc` failed with `deadline-exceeded`, and a fresh step. Both fail on base 876387841. `readme/article` now expects the AKT1 page's `open_access` [false, true, false, false, false, false, true, false, false, false], which matches the recorded Europe PMC reply, and no `open_access_unknown` on any row. Ceilings re-measured in j42490 and j42494 (plus 15 percent): `article/filters/since` steps 3,639 and 3,877, `article/search/affiliation` 2,947, and the `article/help` argument indices move by one for the new option. Replays without a Europe PMC access reply (since, affiliation, the LitVar walk) now show Europe PMC failed with `replay-miss` beside complete pages.
- Paused 2026-11-17 at d259513d3 (review fixes, merged with origin/main 4b721738b). Gate j42495 failed at `changed-library-tests`, on one test: `crates/biomcp-query/tests/article_search.rs:426` `stale_summary_fallback_stays_partial_when_its_page_is_read_offline`. An offline reread of a stored page now runs the access lookup, so `sources` gains Europe PMC failed with `offline`, while the test expects it with `proxy` (its earlier entry). Next: decide whether the lookup should run at all for an offline read (likely skip it when the fetch is offline) or update the test, rerun the gate, then get an independent re-review of the four review fixes, then land.
