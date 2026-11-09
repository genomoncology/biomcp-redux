---
title: Reply of biomcp get disease (biomcp.disease-genes/9)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get disease: the
  fields of schema biomcp.disease-genes/9."
goal: "Parse the biomcp.disease-genes/9 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.disease-genes/9
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get disease`](/reference/commands/get-disease/) and [`biomcp get`](/reference/commands/get/) reply with `schema` set to `biomcp.disease-genes/9`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.disease-genes/9` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_DiseaseGenes](#reply_diseasegenes). `query` holds [Reply_DiseaseGet](/reference/schemas/types/#reply_diseaseget), a shared type. `query` repeats the request as BioMCP read it.

### Reply_DiseaseGenes

Native association rows, not a unique-gene count.

| Field | Type | Meaning |
| --- | --- | --- |
| `components` | list of [Reply_ComponentLabel](/reference/schemas/types/#reply_componentlabel) | Applicable component use terms, independent of row provenance. Optional. |
| `fetched` | integer or null | Original response retrieval Unix seconds, independent of local delivery. Optional. |
| `items` | list of [Reply_DiseaseAssociationItem](/reference/schemas/types/#reply_diseaseassociationitem) | Rows as the source gives them. Totals in `page` count rows, not distinct records. Optional. |

## Next

[`biomcp get disease`](/reference/commands/get-disease/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
