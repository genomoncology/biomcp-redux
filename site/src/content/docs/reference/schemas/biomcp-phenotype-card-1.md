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

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get phenotype`](/reference/commands/get-phenotype/) replies with `schema` set to `biomcp.phenotype-card/1`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp get phenotype HP:0001250 --source hpo
biomcp get phenotype HP:0000001
```

`biomcp schema biomcp.phenotype-card/1` prints the full JSON Schema.

## Example reply

`biomcp get phenotype HP:0001250 --source hpo` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

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
      "attribution": {
        "source": "hpo",
        "record": "HP:0001250",
        "url": "https://hpo.jax.org/browse/term/HP:0001250"
      }
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
    {
      "kind": "section",
      "args": [
        "get", "phenotype", "HP:0001250", "xrefs", "--snapshot",
        "d815af442b500ad19f8f96b77ade12206736cc84cd3edd3f614f7f33c5e9caeb"
      ]
    }
  ]
}
```

## Fields

`result` holds [PhenotypeTermCard](#phenotypetermcard). `query` holds [PhenotypeGet](/reference/schemas/types/#phenotypeget), a shared type. `query` repeats the request as BioMCP read it.

### PhenotypeTermCard

Independent source identity card and its available sections.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `components` | list of [ComponentLabel](/reference/schemas/types/#componentlabel) | yes | Applicable publisher component terms. |
| `sections` | list of [PhenotypeSection](/reference/schemas/types/#phenotypesection) | yes | Sections available on this source record. |
| `term` | [PhenotypeCard](#phenotypecard) | yes | Complete source-specific native card. |

### PhenotypeAssociationCount

Native collection counts, without summing scopes or counting patients.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `category` | [Curie](/reference/schemas/types/#curie) | yes | Native category. |
| `count` | integer (uint64) | yes | Native closure-expanded count. |
| `count_direct` | integer or null | no | Native direct count. |
| `count_with_orthologs` | integer or null | no | Native count including orthologs, only when supplied. |
| `key` | [Curie](/reference/schemas/types/#curie) | yes | Native category key. |
| `label` | string | yes | Native scope label. |

### PhenotypeCard

Native source card, with independent HPO and Monarch metadata.

One of these forms:

- `source` is `"hpo"`: The direct HPO term answer. Fields: `attribution` ([Attribution](/reference/schemas/types/#attribution)), `comment` (string or null), `definition` (string or null), `hpo_descendant_count` (integer or null), `label` (string), `publication_references` (list of [Curie](/reference/schemas/types/#curie) or null), `requested_id` ([HpId](/reference/schemas/types/#hpid)), `returned_id` ([HpId](/reference/schemas/types/#hpid)).
- `source` is `"monarch"`: The independent Monarch term answer. Fields: `association_counts` (list of [PhenotypeAssociationCount](#phenotypeassociationcount) or null), `attribution` ([Attribution](/reference/schemas/types/#attribution)), `deprecated` (boolean or null), `description` (string or null), `label` (string), `monarch_descendant_count` (integer or null), `requested_id` ([HpId](/reference/schemas/types/#hpid)), `returned_id` ([HpId](/reference/schemas/types/#hpid)).

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp get phenotype`](/reference/commands/get-phenotype/) gives the command's arguments.
