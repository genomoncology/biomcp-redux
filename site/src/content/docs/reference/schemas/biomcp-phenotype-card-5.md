---
title: Reply of biomcp get phenotype (biomcp.phenotype-card/5)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get phenotype: the
  fields of schema biomcp.phenotype-card/5."
goal: "Parse the biomcp.phenotype-card/5 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.phenotype-card/5
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get phenotype`](/reference/commands/get-phenotype/) and [`biomcp get`](/reference/commands/get/) reply with `schema` set to `biomcp.phenotype-card/5`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.phenotype-card/5` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_PhenotypeTermCard](#reply_phenotypetermcard). `query` holds [Reply_PhenotypeGet](/reference/schemas/types/#reply_phenotypeget), a shared type. `query` repeats the request as BioMCP read it.

### Reply_PhenotypeTermCard

Independent source identity card and its available sections.

| Field | Type | Meaning |
| --- | --- | --- |
| `components` | list of [Reply_ComponentLabel](/reference/schemas/types/#reply_componentlabel) | Applicable publisher component terms. Optional. |
| `sections` | list of [Reply_PhenotypeSection](/reference/schemas/types/#reply_phenotypesection) | Sections available on this source record. Optional. |
| `term` | [Reply_PhenotypeCard](#reply_phenotypecard) | Complete source-specific native card. |

### Other types in this reply

2 more types appear only in this reply: [Reply_PhenotypeAssociationCount](#reply_phenotypeassociationcount), [Reply_PhenotypeCard](#reply_phenotypecard). Open the list below to read their fields.

<details>
<summary>Fields of the 2 other types</summary>

### Reply_PhenotypeAssociationCount

Native collection counts, without summing scopes or counting patients.

| Field | Type | Meaning |
| --- | --- | --- |
| `category` | [Reply_Curie](/reference/schemas/types/#reply_curie) | Native category. |
| `count` | integer or null | Native closure-expanded count. Optional. |
| `count_direct` | integer or null | Native direct count. Optional. |
| `count_with_orthologs` | integer or null | Native count including orthologs, only when supplied. Optional. |
| `key` | [Reply_Curie](/reference/schemas/types/#reply_curie) | Native category key. |
| `label` | string or null | Native scope label. Optional. |

### Reply_PhenotypeCard

Native source card, with independent HPO and Monarch metadata.

One of these forms:

- `source` is `"hpo"`: The direct HPO term answer. Fields: `attribution` ([Reply_Attribution](/reference/schemas/types/#reply_attribution)), `comment` (string or null), `definition` (string or null), `hpo_descendant_count` (integer or null), `label` (string or null), `publication_references` (list of [Reply_Curie](/reference/schemas/types/#reply_curie) or null), `requested_id` ([Reply_HpId](/reference/schemas/types/#reply_hpid)), `returned_id` ([Reply_HpId](/reference/schemas/types/#reply_hpid)).
- `source` is `"monarch"`: The independent Monarch term answer. Fields: `association_counts` (list of [Reply_PhenotypeAssociationCount](#reply_phenotypeassociationcount) or null), `attribution` ([Reply_Attribution](/reference/schemas/types/#reply_attribution)), `deprecated` (boolean or null), `description` (string or null), `label` (string or null), `monarch_descendant_count` (integer or null), `requested_id` ([Reply_HpId](/reference/schemas/types/#reply_hpid)), `returned_id` ([Reply_HpId](/reference/schemas/types/#reply_hpid)).

</details>

## Next

[`biomcp get phenotype`](/reference/commands/get-phenotype/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
