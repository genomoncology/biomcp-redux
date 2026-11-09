---
title: Reply of biomcp get phenotype (biomcp.phenotype-xrefs/4)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get phenotype: the
  fields of schema biomcp.phenotype-xrefs/4."
goal: "Parse the biomcp.phenotype-xrefs/4 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.phenotype-xrefs/4
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get phenotype`](/reference/commands/get-phenotype/) and [`biomcp get`](/reference/commands/get/) reply with `schema` set to `biomcp.phenotype-xrefs/4`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.phenotype-xrefs/4` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_PhenotypeXrefs](#reply-phenotypexrefs). `query` holds [Reply_PhenotypeGet](/reference/schemas/types/#reply-phenotypeget), a shared type. `query` repeats the request as BioMCP read it.

### Reply_PhenotypeXrefs

Source reference rows without equivalence promotion.

| Field | Type | Meaning |
| --- | --- | --- |
| `components` | list of [Reply_ComponentLabel](/reference/schemas/types/#reply-componentlabel) | Applicable publisher component terms. Optional. |
| `fetched` | integer or null | Original source retrieval seconds, distinct from ontology release. Optional. |
| `items` | list of [Reply_PhenotypeXref](#reply-phenotypexref) | Native ordered rows; empty remains an empty collection. Optional. |

### Other types in this reply

One more type appear only in this reply: [Reply_PhenotypeXref](#reply-phenotypexref). Open the list below to read their fields.

<details>
<summary>Fields of the 1 other type</summary>

### Reply_PhenotypeXref

One exact native cross reference, without equivalence promotion.

| Field | Type | Meaning |
| --- | --- | --- |
| `attribution` | [Reply_Attribution](/reference/schemas/types/#reply-attribution) | Publisher provenance. |
| `id` | [Reply_Curie](/reference/schemas/types/#reply-curie) | Exact publisher reference. |
| `predicate` | [Reply_Curie](/reference/schemas/types/#reply-curie) or null | Native predicate when actually supplied. Optional. |

</details>

## Next

[`biomcp get phenotype`](/reference/commands/get-phenotype/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
