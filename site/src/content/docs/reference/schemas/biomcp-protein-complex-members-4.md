---
title: Reply of biomcp protein complex-members (biomcp.protein-complex-members/4)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp protein
  complex-members: the fields of schema biomcp.protein-complex-members/4."
goal: "Parse the biomcp.protein-complex-members/4 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.protein-complex-members/4
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp protein complex-members`](/reference/commands/protein-complex-members/) and [`biomcp protein`](/reference/commands/protein/) reply with `schema` set to `biomcp.protein-complex-members/4`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.protein-complex-members/4` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_ProteinComplexMembers](#reply_proteincomplexmembers). `query` holds [Reply_ProteinComplexMembersQuery](#reply_proteincomplexmembersquery). `query` repeats the request as BioMCP read it.

### Reply_ProteinComplexMembers

Native participants of one exact containing complex, including non-protein source facts.

| Field | Type | Meaning |
| --- | --- | --- |
| `items` | list of [Reply_ProteinComplexMember](#reply_proteincomplexmember) | Whole native rows in source order; totals apply only to this collection. Optional. |

### Reply_ProteinComplexMembersQuery

Page five native participants of an exact Complex Portal complex containing the anchor. No interaction or clinical meaning is inferred; no bioinformatics tool runs.

| Field | Type | Meaning |
| --- | --- | --- |
| `accession` | [Reply_UniProtIsoform](/reference/schemas/types/#reply_uniprotisoform) | Exact source protein sequence identity. |
| `complex` | [Reply_ComplexPortalId](/reference/schemas/types/#reply_complexportalid) | Exact complex whose native participants contain the anchor. |
| `limit` | integer (uint32) | Five rows by default, at most 25. Optional. |
| `offset` | integer (uint32) | Native or retained page offset. Optional. |
| `pool` | [Reply_Text](/reference/schemas/types/#reply_text) | The ID of a stored list, from an earlier reply, so later pages read the same data. Optional. |

### Other types in this reply

One more type appear only in this reply: [Reply_ProteinComplexMember](#reply_proteincomplexmember). Open the list below to read their fields.

<details>
<summary>Fields of the 1 other type</summary>

### Reply_ProteinComplexMember

A native participant; names never establish a protein identity.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [Reply_Text](/reference/schemas/types/#reply_text) | Native participant identifier, including non-protein identifiers. |
| `name` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Native display name. Optional. |
| `organism` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Literal supplied organism text, including non-taxonomic source sentinels. Optional. |
| `participant_type` | [Reply_Text](/reference/schemas/types/#reply_text) | Literal native participant type; non-protein members remain source facts. |
| `protein` | [Reply_UniProtIsoform](/reference/schemas/types/#reply_uniprotisoform) or null | Only an exact source-supplied protein identifier; never inferred from a name. Optional. |
| `stoichiometry` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Native stoichiometry text; no numeric imputation. Optional. |
| `taxon` | integer or null | Native taxonomy identifier when supplied. Optional. |
| `xrefs` | list of [Reply_Curie](/reference/schemas/types/#reply_curie) | Source-supplied cross-references only. Optional. |

</details>

## Next

[`biomcp protein complex-members`](/reference/commands/protein-complex-members/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
