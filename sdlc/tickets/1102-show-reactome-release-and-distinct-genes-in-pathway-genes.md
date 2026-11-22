# 1102 — show-reactome-release-and-distinct-genes-in-pathway-genes

Status: COMPLETE. Built on its batch branch and landed inside the 1078 branch (f0ef0d9f0)

Milestone: 7

## Outcome

Return the Reactome database release and the distinct mapped human-gene count in the pathway-members reply, with the count’s scope and any unmapped rows stated.

## Why

Found by a hand solve of research tasks, 2026-11-20. `biomcp pathway genes R-HSA-5693579 --limit 200` returned `"total_kind":"distinct-references"`. The standalone reply gives the pathway version only. The `get pathway` form already gives the release, and the report had to isolate and count human genes. Ticket 1045 filters gene-product references.

## Proof

- Proof: Replay a pathway with a small molecule, two references for one gene, another gene and an unmapped reference. Assert the source release, distinct-gene count and reference total before paging and check that they agree in both pathway reply forms.

## Starts from

- Starts from: `crates/biomcp-query/src/pathway/relation.rs`, `crates/biomcp-query/src/pathway/get.rs` and `crates/biomcp-query/src/pathway/get_sections.rs` and ticket 1045.

## Keeps

- Keeps: Pathway version, individual references and paging retain their meaning.

## Defers

- Defers: Counting genes across sources or species.

## Changes

- Changes: Reactome pathway membership replies, help and cases.
