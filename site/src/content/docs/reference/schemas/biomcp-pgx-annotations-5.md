---
title: Reply of biomcp get pgx (biomcp.pgx-annotations/5)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get pgx: the fields
  of schema biomcp.pgx-annotations/5."
goal: "Parse the biomcp.pgx-annotations/5 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.pgx-annotations/5
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get pgx`](/reference/commands/get-pgx/) and [`biomcp get`](/reference/commands/get/) reply with `schema` set to `biomcp.pgx-annotations/5`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.pgx-annotations/5` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_PgxAnnotations](#reply_pgxannotations). `query` holds [Reply_PgxGet](/reference/schemas/types/#reply_pgxget), a shared type. `query` repeats the request as BioMCP read it.

### Reply_PgxAnnotations

One independent ClinPGx collection, preserving its native min-tier shape.

One of these forms:

- `annotation_kind` is `"clinical"`: Complete clinical min rows including location and relationship arrays. Fields: `header` ([Reply_PgxHeader](/reference/schemas/types/#reply_pgxheader)), `items` (list of [Reply_PgxAnnotationHeader](#reply_pgxannotationheader)), `page` ([Reply_PgxPageInfo](/reference/schemas/types/#reply_pgxpageinfo)).
- `annotation_kind` is `"guideline"`: Guideline references without invented full guidance. Fields: `header` ([Reply_PgxHeader](/reference/schemas/types/#reply_pgxheader)), `items` (list of [Reply_PgxAnnotationReference](#reply_pgxannotationreference)), `page` ([Reply_PgxPageInfo](/reference/schemas/types/#reply_pgxpageinfo)).
- `annotation_kind` is `"label"`: Label references without invented full guidance. Fields: `header` ([Reply_PgxHeader](/reference/schemas/types/#reply_pgxheader)), `items` (list of [Reply_PgxAnnotationReference](#reply_pgxannotationreference)), `page` ([Reply_PgxPageInfo](/reference/schemas/types/#reply_pgxpageinfo)).

### Other types in this reply

2 more types appear only in this reply: [Reply_PgxAnnotationHeader](#reply_pgxannotationheader), [Reply_PgxAnnotationReference](#reply_pgxannotationreference). Open the list below to read their fields.

<details>
<summary>Fields of the 2 other types</summary>

### Reply_PgxAnnotationHeader

Source-native PgxAnnotationHeader fields; no clinical interpretation.

| Field | Type | Meaning |
| --- | --- | --- |
| `accessionId` | [Reply_ClinPgxAccession](/reference/schemas/types/#reply_clinpgxaccession) | The source's own `accessionId` value, unchanged. |
| `id` | [Reply_Identifier_int64](/reference/schemas/types/#reply_identifier_int64) | The source's own `id` value, unchanged. |
| `levelOfEvidence` | [Reply_PgxEvidenceTerm](/reference/schemas/types/#reply_pgxevidenceterm) or null | The source's own `levelOfEvidence` value, unchanged. Optional. |
| `location` | [Reply_PgxLocation](/reference/schemas/types/#reply_pgxlocation) or null | The source's own `location` value, unchanged. Optional. |
| `name` | string or null | The source's own `name` value, unchanged. Optional. |
| `relatedChemicals` | list of [Reply_PgxObjectReference](/reference/schemas/types/#reply_pgxobjectreference) | The source's own `relatedChemicals` value, unchanged. Optional. |
| `relatedChemicalsLogic` | string or null | The source's own `relatedChemicalsLogic` value, unchanged. Optional. |
| `relatedGuidelines` | list of [Reply_PgxObjectReference](/reference/schemas/types/#reply_pgxobjectreference) | The source's own `relatedGuidelines` value, unchanged. Optional. |
| `relatedLabels` | list of [Reply_PgxObjectReference](/reference/schemas/types/#reply_pgxobjectreference) | The source's own `relatedLabels` value, unchanged. Optional. |
| `types` | list of string | The source's own `types` value, unchanged. Optional. |

### Reply_PgxAnnotationReference

Source-native PgxAnnotationReference fields; no clinical interpretation.

| Field | Type | Meaning |
| --- | --- | --- |
| `crossReferences` | list of [Reply_PgxUnobserved](/reference/schemas/types/#reply_pgxunobserved) | The source's own `crossReferences` value, unchanged. Optional. |
| `id` | [Reply_ClinPgxAccession](/reference/schemas/types/#reply_clinpgxaccession) | The source's own `id` value, unchanged. |
| `name` | string or null | The source's own `name` value, unchanged. Optional. |
| `objCls` | string | The source's own `objCls` value, unchanged. |

</details>

## Next

[`biomcp get pgx`](/reference/commands/get-pgx/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
