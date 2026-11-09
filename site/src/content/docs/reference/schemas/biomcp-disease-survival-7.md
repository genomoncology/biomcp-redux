---
title: Reply of biomcp get disease (biomcp.disease-survival/7)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get disease: the
  fields of schema biomcp.disease-survival/7."
goal: "Parse the biomcp.disease-survival/7 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.disease-survival/7
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get disease`](/reference/commands/get-disease/) and [`biomcp get`](/reference/commands/get/) reply with `schema` set to `biomcp.disease-survival/7`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.disease-survival/7` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_DiseaseSurvival](#reply_diseasesurvival). `query` holds [Reply_DiseaseGet](/reference/schemas/types/#reply_diseaseget), a shared type. `query` repeats the request as BioMCP read it.

### Reply_DiseaseSurvival

Native annual survival rows with separate observed and modeled values.

| Field | Type | Meaning |
| --- | --- | --- |
| `components` | list of [Reply_ComponentLabel](/reference/schemas/types/#reply_componentlabel) | Applicable component use terms, independent of row provenance. Optional. |
| `fetched` | integer or null | Original response retrieval time in Unix seconds, distinct from dataset publication. Optional. |
| `items` | list of [Reply_DiseaseStatistic](/reference/schemas/types/#reply_diseasestatistic) | Rows as the source gives them. Totals in `page` count rows, not distinct records. Optional. |

## Next

[`biomcp get disease`](/reference/commands/get-disease/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
