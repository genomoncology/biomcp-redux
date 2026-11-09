---
title: Reply of biomcp get gwas (biomcp.gwas_association/1)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get gwas: a real
  example and the fields of schema biomcp.gwas_association/1."
goal: "Parse the biomcp.gwas_association/1 reply: see a real example and what
  each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.gwas_association/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get gwas`](/reference/commands/get-gwas/) replies with `schema` set to `biomcp.gwas_association/1`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp get gwas association:6114
```

`biomcp schema biomcp.gwas_association/1` prints the full JSON Schema.

## Example reply

`biomcp get gwas association:6114` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.gwas_association/1",
  "status": "complete",
  "query": {
    "kind": "association",
    "id": "association:6114",
    "section_limit": 3,
    "section_offset": 0
  },
  "result": {
    "association": {
      "id": 6114,
      "study": "GCST000035",
      "traits": ["Breast cancer"],
      "ontology_traits": [{"id": "MONDO_0004989", "label": "breast carcinoma"}],
      "alleles": [{"rs_id": "rs3817198", "allele": "C"}],
      "p_value": 3.0000000000000004e-9,
      "exact_p_value": {"mantissa": "3", "exponent": -9},
      "author": "Easton DF",
      "mapped_genes": ["LSP1"],
      "locations": ["11:1887776"],
      "ci_lower": 1.04,
      "ci_upper": 1.11,
      "beta": "-",
      "odds_ratio": 1.07,
      "risk_frequency": "0.30",
      "interval": "[1.04-1.11]",
      "publication": "17529967"
    }
  },
  "sources": [
    {
      "source": "gwas",
      "use": "restricted",
      "terms": "GWAS Catalog terms and dataset rights",
      "terms_url": "https://www.ebi.ac.uk/about/terms-of-use/",
      "state": "ok"
    }
  ],
  "next": [
…
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [GwasAssociationCard](#gwasassociationcard). `query` holds [GwasGet](/reference/schemas/types/#gwasget), a shared type. `query` repeats the request as BioMCP read it.

### GwasAssociationCard

Exact association card; no second association representation.

| Field | Type | Meaning |
| --- | --- | --- |
| `association` | [GwasAssociation](/reference/schemas/types/#gwasassociation) | The shared source association record. |

## Next

[`biomcp get gwas`](/reference/commands/get-gwas/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
