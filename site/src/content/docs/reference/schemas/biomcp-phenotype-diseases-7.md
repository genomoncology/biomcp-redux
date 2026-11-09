---
title: Reply of biomcp get phenotype (biomcp.phenotype-diseases/7)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get phenotype: the
  fields of schema biomcp.phenotype-diseases/7."
goal: "Parse the biomcp.phenotype-diseases/7 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.phenotype-diseases/7
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get phenotype`](/reference/commands/get-phenotype/) and [`biomcp get`](/reference/commands/get/) reply with `schema` set to `biomcp.phenotype-diseases/7`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.phenotype-diseases/7` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_PhenotypeDiseases](#reply-phenotypediseases). `query` holds [Reply_PhenotypeGet](/reference/schemas/types/#reply-phenotypeget), a shared type. `query` repeats the request as BioMCP read it.

### Reply_PhenotypeDiseases

Direct source disease association rows without clinical inference.

| Field | Type | Meaning |
| --- | --- | --- |
| `components` | list of [Reply_ComponentLabel](/reference/schemas/types/#reply-componentlabel) | Applicable publisher component terms. Optional. |
| `direct` | boolean | Publisher direct graph scope; this supplies no causal assertion. |
| `fetched` | integer or null | Original source retrieval time, distinct from dataset release. Optional. |
| `items` | list of [Reply_DiseaseAssociationItem](/reference/schemas/types/#reply-diseaseassociationitem) | Native association rows in source order, without deduplication. Optional. |

## Next

[`biomcp get phenotype`](/reference/commands/get-phenotype/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
