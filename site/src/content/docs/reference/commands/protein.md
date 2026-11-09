---
title: biomcp protein
description: "The BioMCP 1.0.0 command biomcp protein: Read protein sequences,
  features and source relationships."
goal: Look up the arguments, sections and examples of biomcp protein.
reader: A BioMCP user or agent checking one command
sidebar:
  label: protein
kind: reference
verified: 2026-11-09
---

`biomcp protein` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It read protein sequences, features and source relationships. This page lists its examples and reply.

The program's help describes it:

> Read protein sequences, features and source relationships.

## Usage

```text
biomcp protein
```

## Examples

The program's help gives this example.

```sh
biomcp protein features P00533
```

## Reply

The reply is one JSON object. Its schema is one of [`biomcp.protein-complex-members/4`](/reference/schemas/biomcp-protein-complex-members-4/), [`biomcp.protein-complexes/5`](/reference/schemas/biomcp-protein-complexes-5/), [`biomcp.protein-domains/7`](/reference/schemas/biomcp-protein-domains-7/), [`biomcp.protein-features/4`](/reference/schemas/biomcp-protein-features-4/), [`biomcp.protein-isoforms/4`](/reference/schemas/biomcp-protein-isoforms-4/), [`biomcp.protein-sequence/4`](/reference/schemas/biomcp-protein-sequence-4/), [`biomcp.protein-structures/5`](/reference/schemas/biomcp-protein-structures-5/), whose page lists its fields.

## Next

The [protein guide](/user-guide/protein/) shows every protein command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
