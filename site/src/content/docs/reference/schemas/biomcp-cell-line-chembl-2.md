---
title: Reply of biomcp get cell-line chembl section (biomcp.cell_line_chembl/2)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get cell-line
  chembl: a real example and the fields of schema biomcp.cell_line_chembl/2."
goal: "Parse the biomcp.cell_line_chembl/2 reply: see a real example and what
  each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.cell_line_chembl/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get cell-line chembl`](/reference/commands/get-cell-line/) replies with `schema` set to `biomcp.cell_line_chembl/2`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp get cell-line CVCL_0132 chembl
```

`biomcp schema biomcp.cell_line_chembl/2` prints the full JSON Schema.

## Example reply

`biomcp get cell-line CVCL_0132 chembl` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

```json
{
  "schema": "biomcp.cell_line_chembl/2",
  "status": "complete",
  "query": {
    "id": "CVCL_0132",
    "section": "chembl",
    "section_offset": 0
  },
  "result": {
    "release": {"version": "ChEMBL_37", "updated": "2026-05-01"},
    "fetched": 1793140298,
    "accession": "CVCL_0132",
    "items": {
      "items": [
        {
          "id": "CHEMBL3308077",
          "cell_id": 455,
          "cellosaurus_id": "CVCL_0132",
          "name": "A-375",
          "description": "A-375 (Malignant melanoma cells)",
          "organism": "Homo sapiens",
          "taxon": 9606,
          "tissue": "Malignant melanoma cells",
          "clo": "CLO_0001582",
          "efo": "EFO_0002103"
        }
      ],
      "total": 1
    },
    "offset": 0
  },
  "sources": [
    {
      "source": "chembl",
      "use": "attribution",
      "terms": "ChEMBL CC BY-SA 3.0",
      "terms_url": "https://chembl.github.io/chembl-licensing/",
      "state": "ok"
    }
  ],
  "next": [
    {
      "kind": "section",
      "args": ["cell-line", "assay-count", "CHEMBL3308077"]
    }
  ]
}
```

## Fields

`result` holds [CellLineChembl](/reference/schemas/types/#celllinechembl) of [Capped](/reference/schemas/types/#capped) of [CellLineChemblRecord](/reference/schemas/types/#celllinechemblrecord), a shared type. `query` holds [CellLineGet](/reference/schemas/types/#celllineget), a shared type. `query` repeats the request as BioMCP read it.

Every type this reply uses is shared with other replies, so its fields are on the shared types page.

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp get cell-line`](/reference/commands/get-cell-line/) gives the command's arguments.
