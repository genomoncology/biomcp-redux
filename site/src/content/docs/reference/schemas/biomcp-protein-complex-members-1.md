---
title: Reply of biomcp protein complex-members (biomcp.protein_complex_members/1)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp protein
  complex-members: a real example and the fields of schema
  biomcp.protein_complex_members/1."
goal: "Parse the biomcp.protein_complex_members/1 reply: see a real example and
  what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.protein_complex_members/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp protein complex-members`](/reference/commands/protein-complex-members/) replies with `schema` set to `biomcp.protein_complex_members/1`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp protein complex-members P15056 --complex CPX-10863
```

`biomcp schema biomcp.protein_complex_members/1` prints the full JSON Schema.

## Example reply

`biomcp protein complex-members P15056 --complex CPX-10863` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

```json
{
  "schema": "biomcp.protein_complex_members/1",
  "status": "complete",
  "query": {
    "accession": "P15056",
    "complex": "CPX-10863",
    "limit": 5,
    "offset": 0
  },
  "result": {
    "items": [
      {
        "id": "Q7Z401",
        "participant_type": "protein",
        "name": "DENND4A",
        "protein": "Q7Z401"
      },
      {
        "id": "Q9UPU9",
        "participant_type": "protein",
        "name": "SAMD4A",
        "protein": "Q9UPU9"
      },
      … 3 more
    ]
  },
  "page": {
    "offset": 0,
    "returned": 5,
    "total": 29,
    "pool": {
      "id": "ebcfcfc1873463d6490855961d381d69",
      "size": 29,
      "created": 1793140311,
      "expires": 1793226711
    }
  },
  "sources": [
    {
      "source": "complex-portal",
      "use": "open",
      "terms": "CC0",
      "terms_url": "https://pmc.ncbi.nlm.nih.gov/articles/PMC11701666/",
      "state": "ok"
    }
  ],
  "next": [
    {
      "kind": "pivot",
      "args": ["get", "protein", "Q7Z401"]
    },
    {
      "kind": "pivot",
      "args": ["get", "protein", "Q9UPU9"]
    },
    … 4 more
  ]
}
```

## Fields

`result` holds [ProteinComplexMembers](#proteincomplexmembers). `query` holds [ProteinComplexMembersQuery](#proteincomplexmembersquery). `query` repeats the request as BioMCP read it.

### ProteinComplexMembers

Native participants of one exact containing complex, including non-protein source facts.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `items` | list of [ProteinComplexMember](#proteincomplexmember) | yes | Whole native rows in source order; totals apply only to this collection. |

### ProteinComplexMembersQuery

Page five native participants of an exact Complex Portal complex containing the anchor. No interaction or clinical meaning is inferred; no bioinformatics tool runs.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `accession` | [UniProtIsoform](/reference/schemas/types/#uniprotisoform) | yes | Exact source protein sequence identity. |
| `complex` | [ComplexPortalId](/reference/schemas/types/#complexportalid) | yes | Exact complex whose native participants contain the anchor. |
| `limit` | integer (uint32) | no | Five rows by default, at most 25. |
| `offset` | integer (uint32) | no | Native or retained page offset. |
| `pool` | [Text](/reference/schemas/types/#text) | no | The ID of a stored list, from an earlier reply, so later pages read the same data. |

### ProteinComplexMember

A native participant; names never establish a protein identity.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `id` | [Text](/reference/schemas/types/#text) | yes | Native participant identifier, including non-protein identifiers. |
| `name` | [Text](/reference/schemas/types/#text) or null | no | Native display name. |
| `organism` | [Text](/reference/schemas/types/#text) or null | no | Literal supplied organism text, including non-taxonomic source sentinels. |
| `participant_type` | [Text](/reference/schemas/types/#text) | yes | Literal native participant type; non-protein members remain source facts. |
| `protein` | [UniProtIsoform](/reference/schemas/types/#uniprotisoform) or null | no | Only an exact source-supplied protein identifier; never inferred from a name. |
| `stoichiometry` | [Text](/reference/schemas/types/#text) or null | no | Native stoichiometry text; no numeric imputation. |
| `taxon` | integer or null | no | Native taxonomy identifier when supplied. |
| `xrefs` | list of [Curie](/reference/schemas/types/#curie) | no | Source-supplied cross-references only. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp protein complex-members`](/reference/commands/protein-complex-members/) gives the command's arguments.
