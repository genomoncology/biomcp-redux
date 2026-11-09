---
title: Reply of biomcp get pgx (biomcp.pgx-recommendations/5)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get pgx: the fields
  of schema biomcp.pgx-recommendations/5."
goal: "Parse the biomcp.pgx-recommendations/5 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.pgx-recommendations/5
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get pgx`](/reference/commands/get-pgx/) and [`biomcp get`](/reference/commands/get/) reply with `schema` set to `biomcp.pgx-recommendations/5`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.pgx-recommendations/5` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_PgxRecommendations](#reply-pgxrecommendations). `query` holds [Reply_PgxGet](/reference/schemas/types/#reply-pgxget), a shared type. `query` repeats the request as BioMCP read it.

### Reply_PgxRecommendations

Independent recommendation contracts preserve native table and view shapes.

One of these forms:

- `representation` is `"view"`: Gene-keyed recommendation_view with native names and no inferred drug ID. Fields: `header` ([Reply_PgxHeader](/reference/schemas/types/#reply-pgxheader)), `items` (list of [Reply_PgxRecommendation](#reply-pgxrecommendation)), `page` ([Reply_PgxPageInfo](/reference/schemas/types/#reply-pgxpageinfo)).
- `representation` is `"table"`: Exact drug selector uses the distinct native recommendation table. Fields: `header` ([Reply_PgxHeader](/reference/schemas/types/#reply-pgxheader)), `items` (list of [Reply_PgxDrugRecommendation](#reply-pgxdrugrecommendation)), `page` ([Reply_PgxPageInfo](/reference/schemas/types/#reply-pgxpageinfo)).

### Other types in this reply

2 more types appear only in this reply: [Reply_PgxDrugRecommendation](#reply-pgxdrugrecommendation), [Reply_PgxRecommendation](#reply-pgxrecommendation). Open the list below to read their fields.

<details>
<summary>Fields of the 2 other types</summary>

### Reply_PgxDrugRecommendation

Source-native PgxDrugRecommendation fields; no clinical interpretation.

| Field | Type | Meaning |
| --- | --- | --- |
| `activityscore` | object or null | The source's own `activityscore` value, unchanged. Optional. |
| `allelestatus` | object or null | The source's own `allelestatus` value, unchanged. Optional. |
| `alternatedrugavailable` | boolean or null | The source's own `alternatedrugavailable` value, unchanged. Optional. |
| `classification` | string or null | The source's own `classification` value, unchanged. Optional. |
| `comments` | string or null | The source's own `comments` value, unchanged. Optional. |
| `dosinginformation` | boolean or null | The source's own `dosinginformation` value, unchanged. Optional. |
| `drugid` | [Reply_Curie](/reference/schemas/types/#reply-curie) | The source's own `drugid` value, unchanged. |
| `drugrecommendation` | string or null | The source's own `drugrecommendation` value, unchanged. Optional. |
| `guidelineid` | [Reply_Identifier_int64](/reference/schemas/types/#reply-identifier-int64) or null | The source's own `guidelineid` value, unchanged. Optional. |
| `id` | [Reply_Identifier_int64](/reference/schemas/types/#reply-identifier-int64) | The source's own `id` value, unchanged. |
| `implications` | object or null | The source's own `implications` value, unchanged. Optional. |
| `lookupkey` | object | The source's own `lookupkey` value, unchanged. |
| `otherprescribingguidance` | boolean or null | The source's own `otherprescribingguidance` value, unchanged. Optional. |
| `phenotypes` | object or null | The source's own `phenotypes` value, unchanged. Optional. |
| `population` | string or null | The source's own `population` value, unchanged. Optional. |
| `version` | integer or null | The source's own `version` value, unchanged. Optional. |

### Reply_PgxRecommendation

Source-native PgxRecommendation fields; no clinical interpretation.

| Field | Type | Meaning |
| --- | --- | --- |
| `activityscore` | object or null | The source's own `activityscore` value, unchanged. Optional. |
| `classification` | string or null | The source's own `classification` value, unchanged. Optional. |
| `comments` | string or null | The source's own `comments` value, unchanged. Optional. |
| `drugname` | string or null | The source's own `drugname` value, unchanged. Optional. |
| `drugrecommendation` | string or null | The source's own `drugrecommendation` value, unchanged. Optional. |
| `guidelinename` | string or null | The source's own `guidelinename` value, unchanged. Optional. |
| `guidelineurl` | string or null | The source's own `guidelineurl` value, unchanged. Optional. |
| `implications` | object or null | The source's own `implications` value, unchanged. Optional. |
| `lookupkey` | object | The source's own `lookupkey` value, unchanged. |
| `phenotypes` | object or null | The source's own `phenotypes` value, unchanged. Optional. |
| `population` | string or null | The source's own `population` value, unchanged. Optional. |
| `recommendationid` | [Reply_Identifier_int64](/reference/schemas/types/#reply-identifier-int64) | The source's own `recommendationid` value, unchanged. |

</details>

## Next

[`biomcp get pgx`](/reference/commands/get-pgx/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
