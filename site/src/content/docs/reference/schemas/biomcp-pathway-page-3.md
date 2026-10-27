---
title: Reply of biomcp search pathway (biomcp.pathway_page/3)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp search pathway: a
  real example and the fields of schema biomcp.pathway_page/3."
goal: "Parse the biomcp.pathway_page/3 reply: see a real example and what each
  field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.pathway_page/3
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search pathway`](/reference/commands/search-pathway/) replies with `schema` set to `biomcp.pathway_page/3`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp search pathway MAPK --source wikipathways --limit 3
biomcp search pathway MAPK --limit 3
```

`biomcp schema biomcp.pathway_page/3` prints the full JSON Schema.

## Example reply

`biomcp search pathway MAPK --source wikipathways --limit 3` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.pathway_page/3",
  "status": "complete",
  "query": {
    "query": "MAPK",
    "top_level": false,
    "source": ["wikipathways"],
    "limit": 3,
    "offset": 0,
    "fresh": false
  },
  "result": {
    "items": [
      {…},
      … 2 more
    ],
    "page": {
      "offset": 0,
      "returned": 3,
      "total": 7,
      "total_kind": "observed",
      "next_offset": 3
    }
  },
  "sources": [
    {
      "source": "wikipathways",
      "use": "open",
      "terms": "CC0 1.0 waiver",
      "terms_url": "https://www.wikipathways.org/help.html",
      "state": "ok"
    }
  ],
  "next": [
    {"kind": "disambiguate", "args": ["get", "pathway", "WP382"]},
    … 3 more
  ]
}
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [PathwayPage](#pathwaypage). `query` holds [PathwaySearch](#pathwaysearch). `query` repeats the request as BioMCP read it.

### PathwayPage

Source-ordered header page; class admission never changes the native total.

| Field | Type | Meaning |
| --- | --- | --- |
| `items` | list of [PathwayHeader](/reference/schemas/types/#pathwayheader) | Whole admitted pathway headers in native order. |
| `page` | [PathwayPageInfo](/reference/schemas/types/#pathwaypageinfo) | Native position and total provenance. |

### PathwaySearch

Search Reactome (default) or WikiPathways human pathways. Membership names a source and release; it implies no therapy, response or actionability. No enrichment is calculated.

| Field | Type | Meaning |
| --- | --- | --- |
| `fresh` | boolean | Request a new retained observation. Optional. |
| `gene` | [GeneInput](/reference/schemas/types/#geneinput) | Human gene input for exact resolution and UniProt mapping; Reactome only. Optional. |
| `limit` | integer (uint32) | Five rows by default, one to fifty. Reactome rows equals this limit without refill. Optional. |
| `offset` | integer (uint32) | Native Reactome Start row or retained WikiPathways catalog offset. Optional. |
| `pool` | [Text](/reference/schemas/types/#text) | Retained snapshot identity for deterministic continuation. Optional. |
| `query` | [Text](/reference/schemas/types/#text) | Literal source query; exclusive with gene and top-level discovery. Optional. |
| `source` | list of [PathwaySource](#pathwaysource) | Sources in caller order; repeated sources take turns without title joins. Optional. |
| `top_level` | boolean | Native human Reactome top-level discovery, exclusive with text and gene. Optional. |

### Other types in this reply

One more type appear only in this reply: [PathwaySource](#pathwaysource). Open the list below to read their fields.

<details>
<summary>Fields of the 1 other type</summary>

### PathwaySource

One explicit pathway source, without fallback or cross-source title joins.

Values: `reactome`, `wikipathways`.

</details>

## Next

[`biomcp search pathway`](/reference/commands/search-pathway/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
