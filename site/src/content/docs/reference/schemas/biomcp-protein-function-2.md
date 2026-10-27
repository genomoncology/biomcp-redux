---
title: Protein function reply (biomcp.protein_function/2)
description: "The JSON reply BioMCP 1.0.0 prints for no command (protein
  function): a real example and the fields of schema biomcp.protein_function/2."
goal: "Parse the biomcp.protein_function/2 reply: see a real example and what
  each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.protein_function/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. The reply schema `biomcp.protein_function/2` is in the program, but no command returns it as a whole reply. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

`get protein ID function` returns the function text inside `biomcp.protein_card/3`, so no command returns this schema as a whole reply.

## Fields

`result` holds [ProteinFunctionResult](/reference/schemas/types/#proteinfunctionresult), a shared type. `query` holds [ProteinGet](/reference/schemas/types/#proteinget), a shared type. `query` repeats the request as BioMCP read it.

Every type this reply uses is shared with other replies, so its fields are on the shared types page.

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply.
