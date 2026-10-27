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

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get protein`](/reference/commands/get-protein/) replies with `schema` set to `biomcp.protein_card/3`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp get protein P15056
biomcp get protein P00533 Q9UIF7 P38398-2
```

`biomcp schema biomcp.protein_card/3` prints the full JSON Schema.

## Example reply

`biomcp get protein P15056` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

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
        "header": {…},
        "audit": {…},
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
    … 4 more
  ]
}
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [ProteinCard](#proteincard). `query` holds [ProteinGet](/reference/schemas/types/#proteinget), a shared type. `query` repeats the request as BioMCP read it.

### ProteinCard

Answers to requested inputs; ambiguous gene inputs retain all candidates.

| Field | Type | Meaning |
| --- | --- | --- |
| `items` | list of [ProteinCardItem](#proteincarditem) | One result per input in caller order. Optional. |

### Other types in this reply

2 more types appear only in this reply: [ProteinCardItem](#proteincarditem), [ProteinSections](#proteinsections). Open the list below to read their fields.

<details>
<summary>Fields of the 2 other types</summary>

### ProteinCardItem

One requested identity and its source outcome, without first-hit selection.

| Field | Type | Meaning |
| --- | --- | --- |
| `audit` | [ProteinAudit](/reference/schemas/types/#proteinaudit) or null | Native audit for the selected entry only. Optional. |
| `candidates` | list of [ProteinHeader](/reference/schemas/types/#proteinheader) | Competing exact source identities when a gene does not resolve uniquely. Optional. |
| `header` | [ProteinHeader](/reference/schemas/types/#proteinheader) or null | Exact resolved native header; absent for ambiguity or failure. Optional. |
| `input` | [ProteinInput](/reference/schemas/types/#proteininput) | Original caller identity. |
| `more` | list of [ProteinSection](/reference/schemas/types/#proteinsection) | Supported sections not requested. Optional. |
| `sections` | [ProteinSections](#proteinsections) or null | Requested bounded sections only. Optional. |
| `status` | [Status](/reference/schemas/types/#status) | Outcome of this input. |

### ProteinSections

Independent source section states; all discloses bounded first pages only.

| Field | Type | Meaning |
| --- | --- | --- |
| `complexes` | [Section](/reference/schemas/types/#section) of [ProteinComplexes](/reference/schemas/types/#proteincomplexes) or null | Requested exact participant-admitted header page. Optional. |
| `domains` | [Section](/reference/schemas/types/#section) of [ProteinDomains](/reference/schemas/types/#proteindomains) or null | Requested InterPro match page. Optional. |
| `features` | [Section](/reference/schemas/types/#section) of [ProteinFeatures](/reference/schemas/types/#proteinfeatures) or null | Requested UniProt feature page. Optional. |
| `function` | [Section](/reference/schemas/types/#section) of [ProteinFunctionResult](/reference/schemas/types/#proteinfunctionresult) or null | Requested whole native function text and evidence. Optional. |
| `isoforms` | [Section](/reference/schemas/types/#section) of [ProteinIsoforms](/reference/schemas/types/#proteinisoforms) or null | Requested native alternative-product page. Optional. |
| `sequence` | [Section](/reference/schemas/types/#section) of [ProteinSequenceResult](/reference/schemas/types/#proteinsequenceresult) or null | Requested direct sequence; absent/unavailable states remain explicit. Optional. |
| `structures` | [Section](/reference/schemas/types/#section) of [ProteinStructures](/reference/schemas/types/#proteinstructures) or null | Requested structure-link page. Optional. |

</details>

## Next

[`biomcp get protein`](/reference/commands/get-protein/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
