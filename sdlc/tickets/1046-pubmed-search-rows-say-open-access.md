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
