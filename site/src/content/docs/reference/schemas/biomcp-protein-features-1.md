---
title: biomcp.protein_features/1
description: "The BioMCP 1.0.0 reply schema biomcp.protein_features/1: its
  envelope fields, its result type ProteinFeatures and the commands that return
  it."
goal: Look up the fields of a biomcp.protein_features/1 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.protein_features/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.protein_features/1`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.protein_features/1`.

These commands return it: [`biomcp protein features`](/reference/commands/protein-features/).

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | ProteinFeaturesQuery | yes | Parsed query. |
| `result` | ProteinFeatures or null | no | Answer when available. |
| `schema` | "biomcp.protein_features/1" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: ProteinFeatures

Native UniProt feature rows, preserving exact labels and uncertain bounds.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `items` | array of ProteinFeature | yes | Whole native rows in source order; totals apply only to this collection. |

## Query: ProteinFeaturesQuery

Page five native UniProt features by exact type and displayed residue. No variant effect or clinical meaning is inferred; no bioinformatics tool runs.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `accession` | UniProtIsoform | yes | Exact source protein sequence identity. |
| `limit` | integer (uint32) | no | Five rows by default, at most 25. |
| `offset` | integer (uint32) | no | Native or retained page offset. |
| `pool` | Text | no | Retained collection identity. |
| `position` | integer (uint32) | no | Native one-based residue; zero fails during deserialization. |
| `type` | Text | no | Exact UniProt native label, without an alias map. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.protein_features/1` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
