---
title: Reply of biomcp get cell-line xrefs section (biomcp.cell_line_xrefs/1)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get cell-line xrefs:
  a real example and the fields of schema biomcp.cell_line_xrefs/1."
goal: "Parse the biomcp.cell_line_xrefs/1 reply: see a real example and what
  each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.cell_line_xrefs/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get cell-line xrefs`](/reference/commands/get-cell-line/) replies with `schema` set to `biomcp.cell_line_xrefs/1`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp get cell-line CVCL_2119 xrefs
```

`biomcp schema biomcp.cell_line_xrefs/1` prints the full JSON Schema.

## Example reply

`biomcp get cell-line CVCL_2119 xrefs` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

```json
{
  "schema": "biomcp.cell_line_xrefs/1",
  "status": "complete",
  "query": {
    "id": "CVCL_2119",
    "section": "xrefs",
    "section_offset": 0
  },
  "result": {
    "accession": "CVCL_2119",
    "items": {
      "items": [
        {
          "database": "BTO",
          "accession": "BTO_0004175",
          "url": "https://www.ebi.ac.uk/ols4/ontologies/bto/classes/http%253A…",
          "category": "Anatomy/cell type resources",
          "iri": "http://purl.obolibrary.org/obo/BTO_0004175"
        },
        {
          "database": "CLO",
          "accession": "CLO_0009848",
          "url": "https://www.ebi.ac.uk/ols4/ontologies/clo/classes/http%253A…",
          "category": "Cell line databases/resources",
          "iri": "http://purl.obolibrary.org/obo/CLO_0009848"
        },
        … 3 more
      ],
      "total": 71
    },
    "offset": 0,
    "release": {"version": "57.0", "updated": "2026-10-22"},
    "fetched": 1793140300
  },
  "page": {"offset": 0, "returned": 5, "total": 71},
  "sources": [
    {
      "source": "cellosaurus",
      "use": "open",
      "terms": "Cellosaurus CC BY 4.0; cite Bairoch (2018)",
      "terms_url": "https://www.cellosaurus.org/description.html",
      "state": "ok"
    }
  ],
  "next": [
    {
      "kind": "page",
      "args": [
        "get", "cell-line", "CVCL_2119", "xrefs", "--section-offset", "5"
      ]
    }
  ]
}
```

## Fields

`result` holds [CellLineXrefs](/reference/schemas/types/#celllinexrefs), a shared type. `query` holds [CellLineGet](/reference/schemas/types/#celllineget), a shared type. `query` repeats the request as BioMCP read it.

Every type this reply uses is shared with other replies, so its fields are on the shared types page.

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp get cell-line`](/reference/commands/get-cell-line/) gives the command's arguments.
