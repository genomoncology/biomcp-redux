---
title: Reply of biomcp get gwas ancestries section (biomcp.gwas_ancestries/1)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get gwas ancestries:
  a real example and the fields of schema biomcp.gwas_ancestries/1."
goal: "Parse the biomcp.gwas_ancestries/1 reply: see a real example and what
  each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.gwas_ancestries/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get gwas ancestries`](/reference/commands/get-gwas/) replies with `schema` set to `biomcp.gwas_ancestries/1`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp get gwas GCST000035 ancestries
```

`biomcp schema biomcp.gwas_ancestries/1` prints the full JSON Schema.

## Example reply

`biomcp get gwas GCST000035 ancestries` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.gwas_ancestries/1",
  "status": "complete",
  "query": {
    "kind": "study",
    "id": "GCST000035",
    "section": "ancestries",
    "section_limit": 3,
    "section_offset": 0
  },
  "result": {
    "study": "GCST000035",
    "items": [
      {
        "stage": "replication",
        "number_of_individuals": 48565,
        "ancestry_groups": ["European"],
        "country_of_origin": [
          {"country": "NR", "major_area": "NR", "region": "NR"}
        ],
        "country_of_recruitment": [
          {…},
          … 11 more
        ],
        "source_url": "https://www.ebi.ac.uk/gwas/rest/api/v2/studies/GCST0000…"
      },
      … 2 more
    ]
  },
  "page": {"offset": 0, "returned": 3, "total": 3},
  "sources": [
    {
      "source": "gwas",
      "use": "restricted",
      "terms": "GWAS Catalog terms and dataset rights",
      "terms_url": "https://www.ebi.ac.uk/about/terms-of-use/",
      "state": "ok"
    }
  ]
}
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [GwasAncestries](#gwasancestries). `query` holds [GwasGet](/reference/schemas/types/#gwasget), a shared type. `query` repeats the request as BioMCP read it.

### GwasAncestries

Bounded retained ancestry relation for one exact study.

| Field | Type | Meaning |
| --- | --- | --- |
| `continuation` | [GwasGet](/reference/schemas/types/#gwasget) | Retained relation continuation. Optional. |
| `items` | list of [GwasAncestry](#gwasancestry) | Independent source rows. |
| `study` | [GwasStudyId](/reference/schemas/types/#gwasstudyid) | Exact owning study. |

### GwasAncestry

One independent source ancestry row; counts are never summed.

| Field | Type | Meaning |
| --- | --- | --- |
| `ancestry_groups` | list of [Text](/reference/schemas/types/#text) | Original ancestry group labels. |
| `country_of_origin` | list of [GwasCountry](#gwascountry) | Native origin metadata. |
| `country_of_recruitment` | list of [GwasCountry](#gwascountry) | Native recruitment metadata. |
| `number_of_individuals` | integer or null | Source row count; no unique-person denominator is inferred. Optional. |
| `source_url` | [Text](/reference/schemas/types/#text) | Native row self link, checked against its owning study by the adapter. |
| `stage` | [Text](/reference/schemas/types/#text) | Native discovery or replication stage. |

### GwasCountry

Native country and region metadata.

| Field | Type | Meaning |
| --- | --- | --- |
| `country` | [Text](/reference/schemas/types/#text) | Original country label including NR. |
| `major_area` | [Text](/reference/schemas/types/#text) or null | Native source major area when supplied. Optional. |
| `region` | [Text](/reference/schemas/types/#text) or null | Original region label when supplied. Optional. |

## Next

[`biomcp get gwas`](/reference/commands/get-gwas/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
