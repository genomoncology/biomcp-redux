---
title: Reply of biomcp protein sequence (biomcp.protein-sequence/4)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp protein sequence:
  the fields of schema biomcp.protein-sequence/4."
goal: "Parse the biomcp.protein-sequence/4 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.protein-sequence/4
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp protein sequence`](/reference/commands/protein-sequence/) and [`biomcp protein`](/reference/commands/protein/) reply with `schema` set to `biomcp.protein-sequence/4`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.protein-sequence/4` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_ProteinSequenceResult](/reference/schemas/types/#reply-proteinsequenceresult), a shared type. `query` holds [Reply_ProteinSequenceQuery](#reply-proteinsequencequery). `query` repeats the request as BioMCP read it.

### Reply_ProteinSequenceQuery

Read one complete native sequence for an exact source accession. No variant effect, structure or clinical meaning is inferred; no bioinformatics tool runs.

| Field | Type | Meaning |
| --- | --- | --- |
| `isoform` | [Reply_UniProtIsoform](/reference/schemas/types/#reply-uniprotisoform) | Exact isoform or base accession; no canonical suffix is synthesized. |

## Next

[`biomcp protein sequence`](/reference/commands/protein-sequence/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
