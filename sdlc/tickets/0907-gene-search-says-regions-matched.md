# 0907 — gene-search-says-regions-matched

Status: OPEN.

Milestone: 7

## Outcome

An agent that searches genes by band or region learns from the top of the reply that ClinGen regions matched, how many there are, and how to see only them. Its own filter on the gene rows can no longer hide the regions.

## Why

In rare-disease round 2 (biomed-agent-bench `personas/rare-disease/runs/2026-11-13-r2/`, log.md:26-31, analysis beside it), the agent ran `biomcp search gene --band 16p11.2 --limit 12`. It read `.result.items[]` with jq, printed the first 700 bytes, and reported "No region-level ClinGen dosage record anywhere in the reply".

The PM re-ran that call offline on the run's build and cache. The reply carried `result.regions` with ISCA-37400 and ISCA-37486 (and four gnomAD population regions), after 12 gene rows, with nothing earlier saying so. 0888 delivered the data, but the agent never saw it.

## Proof

- Proof: the recorded `tests/cases/gene/regions.yaml` 16p11.2 step shows three things:
  - a notice near the start of the reply naming how many ClinGen regions matched, ahead of the gene rows;
  - a first `next` step that returns only the regions, as a runnable command;
  - a regions-only form (for example `--regions-only` or a section) that returns the regions without gene rows.
  All three work the same through CLI and MCP.

## Starts from

- Starts from:
  - 0888's region search in `crates/biomcp-query/src/gene/search.rs`;
  - the reply notice and `next` builders (`crates/biomcp-query/src/next.rs`).

## Keeps

- Keeps: the gene rows and region fields as they are; reply size within its measured limit plus 15%.

## Defers

- Defers: the general "related queries" form, which is 0904.

## Changes

- Changes:
  - `crates/biomcp-query/src/gene/`;
  - `crates/biomcp-types/` (an optional notice or selector);
  - `tests/cases/gene/regions.yaml`;
  - `docs/`.
