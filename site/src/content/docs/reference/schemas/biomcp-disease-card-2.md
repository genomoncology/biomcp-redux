---
title: Reply of biomcp get disease (biomcp.disease_card/2)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get disease: a real
  example and the fields of schema biomcp.disease_card/2."
goal: "Parse the biomcp.disease_card/2 reply: see a real example and what each
  field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.disease_card/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get disease`](/reference/commands/get-disease/) replies with `schema` set to `biomcp.disease_card/2`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp get disease MONDO:0005105
biomcp get disease MONDO:0005105 --source mydisease
```

`biomcp schema biomcp.disease_card/2` prints the full JSON Schema.

## Example reply

`biomcp get disease MONDO:0005105` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.disease_card/2",
  "status": "complete",
  "query": {
    "id": "MONDO:0005105",
    "source": "mydisease",
    "section_limit": 5,
    "section_offset": 0
  },
  "result": {
    "requested_id": "MONDO:0005105",
    "returned_id": "MONDO:0005105",
    "concept": {"id": "MONDO:0005105", "label": "melanoma"},
    "definition": "A malignant, usually aggressive tumor composed of atypical…",
    "replaced_by": null,
    "attribution": {
      "source": "mydisease",
      "record": "MONDO:0005105",
      "version": "1",
      "url": "https://mydisease.info/v1/disease/MONDO:0005105"
    },
    "sections": ["synonyms", "xrefs", "hierarchy"],
    "components": [
      {…},
      … 1 more
    ]
  },
  "sources": [
    {
      "source": "mydisease",
      "use": "attribution",
      "terms": "MyDisease source-specific terms",
      "terms_url": "https://docs.mydisease.info/en/latest/doc/data.html",
      "state": "ok"
    }
  ],
  "next": [
    {
      "kind": "section",
      "args": [
…
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [DiseaseCard](#diseasecard). `query` holds [DiseaseGet](/reference/schemas/types/#diseaseget), a shared type. `query` repeats the request as BioMCP read it.

### DiseaseCard

Exact identity and available pages without hidden enrichment.

| Field | Type | Meaning |
| --- | --- | --- |
| `alias` | [DiseaseRelation](/reference/schemas/types/#diseaserelation) or null | Native relation proving a source-accepted alternate identifier. Optional. |
| `attribution` | [Attribution](/reference/schemas/types/#attribution) | Publisher record and version. |
| `components` | list of [ComponentLabel](/reference/schemas/types/#componentlabel) | Applicable component terms. |
| `concept` | [Concept](/reference/schemas/types/#concept) | Native identity and complete primary label. |
| `consider` | list of [Curie](/reference/schemas/types/#curie) or null | Publisher alternatives, never an automatic replacement. Optional. |
| `definition` | [Text](/reference/schemas/types/#text) or null | Complete source definition when supplied. Optional. |
| `obsolete` | boolean or null | Publisher obsolete flag. Optional. |
| `replaced_by` | list of [Curie](/reference/schemas/types/#curie) or null | Publisher explicit replacement identifiers; null preserves no native replacement. Optional. |
| `requested_id` | [Curie](/reference/schemas/types/#curie) | Exact requested identifier. |
| `returned_id` | [Curie](/reference/schemas/types/#curie) | Actual returned identifier. |
| `sections` | list of [DiseaseSection](/reference/schemas/types/#diseasesection) | Available named sections. |

## Next

[`biomcp get disease`](/reference/commands/get-disease/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
