---
title: Reply of biomcp variant population (biomcp.variant_population/4)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp variant population:
  a real example and the fields of schema biomcp.variant_population/4."
goal: "Parse the biomcp.variant_population/4 reply: see a real example and what
  each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.variant_population/4
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp variant population`](/reference/commands/variant-population/) replies with `schema` set to `biomcp.variant_population/4`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp variant population CA118786
biomcp variant population CA118786 --detail --limit 50
```

`biomcp schema biomcp.variant_population/4` prints the full JSON Schema.

## Example reply

`biomcp variant population CA118786 --detail --limit 50` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.variant_population/4",
  "status": "partial",
  "query": {
    "input": "CA118786",
    "resolver": "car",
    "detail": true,
    "limit": 50,
    "offset": 0
  },
  "result": {
    "population": {
      "state": "data",
      "source": "gnomad",
      "data": {…}
    },
    "groups": [
      {…},
      … 49 more
    ]
  },
  "page": {"offset": 0, "returned": 50, "total": 276},
  "sources": [
    {
      "source": "car",
      "use": "restricted",
      "terms": "ClinGen Allele Registry service terms; output rights require…",
      "terms_url": "https://clinicalgenome.org/about/terms-of-use/",
      "state": "ok"
    },
    … 1 more
  ],
  "problems": [
    {
      "source": "car",
      "record": "CA118786",
      "field": "representations.expression",
      "code": "value.unknown"
    },
    … 248 more
…
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [VariantPopulationResult](#variantpopulationresult). `query` holds [VariantPopulationQuery](#variantpopulationquery). `query` repeats the request as BioMCP read it.

### VariantPopulationResult

Population summary and one explicitly requested ancestry-detail page.

| Field | Type | Meaning |
| --- | --- | --- |
| `groups` | list of [GnomadCounts](/reference/schemas/types/#gnomadcounts) | Selected native groups, qualified by sequencing subset. Optional. |
| `population` | [Section](/reference/schemas/types/#section) of [VariantPopulation](/reference/schemas/types/#variantpopulation) | Source population state; missing datasets stay absent within its summary. |

### VariantPopulationQuery

Read a bounded ancestry-detail page from the exact source-given GRCh38 allele.

| Field | Type | Meaning |
| --- | --- | --- |
| `build` | [Build](/reference/schemas/types/#build) or null | Explicit source assembly when required by identity input. Optional. |
| `detail` | boolean | Read source ancestry groups, preserving overlapping groups individually. Optional. |
| `input` | [VariantInput](/reference/schemas/types/#variantinput) | Exact published allele identity; ambiguous candidates must first be selected. |
| `limit` | integer (uint32) | Maximum rows per page. Optional. |
| `offset` | integer (uint32) | Native rows to skip. Optional. |
| `resolver` | [VariantResolver](/reference/schemas/types/#variantresolver) | Resolver for reference-qualified input. Optional. |

## Next

[`biomcp variant population`](/reference/commands/variant-population/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
