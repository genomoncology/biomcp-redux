---
title: Reply of biomcp protein features (biomcp.protein-features/4)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp protein features:
  the fields of schema biomcp.protein-features/4."
goal: "Parse the biomcp.protein-features/4 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.protein-features/4
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp protein features`](/reference/commands/protein-features/) and [`biomcp protein`](/reference/commands/protein/) reply with `schema` set to `biomcp.protein-features/4`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.protein-features/4` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_ProteinFeatures](#reply-proteinfeatures). `query` holds [Reply_ProteinFeaturesQuery](#reply-proteinfeaturesquery). `query` repeats the request as BioMCP read it.

### Reply_ProteinFeatures

Native UniProt feature rows, preserving exact labels and uncertain bounds.

| Field | Type | Meaning |
| --- | --- | --- |
| `items` | list of [Reply_ProteinFeature](/reference/schemas/types/#reply-proteinfeature) | Whole native rows in source order; totals apply only to this collection. Optional. |

### Reply_ProteinFeaturesQuery

Page five native UniProt features by exact type and displayed residue. No variant effect or clinical meaning is inferred; no bioinformatics tool runs.

| Field | Type | Meaning |
| --- | --- | --- |
| `accession` | [Reply_UniProtIsoform](/reference/schemas/types/#reply-uniprotisoform) | Exact source protein sequence identity. |
| `limit` | integer (uint32) | Five rows by default, at most 25. Optional. |
| `offset` | integer (uint32) | Native or retained page offset. Optional. |
| `pool` | [Reply_Text](/reference/schemas/types/#reply-text) | The ID of a stored list, from an earlier reply, so later pages read the same data. Optional. |
| `position` | integer (uint32) | Native one-based residue; zero fails during deserialization. Optional. |
| `type` | [Reply_Text](/reference/schemas/types/#reply-text) | Exact UniProt native label, without an alias map. Optional. |

## Next

[`biomcp protein features`](/reference/commands/protein-features/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
