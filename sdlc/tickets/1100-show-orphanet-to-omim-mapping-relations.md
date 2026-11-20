# 1100 — show-orphanet-to-omim-mapping-relations

Status: OPEN.

Milestone: 7

## Outcome

Return the OMIM IDs mapped by Orphanet from a requested ORPHA ID, with the published relation, direction, release and source.

## Why

Found by a hand solve of research tasks, 2026-11-20. `biomcp get disease ORPHA:115 xrefs --source orphadata` returned `"valid_values":["mydisease"]`. Ticket 1062 deferred Orphanet’s own mapping relations because current cross-references hold bare codes.

New source: Orphadata rare-disease cross-reference product, https://www.orphadata.com/data/xml/en_product1.xml, Orphanet

## Proof

- Proof: A saved Orphadata release with exact and broader mappings returns both relations in the published direction and keeps the ORPHA and OMIM IDs.

## Starts from

- Starts from: `crates/biomcp-query/src/disease/orphadata.rs` and `crates/biomcp-adapters/src/orphadata/`.

## Keeps

- Keeps: MONDO identity lookup and prevalence reads retain their behavior.

## Defers

- Defers: Inferring an exact relation from a bare cross-reference.

## Changes

- Changes: Disease cross-references, the Orphadata adapter, help and cases.
