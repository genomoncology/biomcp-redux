---
title: Reply of biomcp pathway events (biomcp.pathway_events/2)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp pathway events: a
  real example and the fields of schema biomcp.pathway_events/2."
goal: "Parse the biomcp.pathway_events/2 reply: see a real example and what each
  field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.pathway_events/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp pathway events`](/reference/commands/pathway-events/) replies with `schema` set to `biomcp.pathway_events/2`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp pathway events R-HSA-5673001
biomcp pathway events R-HSA-5673001 --limit 50
```

`biomcp schema biomcp.pathway_events/2` prints the full JSON Schema.

## Example reply

`biomcp pathway events R-HSA-5673001` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

```json
{
  "schema": "biomcp.pathway_events/2",
  "status": "complete",
  "query": {"id": "R-HSA-5673001", "limit": 5, "offset": 0},
  "result": {
    "header": {
      "id": {"source": "reactome", "id": "R-HSA-5673001"},
      "name": "RAF/MAP kinase cascade",
      "species": "Homo sapiens",
      "version": "R-HSA-5673001.12"
    },
    "items": [
      {
        "kind": "object",
        "db_id": 5675206,
        "id": "R-HSA-5675206",
        "source_url": "https://reactome.org/content/detail/R-HSA-5675206",
        "version": "R-HSA-5675206.2",
        "name": "PEA15 binds MAPK monomers and dimers",
        "schema_class": "Reaction"
      },
      {"kind": "database", "db_id": 5674385},
      … 3 more
    ],
    "page": {
      "offset": 0,
      "returned": 5,
      "total": 81,
      "total_kind": "observed",
      "next_offset": 5,
      "pool": {
        "id": "bc11fae544162020ea8498544c4ef476",
        "size": 81,
        "created": 1793140307,
        "expires": 1793226707
      }
    }
  },
  "sources": [
    {
      "source": "reactome",
      "use": "open",
      "terms": "CC0 1.0 database data",
      "terms_url": "https://reactome.org/license",
      "state": "ok"
    }
  ],
  "next": [
    {
      "kind": "page",
      "args": [
        "pathway", "events", "R-HSA-5673001", "--offset", "5", "--pool",
        "bc11fae544162020ea8498544c4ef476"
      ]
    }
  ]
}
```

## Fields

`result` holds [PathwayEvents](/reference/schemas/types/#pathwayevents), a shared type. `query` holds [PathwayEventsQuery](#pathwayeventsquery). `query` repeats the request as BioMCP read it.

### PathwayEventsQuery

Read Reactome event objects and unresolved native integer references in source order. Membership names a source and release; it implies no therapy, response or actionability. No enrichment is calculated.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `id` | string | yes | Checked source-owned human pathway identity. |
| `limit` | integer (uint32) | no | Five complete relation rows by default, at most fifty. |
| `offset` | integer (uint32) | no | Offset within the retained selected relation. |
| `pool` | [Text](/reference/schemas/types/#text) | no | Immutable relation snapshot. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp pathway events`](/reference/commands/pathway-events/) gives the command's arguments.
