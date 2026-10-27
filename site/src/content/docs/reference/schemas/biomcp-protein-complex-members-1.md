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

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp protein complex-members`](/reference/commands/protein-complex-members/) replies with `schema` set to `biomcp.protein_complex_members/1`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp protein complex-members P15056 --complex CPX-10863
```

`biomcp schema biomcp.protein_complex_members/1` prints the full JSON Schema.

## Example reply

`biomcp protein complex-members P15056 --complex CPX-10863` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

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
      {…},
      … 4 more
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
    {"kind": "pivot", "args": ["get", "protein", "Q7Z401"]},
    … 5 more
  ]
}
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [ProteinComplexMembers](#proteincomplexmembers). `query` holds [ProteinComplexMembersQuery](#proteincomplexmembersquery). `query` repeats the request as BioMCP read it.

### ProteinComplexMembers

Native participants of one exact containing complex, including non-protein source facts.

| Field | Type | Meaning |
| --- | --- | --- |
| `items` | list of [ProteinComplexMember](#proteincomplexmember) | Whole native rows in source order; totals apply only to this collection. |

### ProteinComplexMembersQuery

Page five native participants of an exact Complex Portal complex containing the anchor. No interaction or clinical meaning is inferred; no bioinformatics tool runs.

| Field | Type | Meaning |
| --- | --- | --- |
| `accession` | [UniProtIsoform](/reference/schemas/types/#uniprotisoform) | Exact source protein sequence identity. |
| `complex` | [ComplexPortalId](/reference/schemas/types/#complexportalid) | Exact complex whose native participants contain the anchor. |
| `limit` | integer (uint32) | Five rows by default, at most 25. Optional. |
| `offset` | integer (uint32) | Native or retained page offset. Optional. |
| `pool` | [Text](/reference/schemas/types/#text) | The ID of a stored list, from an earlier reply, so later pages read the same data. Optional. |

### Other types in this reply

One more type appear only in this reply: [ProteinComplexMember](#proteincomplexmember). Open the list below to read their fields.

<details>
<summary>Fields of the 1 other type</summary>

### ProteinComplexMember

A native participant; names never establish a protein identity.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [Text](/reference/schemas/types/#text) | Native participant identifier, including non-protein identifiers. |
| `name` | [Text](/reference/schemas/types/#text) or null | Native display name. Optional. |
| `organism` | [Text](/reference/schemas/types/#text) or null | Literal supplied organism text, including non-taxonomic source sentinels. Optional. |
| `participant_type` | [Text](/reference/schemas/types/#text) | Literal native participant type; non-protein members remain source facts. |
| `protein` | [UniProtIsoform](/reference/schemas/types/#uniprotisoform) or null | Only an exact source-supplied protein identifier; never inferred from a name. Optional. |
| `stoichiometry` | [Text](/reference/schemas/types/#text) or null | Native stoichiometry text; no numeric imputation. Optional. |
| `taxon` | integer or null | Native taxonomy identifier when supplied. Optional. |
| `xrefs` | list of [Curie](/reference/schemas/types/#curie) | Source-supplied cross-references only. Optional. |

</details>

## Next

[`biomcp protein complex-members`](/reference/commands/protein-complex-members/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
