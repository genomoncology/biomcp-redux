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

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search pathway`](/reference/commands/search-pathway/) replies with `schema` set to `biomcp.pathway_page/3`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp search pathway MAPK --source wikipathways --limit 3
biomcp search pathway MAPK --limit 3
```

`biomcp schema biomcp.pathway_page/3` prints the full JSON Schema.

## Example reply

`biomcp search pathway MAPK --source wikipathways --limit 3` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

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
      {
        "id": {"source": "wikipathways", "id": "WP382"},
        "name": "MAPK signaling",
        "species": "Homo sapiens",
        "revision": "2025-08-30"
      },
      {
        "id": {"source": "wikipathways", "id": "WP3849"},
        "name": "MAPK and NFkB signaling inhibited by Yersinia YopJ",
        "species": "Homo sapiens",
        "revision": "2025-03-03"
      },
      … 1 more
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
    {
      "kind": "disambiguate",
      "args": ["get", "pathway", "WP382"]
    },
    {
      "kind": "disambiguate",
      "args": ["get", "pathway", "WP3849"]
    },
    … 2 more
  ]
}
```

## Fields

`result` holds [PathwayPage](#pathwaypage). `query` holds [PathwaySearch](#pathwaysearch). `query` repeats the request as BioMCP read it.

### PathwayPage

Source-ordered header page; class admission never changes the native total.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `items` | list of [PathwayHeader](/reference/schemas/types/#pathwayheader) | yes | Whole admitted pathway headers in native order. |
| `page` | [PathwayPageInfo](/reference/schemas/types/#pathwaypageinfo) | yes | Native position and total provenance. |

### PathwaySearch

Search Reactome (default) or WikiPathways human pathways. Membership names a source and release; it implies no therapy, response or actionability. No enrichment is calculated.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `fresh` | boolean | no | Request a new retained observation. |
| `gene` | [GeneInput](/reference/schemas/types/#geneinput) | no | Human gene input for exact resolution and UniProt mapping; Reactome only. |
| `limit` | integer (uint32) | no | Five rows by default, one to fifty. Reactome rows equals this limit without refill. |
| `offset` | integer (uint32) | no | Native Reactome Start row or retained WikiPathways catalog offset. |
| `pool` | [Text](/reference/schemas/types/#text) | no | Retained snapshot identity for deterministic continuation. |
| `query` | [Text](/reference/schemas/types/#text) | no | Literal source query; exclusive with gene and top-level discovery. |
| `source` | list of [PathwaySource](#pathwaysource) | no | Sources in caller order; repeated sources take turns without title joins. |
| `top_level` | boolean | no | Native human Reactome top-level discovery, exclusive with text and gene. |

### PathwaySource

One explicit pathway source, without fallback or cross-source title joins.

Values: `reactome`, `wikipathways`.

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp search pathway`](/reference/commands/search-pathway/) gives the command's arguments.
