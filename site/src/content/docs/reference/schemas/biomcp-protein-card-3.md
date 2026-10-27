---
title: Reply of biomcp get protein (biomcp.protein_card/3)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get protein: a real
  example and the fields of schema biomcp.protein_card/3."
goal: "Parse the biomcp.protein_card/3 reply: see a real example and what each
  field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.protein_card/3
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get protein`](/reference/commands/get-protein/) replies with `schema` set to `biomcp.protein_card/3`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp get protein P15056
biomcp get protein P00533 Q9UIF7 P38398-2
```

`biomcp schema biomcp.protein_card/3` prints the full JSON Schema.

## Example reply

`biomcp get protein P15056` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

```json
{
  "schema": "biomcp.protein_card/3",
  "status": "complete",
  "query": {"input": ["P15056"], "section_limit": 5},
  "result": {
    "items": [
      {
        "input": "P15056",
        "status": "complete",
        "header": {
          "protein": {
            "accession": "P15056",
            "name": "Serine/threonine-protein kinase B-raf",
            "length": 766
          },
          "entry_id": "BRAF_HUMAN",
          "gene_names": ["BRAF", "BRAF1", "RAFB1"],
          "taxon": 9606,
          "reviewed": true,
          "url": "https://www.uniprot.org/uniprotkb/P15056/entry"
        },
        "audit": {
          "entry_version": 269,
          "sequence_version": 4,
          "first_public": "1990-11-01",
          "last_annotation_update": "2026-09-02",
          "last_sequence_update": "2004-07-19"
        },
        "more": [
          "domains", "complexes", "features", "isoforms", "structures",
          "function", "sequence"
        ]
      }
    ]
  },
  "sources": [
    {
      "source": "uniprot",
      "use": "attribution",
      "terms": "CC BY 4.0",
      "terms_url": "https://www.uniprot.org/help/license",
      "state": "ok"
    }
  ],
  "next": [
    {
      "kind": "section",
      "args": ["get", "protein", "P15056", "domains"]
    },
    {
      "kind": "section",
      "args": ["get", "protein", "P15056", "complexes"]
    },
    … 3 more
  ]
}
```

## Fields

`result` holds [ProteinCard](#proteincard). `query` holds [ProteinGet](/reference/schemas/types/#proteinget), a shared type. `query` repeats the request as BioMCP read it.

### ProteinCard

Answers to requested inputs; ambiguous gene inputs retain all candidates.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `items` | list of [ProteinCardItem](#proteincarditem) | no | One result per input in caller order. |

### ProteinCardItem

One requested identity and its source outcome, without first-hit selection.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `audit` | [ProteinAudit](/reference/schemas/types/#proteinaudit) or null | no | Native audit for the selected entry only. |
| `candidates` | list of [ProteinHeader](/reference/schemas/types/#proteinheader) | no | Competing exact source identities when a gene does not resolve uniquely. |
| `header` | [ProteinHeader](/reference/schemas/types/#proteinheader) or null | no | Exact resolved native header; absent for ambiguity or failure. |
| `input` | [ProteinInput](/reference/schemas/types/#proteininput) | yes | Original caller identity. |
| `more` | list of [ProteinSection](/reference/schemas/types/#proteinsection) | no | Supported sections not requested. |
| `sections` | [ProteinSections](#proteinsections) or null | no | Requested bounded sections only. |
| `status` | [Status](/reference/schemas/types/#status) | yes | Outcome of this input. |

### ProteinSections

Independent source section states; all discloses bounded first pages only.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `complexes` | [Section](/reference/schemas/types/#section) of [ProteinComplexes](/reference/schemas/types/#proteincomplexes) or null | no | Requested exact participant-admitted header page. |
| `domains` | [Section](/reference/schemas/types/#section) of [ProteinDomains](/reference/schemas/types/#proteindomains) or null | no | Requested InterPro match page. |
| `features` | [Section](/reference/schemas/types/#section) of [ProteinFeatures](/reference/schemas/types/#proteinfeatures) or null | no | Requested UniProt feature page. |
| `function` | [Section](/reference/schemas/types/#section) of [ProteinFunctionResult](/reference/schemas/types/#proteinfunctionresult) or null | no | Requested whole native function text and evidence. |
| `isoforms` | [Section](/reference/schemas/types/#section) of [ProteinIsoforms](/reference/schemas/types/#proteinisoforms) or null | no | Requested native alternative-product page. |
| `sequence` | [Section](/reference/schemas/types/#section) of [ProteinSequenceResult](/reference/schemas/types/#proteinsequenceresult) or null | no | Requested direct sequence; absent/unavailable states remain explicit. |
| `structures` | [Section](/reference/schemas/types/#section) of [ProteinStructures](/reference/schemas/types/#proteinstructures) or null | no | Requested structure-link page. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp get protein`](/reference/commands/get-protein/) gives the command's arguments.
