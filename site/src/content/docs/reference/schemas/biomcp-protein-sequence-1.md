---
title: Reply of biomcp protein sequence (biomcp.protein_sequence/1)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp protein sequence: a
  real example and the fields of schema biomcp.protein_sequence/1."
goal: "Parse the biomcp.protein_sequence/1 reply: see a real example and what
  each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.protein_sequence/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp protein sequence`](/reference/commands/protein-sequence/) replies with `schema` set to `biomcp.protein_sequence/1`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp protein sequence P00533
biomcp protein sequence P00533 --fresh
```

`biomcp schema biomcp.protein_sequence/1` prints the full JSON Schema.

## Example reply

`biomcp protein sequence P00533` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.protein_sequence/1",
  "status": "complete",
  "query": {"isoform": "P00533"},
  "result": {
    "sequence": {
      "accession": "P00533",
      "sequence": "MRPSGTAGAALLALLAALCPASRALEEKKVCQGTSNKLTQLGTFEDHFLSLQRMFNNC…",
      "length": 1210,
      "crc64": "D8A2A50B4EFB6ED2",
      "md5": "99D03B567DBCDD7FB0DD21FB40D5D283",
      "audit": {
        "entry_version": 301,
        "sequence_version": 2,
        "first_public": "1986-07-21",
        "last_annotation_update": "2026-09-02",
        "last_sequence_update": "1997-11-01"
      }
    }
  },
  "sources": [
    {
      "source": "uniprot",
      "use": "attribution",
      "terms": "CC BY 4.0",
      "terms_url": "https://www.uniprot.org/help/license",
      "state": "ok"
    }
  ]
}
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [ProteinSequenceResult](/reference/schemas/types/#proteinsequenceresult), a shared type. `query` holds [ProteinSequenceQuery](#proteinsequencequery). `query` repeats the request as BioMCP read it.

### ProteinSequenceQuery

Read one complete native sequence for an exact source accession. No variant effect, structure or clinical meaning is inferred; no bioinformatics tool runs.

| Field | Type | Meaning |
| --- | --- | --- |
| `isoform` | [UniProtIsoform](/reference/schemas/types/#uniprotisoform) | Exact isoform or base accession; no canonical suffix is synthesized. |

## Next

[`biomcp protein sequence`](/reference/commands/protein-sequence/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
