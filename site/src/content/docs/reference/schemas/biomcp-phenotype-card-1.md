---
title: Reply of biomcp get phenotype (biomcp.phenotype-card/1)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get phenotype: a
  real example and the fields of schema biomcp.phenotype-card/1."
goal: "Parse the biomcp.phenotype-card/1 reply: see a real example and what each
  field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.phenotype-card/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get phenotype`](/reference/commands/get-phenotype/) replies with `schema` set to `biomcp.phenotype-card/1`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp get phenotype HP:0001250 --source hpo
biomcp get phenotype HP:0000001
```

`biomcp schema biomcp.phenotype-card/1` prints the full JSON Schema.

## Example reply

`biomcp get phenotype HP:0001250 --source hpo` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.phenotype-card/1",
  "status": "complete",
  "query": {
    "id": "HP:0001250",
    "source": "hpo",
    "section_offset": 0
  },
  "result": {
    "term": {
      "source": "hpo",
      "requested_id": "HP:0001250",
      "returned_id": "HP:0001250",
      "label": "Seizure",
      "definition": "A seizure is an intermittent abnormality of nervous syst…",
      "comment": "A type of electrographic seizure has been proposed in neona…",
      "publication_references": ["PMID:15816939"],
      "hpo_descendant_count": 346,
      "attribution": {…}
    },
    "sections": ["synonyms", "xrefs"],
    "components": []
  },
  "sources": [
    {
      "source": "hpo",
      "use": "attribution",
      "terms": "HPO project custom license: attribution, version and integrity",
      "terms_url": "https://human-phenotype-ontology.github.io/license.html",
      "state": "ok"
    }
  ],
  "next": [
    {
      "kind": "section",
      "args": [
        "get", "phenotype", "HP:0001250", "synonyms", "--snapshot",
        "d815af442b500ad19f8f96b77ade12206736cc84cd3edd3f614f7f33c5e9caeb"
      ]
    },
…
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [PhenotypeTermCard](#phenotypetermcard). `query` holds [PhenotypeGet](/reference/schemas/types/#phenotypeget), a shared type. `query` repeats the request as BioMCP read it.

### PhenotypeTermCard

Independent source identity card and its available sections.

| Field | Type | Meaning |
| --- | --- | --- |
| `components` | list of [ComponentLabel](/reference/schemas/types/#componentlabel) | Applicable publisher component terms. |
| `sections` | list of [PhenotypeSection](/reference/schemas/types/#phenotypesection) | Sections available on this source record. |
| `term` | [PhenotypeCard](#phenotypecard) | Complete source-specific native card. |

### Other types in this reply

2 more types appear only in this reply: [PhenotypeAssociationCount](#phenotypeassociationcount), [PhenotypeCard](#phenotypecard). Open the list below to read their fields.

<details>
<summary>Fields of the 2 other types</summary>

### PhenotypeAssociationCount

Native collection counts, without summing scopes or counting patients.

| Field | Type | Meaning |
| --- | --- | --- |
| `category` | [Curie](/reference/schemas/types/#curie) | Native category. |
| `count` | integer (uint64) | Native closure-expanded count. |
| `count_direct` | integer or null | Native direct count. Optional. |
| `count_with_orthologs` | integer or null | Native count including orthologs, only when supplied. Optional. |
| `key` | [Curie](/reference/schemas/types/#curie) | Native category key. |
| `label` | string | Native scope label. |

### PhenotypeCard

Native source card, with independent HPO and Monarch metadata.

One of these forms:

- `source` is `"hpo"`: The direct HPO term answer. Fields: `attribution` ([Attribution](/reference/schemas/types/#attribution)), `comment` (string or null), `definition` (string or null), `hpo_descendant_count` (integer or null), `label` (string), `publication_references` (list of [Curie](/reference/schemas/types/#curie) or null), `requested_id` ([HpId](/reference/schemas/types/#hpid)), `returned_id` ([HpId](/reference/schemas/types/#hpid)).
- `source` is `"monarch"`: The independent Monarch term answer. Fields: `association_counts` (list of [PhenotypeAssociationCount](#phenotypeassociationcount) or null), `attribution` ([Attribution](/reference/schemas/types/#attribution)), `deprecated` (boolean or null), `description` (string or null), `label` (string), `monarch_descendant_count` (integer or null), `requested_id` ([HpId](/reference/schemas/types/#hpid)), `returned_id` ([HpId](/reference/schemas/types/#hpid)).

</details>

## Next

[`biomcp get phenotype`](/reference/commands/get-phenotype/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
