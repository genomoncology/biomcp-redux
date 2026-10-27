---
title: Reply of biomcp get author (biomcp.author_card/3)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get author: a real
  example and the fields of schema biomcp.author_card/3."
goal: "Parse the biomcp.author_card/3 reply: see a real example and what each
  field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.author_card/3
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get author`](/reference/commands/get-author/) replies with `schema` set to `biomcp.author_card/3`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp get author orcid:0000-0002-1825-0097
biomcp get author semantic-scholar:123 semantic-scholar:123
```

`biomcp schema biomcp.author_card/3` prints the full JSON Schema.

## Example reply

`biomcp get author orcid:0000-0002-1825-0097` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.author_card/3",
  "status": "complete",
  "query": {"ids": ["orcid:0000-0002-1825-0097"]},
  "result": {
    "items": [
      {
        "input": "orcid:0000-0002-1825-0097",
        "status": "complete",
        "card": {
          "header": {
            "id": "orcid:0000-0002-1825-0097",
            "name": "Josiah Carberry"
          },
          "url": "https://orcid.org/0000-0002-1825-0097"
        }
      }
    ]
  },
  "sources": [
    {
      "source": "orcid",
      "use": "noncommercial",
      "terms": "ORCID Public API Terms of Service",
      "terms_url": "https://info.orcid.org/public-client-terms-of-service/",
      "state": "ok"
    }
  ],
  "next": [
    {
      "kind": "pivot",
      "args": ["author", "papers", "orcid:0000-0002-1825-0097"]
    }
  ]
}
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [AuthorCards](#authorcards). `query` holds [AuthorGet](#authorget). `query` repeats the request as BioMCP read it.

### AuthorCards

Exact source cards in input order; outer source outcomes retain failures.

| Field | Type | Meaning |
| --- | --- | --- |
| `items` | list of [AuthorCardItem](#authorcarditem) | Exact returned source cards, with no cross-source links. |

### AuthorGet

Exact qualified records in caller order, with no name reconciliation.

| Field | Type | Meaning |
| --- | --- | --- |
| `ids` | list of [AuthorId](/reference/schemas/types/#authorid) | One or more exact qualified IDs. |

### AuthorCard

Exact source author metadata without identity reconciliation.

| Field | Type | Meaning |
| --- | --- | --- |
| `header` | [AuthorHeader](/reference/schemas/types/#authorheader) | Exact source candidate. |
| `url` | [Text](/reference/schemas/types/#text) or null | Provider record link. Optional. |

### AuthorCardItem

One exact input and its independent outcome.

| Field | Type | Meaning |
| --- | --- | --- |
| `card` | [AuthorCard](#authorcard) or null | Exact checked source card when available. Optional. |
| `input` | [AuthorId](/reference/schemas/types/#authorid) | Requested qualified provider identity. |
| `status` | [Status](/reference/schemas/types/#status) | This exact input's outcome. |

## Next

[`biomcp get author`](/reference/commands/get-author/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
