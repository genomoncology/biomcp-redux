---
title: Reply of biomcp get pgx annotations section (biomcp.pgx_annotations/1)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get pgx annotations:
  a real example and the fields of schema biomcp.pgx_annotations/1."
goal: "Parse the biomcp.pgx_annotations/1 reply: see a real example and what
  each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.pgx_annotations/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get pgx annotations`](/reference/commands/get-pgx/) replies with `schema` set to `biomcp.pgx_annotations/1`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp get pgx --gene DPYD annotations --source clinpgx
```

`biomcp schema biomcp.pgx_annotations/1` prints the full JSON Schema.

## Example reply

`biomcp get pgx --gene DPYD annotations --source clinpgx` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.pgx_annotations/1",
  "status": "complete",
  "query": {
    "gene": "DPYD",
    "source": "clinpgx",
    "section": "annotations",
    "kind": "clinical",
    "section_limit": 3,
    "section_offset": 0
  },
  "result": {
    "annotationkind": "clinical",
    "header": {"source": "clinpgx", "gene": "DPYD"},
    "items": [
      {…},
      … 2 more
    ],
    "page": {"offset": 0, "returned": 3, "total": 83, "total_kind": "observed"}
  },
  "sources": [
    {
      "source": "clinpgx",
      "use": "noncommercial",
      "terms": "ClinPGx data use policy; noncommercial access restriction; ca…",
      "terms_url": "https://www.clinpgx.org/page/dataUsagePolicy",
      "state": "ok"
    }
  ],
  "next": [
    {
      "kind": "disambiguate",
      "args": [
        "get", "pgx", "--annotation-id", "PA166169507", "--source", "clinpgx"
      ]
    },
    … 3 more
  ]
}
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [PgxAnnotations](#pgxannotations). `query` holds [PgxGet](/reference/schemas/types/#pgxget), a shared type. `query` repeats the request as BioMCP read it.

### PgxAnnotations

One independent ClinPGx collection, preserving its native min-tier shape.

One of these forms:

- `annotationkind` is `"clinical"`: Complete clinical min rows including location and relationship arrays. Fields: `header` ([PgxHeader](/reference/schemas/types/#pgxheader)), `items` (list of [PgxAnnotationHeader](#pgxannotationheader)), `page` ([PgxPageInfo](/reference/schemas/types/#pgxpageinfo)).
- `annotationkind` is `"guideline"`: Guideline references without invented full guidance. Fields: `header` ([PgxHeader](/reference/schemas/types/#pgxheader)), `items` (list of [PgxAnnotationReference](#pgxannotationreference)), `page` ([PgxPageInfo](/reference/schemas/types/#pgxpageinfo)).
- `annotationkind` is `"label"`: Label references without invented full guidance. Fields: `header` ([PgxHeader](/reference/schemas/types/#pgxheader)), `items` (list of [PgxAnnotationReference](#pgxannotationreference)), `page` ([PgxPageInfo](/reference/schemas/types/#pgxpageinfo)).

### PgxAnnotationHeader

Source-native PgxAnnotationHeader fields; no clinical interpretation.

| Field | Type | Meaning |
| --- | --- | --- |
| `accessionId` | [ClinPgxAccession](/reference/schemas/types/#clinpgxaccession) | The source's own `accessionId` value, unchanged. |
| `id` | integer (int64) | The source's own `id` value, unchanged. |
| `levelOfEvidence` | [PgxEvidenceTerm](/reference/schemas/types/#pgxevidenceterm) | The source's own `levelOfEvidence` value, unchanged. |
| `location` | [PgxLocation](/reference/schemas/types/#pgxlocation) | The source's own `location` value, unchanged. |
| `name` | string | The source's own `name` value, unchanged. |
| `relatedChemicals` | list of [PgxObjectReference](/reference/schemas/types/#pgxobjectreference) | The source's own `relatedChemicals` value, unchanged. |
| `relatedChemicalsLogic` | string | The source's own `relatedChemicalsLogic` value, unchanged. |
| `relatedGuidelines` | list of [PgxObjectReference](/reference/schemas/types/#pgxobjectreference) | The source's own `relatedGuidelines` value, unchanged. |
| `relatedLabels` | list of [PgxObjectReference](/reference/schemas/types/#pgxobjectreference) | The source's own `relatedLabels` value, unchanged. |
| `types` | list of string | The source's own `types` value, unchanged. |

### PgxAnnotationReference

Source-native PgxAnnotationReference fields; no clinical interpretation.

| Field | Type | Meaning |
| --- | --- | --- |
| `crossReferences` | list of [PgxUnobserved](/reference/schemas/types/#pgxunobserved) | The source's own `crossReferences` value, unchanged. |
| `id` | [ClinPgxAccession](/reference/schemas/types/#clinpgxaccession) | The source's own `id` value, unchanged. |
| `name` | string | The source's own `name` value, unchanged. |
| `objCls` | string | The source's own `objCls` value, unchanged. |

## Next

[`biomcp get pgx`](/reference/commands/get-pgx/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
