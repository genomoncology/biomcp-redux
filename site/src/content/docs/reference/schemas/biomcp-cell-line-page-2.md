---
title: Reply of biomcp search cell-line (biomcp.cell_line_page/2)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp search cell-line: a
  real example and the fields of schema biomcp.cell_line_page/2."
goal: "Parse the biomcp.cell_line_page/2 reply: see a real example and what each
  field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.cell_line_page/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search cell-line`](/reference/commands/search-cell-line/) replies with `schema` set to `biomcp.cell_line_page/2`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp search cell-line A-375 --limit 20
biomcp search cell-line MOLM13
```

`biomcp schema biomcp.cell_line_page/2` prints the full JSON Schema.

## Example reply

`biomcp search cell-line A-375 --limit 20` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.cell_line_page/2",
  "status": "complete",
  "query": {"query": "A-375", "limit": 20, "offset": 0},
  "result": {
    "items": [
      {
        "accession": "CVCL_F6DR",
        "name": "EDITGENE A-375-CopGFP",
        "category": "Cancer cell line",
        "species": {…},
        "diseases": {…}
      },
      … 19 more
    ],
    "release": {"version": "57.0", "updated": "2026-10-22"},
    "fetched": 1793140300
  },
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
      "kind": "disambiguate",
      "args": ["get", "cell-line", "CVCL_F6DR"]
    },
    … 20 more
  ]
}
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [CellLinePage](#celllinepage). `query` holds [CellLineSearch](#celllinesearch). `query` repeats the request as BioMCP read it.

### CellLinePage

Source-ordered cell line search headers; totals remain unknown.

| Field | Type | Meaning |
| --- | --- | --- |
| `fetched` | integer or null | Original retrieval seconds; not a source release date. Optional. |
| `items` | list of [CellLineHeader](/reference/schemas/types/#celllineheader) | Native result order, without reranking or selected identity. |
| `release` | [CellLineRelease](/reference/schemas/types/#celllinerelease) or null | Independently fetched publisher release, never a result total. Optional. |

### CellLineSearch

One literal cell line search, with native order and no total claim.

| Field | Type | Meaning |
| --- | --- | --- |
| `limit` | integer (uint32) | Native page size. Optional. |
| `offset` | integer (uint32) | Native start offset. Optional. |
| `query` | [Text](/reference/schemas/types/#text) | Literal source query; positional spelling and --query use the same field. |

## Next

[`biomcp search cell-line`](/reference/commands/search-cell-line/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
