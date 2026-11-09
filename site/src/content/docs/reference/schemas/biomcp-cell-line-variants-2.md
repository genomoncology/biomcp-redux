---
title: Reply of biomcp get cell-line variants section (biomcp.cell_line_variants/2)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get cell-line
  variants: a real example and the fields of schema
  biomcp.cell_line_variants/2."
goal: "Parse the biomcp.cell_line_variants/2 reply: see a real example and what
  each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.cell_line_variants/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get cell-line variants`](/reference/commands/get-cell-line/) replies with `schema` set to `biomcp.cell_line_variants/2`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp get cell-line CVCL_0132 variants
```

`biomcp schema biomcp.cell_line_variants/2` prints the full JSON Schema.

## Example reply

`biomcp get cell-line CVCL_0132 variants` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.cell_line_variants/2",
  "status": "complete",
  "query": {
    "id": "CVCL_0132",
    "section": "variants",
    "section_offset": 0
  },
  "result": {
    "accession": "CVCL_0132",
    "items": {
      "items": […],
      "total": 4
    },
    "offset": 0,
    "release": {"version": "57.0", "updated": "2026-10-22"},
    "fetched": 1793140300
  },
  "page": {"offset": 0, "returned": 3, "total": 4},
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
        "get", "cell-line", "CVCL_0132", "variants", "--section-offset", "3"
      ]
    },
    … 2 more
  ]
}
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [CellLineVariants](/reference/schemas/types/#celllinevariants) of [Capped](/reference/schemas/types/#capped) of [CellLineVariation](/reference/schemas/types/#celllinevariation), a shared type. `query` holds [CellLineGet](/reference/schemas/types/#celllineget), a shared type. `query` repeats the request as BioMCP read it.

Every type this reply uses is shared with other replies, so its fields are on the shared types page.

## Next

[`biomcp get cell-line`](/reference/commands/get-cell-line/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
