---
title: Reply of biomcp get phenotype (biomcp.phenotype-hierarchy/5)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get phenotype: the
  fields of schema biomcp.phenotype-hierarchy/5."
goal: "Parse the biomcp.phenotype-hierarchy/5 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.phenotype-hierarchy/5
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get phenotype`](/reference/commands/get-phenotype/) and [`biomcp get`](/reference/commands/get/) reply with `schema` set to `biomcp.phenotype-hierarchy/5`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.phenotype-hierarchy/5` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_PhenotypeHierarchy](#reply_phenotypehierarchy). `query` holds [Reply_PhenotypeGet](/reference/schemas/types/#reply_phenotypeget), a shared type. `query` repeats the request as BioMCP read it.

### Reply_PhenotypeHierarchy

Direct source hierarchy links without graph traversal.

| Field | Type | Meaning |
| --- | --- | --- |
| `components` | list of [Reply_ComponentLabel](/reference/schemas/types/#reply_componentlabel) | Applicable publisher component terms. Optional. |
| `fetched` | integer or null | Original source retrieval seconds, distinct from ontology release. Optional. |
| `items` | list of [Reply_PhenotypeHierarchyLink](#reply_phenotypehierarchylink) | Native ordered rows; empty remains an empty collection. Optional. |

### Other types in this reply

2 more types appear only in this reply: [Reply_PhenotypeHierarchyLink](#reply_phenotypehierarchylink), [Reply_PhenotypeHierarchyRelation](#reply_phenotypehierarchyrelation). Open the list below to read their fields.

<details>
<summary>Fields of the 2 other types</summary>

### Reply_PhenotypeHierarchyLink

One native direct hierarchy link, without traversal or inferred equivalence.

| Field | Type | Meaning |
| --- | --- | --- |
| `attribution` | [Reply_Attribution](/reference/schemas/types/#reply_attribution) | Publisher provenance. |
| `id` | [Reply_Curie](/reference/schemas/types/#reply_curie) | Exact related term identifier; native parents can be outside HP. |
| `label` | string or null | Whole related term label. Optional. |
| `relation` | [Reply_PhenotypeHierarchyRelation](#reply_phenotypehierarchyrelation) | Original source relation, such as super_classes or sub_classes. |

### Reply_PhenotypeHierarchyRelation

Native hierarchy collection that supplies the link.

| Value | Meaning |
| --- | --- |
| `super_classes` | A direct parent in the native super_classes collection. |
| `sub_classes` | A direct child in the native sub_classes collection. |

</details>

## Next

[`biomcp get phenotype`](/reference/commands/get-phenotype/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
