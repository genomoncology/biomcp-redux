---
title: Reply of biomcp get pgx recommendations section (biomcp.pgx_recommendations/1)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get pgx
  recommendations: a real example and the fields of schema
  biomcp.pgx_recommendations/1."
goal: "Parse the biomcp.pgx_recommendations/1 reply: see a real example and what
  each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.pgx_recommendations/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get pgx recommendations`](/reference/commands/get-pgx/) replies with `schema` set to `biomcp.pgx_recommendations/1`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp get pgx --gene DPYD --source cpic recommendations
```

`biomcp schema biomcp.pgx_recommendations/1` prints the full JSON Schema.

## Example reply

`biomcp get pgx --gene DPYD --source cpic recommendations` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.pgx_recommendations/1",
  "status": "complete",
  "query": {
    "gene": "DPYD",
    "source": "cpic",
    "section": "recommendations",
    "section_limit": 5,
    "section_offset": 0
  },
  "result": {
    "representation": "view",
    "header": {"source": "cpic", "gene": "DPYD"},
    "items": [
      {…},
      … 4 more
    ],
    "page": {"offset": 0, "returned": 5, "total": 10, "total_kind": "source"}
  },
  "sources": [
    {
      "source": "cpic",
      "use": "noncommercial",
      "terms": "ClinPGx data use policy; noncommercial access restriction; ca…",
      "terms_url": "https://www.clinpgx.org/page/dataUsagePolicy",
      "state": "ok"
    }
  ],
  "next": [
    {
      "kind": "page",
      "args": [
        "get", "pgx", "recommendations", "--gene", "DPYD", "--section-offset",
        "5", "--source",
        … 1 more
      ]
    }
  ]
}
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [PgxRecommendations](#pgxrecommendations). `query` holds [PgxGet](/reference/schemas/types/#pgxget), a shared type. `query` repeats the request as BioMCP read it.

### PgxRecommendations

Independent recommendation contracts preserve native table and view shapes.

One of these forms:

- `representation` is `"view"`: Gene-keyed recommendation_view with native names and no inferred drug ID. Fields: `header` ([PgxHeader](/reference/schemas/types/#pgxheader)), `items` (list of [PgxRecommendation](#pgxrecommendation)), `page` ([PgxPageInfo](/reference/schemas/types/#pgxpageinfo)).
- `representation` is `"table"`: Exact drug selector uses the distinct native recommendation table. Fields: `header` ([PgxHeader](/reference/schemas/types/#pgxheader)), `items` (list of [PgxDrugRecommendation](#pgxdrugrecommendation)), `page` ([PgxPageInfo](/reference/schemas/types/#pgxpageinfo)).

### PgxDrugRecommendation

Source-native PgxDrugRecommendation fields; no clinical interpretation.

| Field | Type | Meaning |
| --- | --- | --- |
| `activityscore` | object | The source's own `activityscore` value, unchanged. |
| `allelestatus` | object | The source's own `allelestatus` value, unchanged. |
| `alternatedrugavailable` | boolean | The source's own `alternatedrugavailable` value, unchanged. |
| `classification` | string | The source's own `classification` value, unchanged. |
| `comments` | string | The source's own `comments` value, unchanged. |
| `dosinginformation` | boolean | The source's own `dosinginformation` value, unchanged. |
| `drugid` | [Curie](/reference/schemas/types/#curie) | The source's own `drugid` value, unchanged. |
| `drugrecommendation` | string | The source's own `drugrecommendation` value, unchanged. |
| `guidelineid` | integer (int64) | The source's own `guidelineid` value, unchanged. |
| `id` | integer (int64) | The source's own `id` value, unchanged. |
| `implications` | object | The source's own `implications` value, unchanged. |
| `lookupkey` | object | The source's own `lookupkey` value, unchanged. |
| `otherprescribingguidance` | boolean | The source's own `otherprescribingguidance` value, unchanged. |
| `phenotypes` | object | The source's own `phenotypes` value, unchanged. |
| `population` | string | The source's own `population` value, unchanged. |
| `version` | integer (int64) | The source's own `version` value, unchanged. |

### PgxRecommendation

Source-native PgxRecommendation fields; no clinical interpretation.

| Field | Type | Meaning |
| --- | --- | --- |
| `activityscore` | object | The source's own `activityscore` value, unchanged. |
| `classification` | string | The source's own `classification` value, unchanged. |
| `comments` | string or null | The source's own `comments` value, unchanged. Optional. |
| `drugname` | string | The source's own `drugname` value, unchanged. |
| `drugrecommendation` | string | The source's own `drugrecommendation` value, unchanged. |
| `guidelinename` | string | The source's own `guidelinename` value, unchanged. |
| `guidelineurl` | string | The source's own `guidelineurl` value, unchanged. |
| `implications` | object | The source's own `implications` value, unchanged. |
| `lookupkey` | object | The source's own `lookupkey` value, unchanged. |
| `phenotypes` | object | The source's own `phenotypes` value, unchanged. |
| `population` | string | The source's own `population` value, unchanged. |
| `recommendationid` | integer (int64) | The source's own `recommendationid` value, unchanged. |

## Next

[`biomcp get pgx`](/reference/commands/get-pgx/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
