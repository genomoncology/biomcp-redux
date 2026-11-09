---
title: Reply of biomcp get disease (biomcp.disease-card/6)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get disease: the
  fields of schema biomcp.disease-card/6."
goal: "Parse the biomcp.disease-card/6 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.disease-card/6
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get disease`](/reference/commands/get-disease/) and [`biomcp get`](/reference/commands/get/) reply with `schema` set to `biomcp.disease-card/6`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.disease-card/6` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_DiseaseCard](#reply-diseasecard). `query` holds [Reply_DiseaseGet](/reference/schemas/types/#reply-diseaseget), a shared type. `query` repeats the request as BioMCP read it.

### Reply_DiseaseCard

Exact identity and available pages without hidden enrichment.

| Field | Type | Meaning |
| --- | --- | --- |
| `alias` | [Reply_DiseaseRelation](/reference/schemas/types/#reply-diseaserelation) or null | Native relation proving a source-accepted alternate identifier. Optional. |
| `attribution` | [Reply_Attribution](/reference/schemas/types/#reply-attribution) | Publisher record and version. |
| `components` | list of [Reply_ComponentLabel](/reference/schemas/types/#reply-componentlabel) | Applicable component terms. Optional. |
| `concept` | [Reply_DiseaseConcept](/reference/schemas/types/#reply-diseaseconcept) | Native identity and complete primary label. |
| `consider` | list of [Reply_Curie](/reference/schemas/types/#reply-curie) or null | Publisher alternatives, never an automatic replacement. Optional. |
| `definition` | [Reply_Text](/reference/schemas/types/#reply-text) or null | Complete source definition when supplied. Optional. |
| `obsolete` | boolean or null | Publisher obsolete flag. Optional. |
| `replaced_by` | list of [Reply_Curie](/reference/schemas/types/#reply-curie) or null | Publisher explicit replacement identifiers; null preserves no native replacement. Optional. |
| `requested_id` | [Reply_Curie](/reference/schemas/types/#reply-curie) | Exact requested identifier. |
| `returned_id` | [Reply_Curie](/reference/schemas/types/#reply-curie) | Actual returned identifier. |
| `sections` | list of [Reply_DiseaseSection](/reference/schemas/types/#reply-diseasesection) | Available named sections. Optional. |

## Next

[`biomcp get disease`](/reference/commands/get-disease/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
