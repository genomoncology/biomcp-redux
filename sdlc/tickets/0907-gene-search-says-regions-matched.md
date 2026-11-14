# 0907 — gene-search-says-regions-matched

Status: COMPLETE.
Landed: 3783419ee2de1598581da9ea8b58f12ec0fd63f8

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

## Progress

- Changed: `search gene` replies carry `result.clingen_regions_matched`, the number of ClinGen regions that matched, whenever any did. It is the first field of `result`, so in CLI JSON it comes right after `schema`, `status` and `query`, before `items` (the key starts at byte 111 of the live 16p11.2 reply in job j39834, ahead of the first gene row). MCP and Markdown replies sort keys, and the name sorts before `items` there too; in MCP the sorted envelope also puts `next` ahead of `result`. A `.result.items[]` filter still hides it, as it hides anything outside the rows; the count is the first thing past the query.
- Changed: when regions matched and gene rows were read, the first `next` step (kind `section`) is the same search with `--regions-only`. The new optional `regions_only` selector (`--regions-only`) skips HGNC and returns only the ClinGen regions for a band, interval or ISCA ID; with plain text and no band or interval it is a usage error. `GenePage` gains an optional field and `GeneSearch` an optional boolean, both allowed for the 1.0.0 candidate.
- Measured in job j39834 on live sources: `search gene --band 16p11.2 --limit 12` is 5,430 bytes, 245 genes and 6 regions (ISCA-46492, -46491, -46490, -46489, -37486, -37400); the regions-only reply is 2,672 bytes. `help search gene` grew from under 2,435 to 2,767 bytes for the new option, so its cap is 3,183 (measured plus 15%, per `sdlc/decisions/2026-10-26-measured-entity-caps-need-no-ask.md`).
- Proof: `tests/cases/gene/regions.yaml` runs the agent's own `search gene --band 16p11.2 --limit 12` through CLI and MCP: count 6, 12 gene rows of 245, both ISCA regions, and `next[0]` the regions-only command. A second step runs that command through CLI and MCP and gets the six regions, ClinGen as the only source and no gene rows. The case's HGNC slice gains the 245 real 16p11.2 rows recorded in j39834 (`tests/replies/gene/regions/README.md`). `crates/biomcp-types/tests/gene_model.rs` checks that the count precedes the gene rows in both declared and sorted key order.
