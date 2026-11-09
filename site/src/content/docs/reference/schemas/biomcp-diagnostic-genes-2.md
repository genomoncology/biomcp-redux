---
title: Reply of biomcp get diagnostic genes section (biomcp.diagnostic_genes/2)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get diagnostic
  genes: a real example and the fields of schema biomcp.diagnostic_genes/2."
goal: "Parse the biomcp.diagnostic_genes/2 reply: see a real example and what
  each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.diagnostic_genes/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get diagnostic genes`](/reference/commands/get-diagnostic/) replies with `schema` set to `biomcp.diagnostic_genes/2`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp get diagnostic GTR000006692.3 genes --source gtr
```

`biomcp schema biomcp.diagnostic_genes/2` prints the full JSON Schema.

## Example reply

`biomcp get diagnostic GTR000006692.3 genes --source gtr` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.diagnostic_genes/2",
  "status": "complete",
  "query": {
    "id": "GTR000006692.3",
    "source": "gtr",
    "section": "genes",
    "section_offset": 0
  },
  "result": {
    "items": [
      {
        "test_id": "GTR000006692.3",
        "test_type": "Clinical",
        "gene": {"ncbi_gene_id": "672", "label": "BRCA1"},
        "native_medgen": "MedGen:C0376571",
        "mim_number": "113705",
        "object_name": "BRCA1:BRCA1 DNA repair associated"
      }
    ],
    "count_unit": "native_relation_rows",
    "distinct_id_total": 1
  },
  "page": {"offset": 0, "returned": 1, "total": 1},
  "sources": [
    {
      "source": "gtr",
      "use": "open",
      "terms": "NCBI usage policies; submitter and third-party rights retained",
      "terms_url": "https://www.ncbi.nlm.nih.gov/home/about/policies/",
      "state": "ok"
    }
  ],
  "next": [
    {
      "kind": "disambiguate",
      "args": ["get", "gene", "672"]
    }
  ]
}
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [DiagnosticGenes](#diagnosticgenes). `query` holds [DiagnosticGet](/reference/schemas/types/#diagnosticget), a shared type. `query` repeats the request as BioMCP read it.

### DiagnosticGenes

Native GTR gene relation rows without variant-coverage inference.

| Field | Type | Meaning |
| --- | --- | --- |
| `count_unit` | [DiagnosticCountUnit](/reference/schemas/types/#diagnosticcountunit) or null | Exact meaning of the envelope total; supplied by the owning source command. Optional. |
| `distinct_id_total` | integer or null | Distinct complete native IDs across the selected relation collection. Optional. |
| `items` | list of [GtrGeneRelation](/reference/schemas/types/#gtrgenerelation) | Complete source-owned rows in native order; envelope totals count native rows. |

## Next

[`biomcp get diagnostic`](/reference/commands/get-diagnostic/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
