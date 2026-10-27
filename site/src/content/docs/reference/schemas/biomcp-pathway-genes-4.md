---
title: Reply of biomcp pathway genes (biomcp.pathway_genes/4)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp pathway genes: a
  real example and the fields of schema biomcp.pathway_genes/4."
goal: "Parse the biomcp.pathway_genes/4 reply: see a real example and what each
  field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.pathway_genes/4
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp pathway genes`](/reference/commands/pathway-genes/) replies with `schema` set to `biomcp.pathway_genes/4`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp pathway genes R-HSA-5674135
biomcp pathway genes R-HSA-5673001 details
```

`biomcp schema biomcp.pathway_genes/4` prints the full JSON Schema.

## Example reply

`biomcp pathway genes R-HSA-5674135` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.pathway_genes/4",
  "status": "partial",
  "query": {"id": "R-HSA-5674135", "limit": 5, "offset": 0},
  "result": {
    "header": {
      "id": {"source": "reactome", "id": "R-HSA-5674135"},
      "name": "MAP2K and MAPK activation",
      "species": "Homo sapiens",
      "version": "R-HSA-5674135.4"
    },
    "items": [
      {…},
      … 4 more
    ],
    "page": {
      "offset": 0,
      "returned": 5,
      "total": 66,
      "total_kind": "distinct_references",
      "next_offset": 5,
      "pool": {…}
    },
    "reference_occurrences": 440,
    "physical_entities": 29
  },
  "sources": [
    {
      "source": "reactome",
      "use": "open",
      "terms": "CC0 1.0 database data",
      "terms_url": "https://reactome.org/license",
      "state": "ok"
    },
    … 1 more
  ],
  "problems": [
    {
      "source": "reactome",
      "record": "chebi:37565",
…
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [PathwayGenes](/reference/schemas/types/#pathwaygenes), a shared type. `query` holds [PathwayGenesQuery](#pathwaygenesquery). `query` repeats the request as BioMCP read it.

### PathwayGenesQuery

Read Reactome exact references or WikiPathways GeneProduct data nodes. The details layer discloses complete Reactome physical contexts. Membership names a source and release; it implies no therapy, response or actionability. No enrichment is calculated.

| Field | Type | Meaning |
| --- | --- | --- |
| `details` | [Details](#details) | Adds each reference's Reactome `PhysicalEntity` contexts, with their source text whole. Optional. |
| `id` | string | Checked source-owned human pathway identity. |
| `limit` | integer (uint32) | Five complete relation rows by default, at most fifty. Optional. |
| `offset` | integer (uint32) | Offset within the retained selected relation. Optional. |
| `pool` | [Text](/reference/schemas/types/#text) | Immutable relation snapshot. Optional. |

### Details

Explicit full physical-context disclosure; omission keeps membership compact.

Values: `details`.

## Next

[`biomcp pathway genes`](/reference/commands/pathway-genes/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
