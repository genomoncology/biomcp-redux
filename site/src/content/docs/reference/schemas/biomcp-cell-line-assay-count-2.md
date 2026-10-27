---
title: Reply of biomcp cell-line assay-count (biomcp.cell_line_assay_count/2)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp cell-line
  assay-count: a real example and the fields of schema
  biomcp.cell_line_assay_count/2."
goal: "Parse the biomcp.cell_line_assay_count/2 reply: see a real example and
  what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.cell_line_assay_count/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp cell-line assay-count`](/reference/commands/cell-line-assay-count/) replies with `schema` set to `biomcp.cell_line_assay_count/2`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp cell-line assay-count CHEMBL3308077
biomcp cell-line assay-count CHEMBL3706573 --fresh
```

`biomcp schema biomcp.cell_line_assay_count/2` prints the full JSON Schema.

## Example reply

`biomcp cell-line assay-count CHEMBL3308077` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

```json
{
  "schema": "biomcp.cell_line_assay_count/2",
  "status": "complete",
  "query": {"id": "CHEMBL3308077"},
  "result": {
    "release": {"version": "ChEMBL_37", "updated": "2026-05-01"},
    "fetched": 1793140297,
    "id": "CHEMBL3308077",
    "assay_count": 2771
  },
  "sources": [
    {
      "source": "chembl",
      "use": "attribution",
      "terms": "ChEMBL CC BY-SA 3.0",
      "terms_url": "https://chembl.github.io/chembl-licensing/",
      "state": "ok"
    }
  ]
}
```

## Fields

`result` holds [CellLineAssayCount](#celllineassaycount). `query` holds [CellLineAssayCountQuery](#celllineassaycountquery). `query` repeats the request as BioMCP read it.

### CellLineAssayCount

Native assay total for one source-local cell identity; no activity or sensitivity count.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `assay_count` | integer (uint64) | yes | Native assay page total_count, including a reported zero. |
| `fetched` | integer or null | no | Original retrieval seconds; never a source release date. |
| `id` | [ChemblCellId](/reference/schemas/types/#chemblcellid) | yes | Exact requested ChEMBL cell ID. |
| `release` | [CellLineRelease](/reference/schemas/types/#celllinerelease) or null | no | Independent ChEMBL release, distinct from retrieval time. |

### CellLineAssayCountQuery

A separate assay-count request, never implicit in metadata or all.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `id` | [ChemblCellId](/reference/schemas/types/#chemblcellid) | yes | Exact ChEMBL cell identity returned by metadata. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp cell-line assay-count`](/reference/commands/cell-line-assay-count/) gives the command's arguments.
