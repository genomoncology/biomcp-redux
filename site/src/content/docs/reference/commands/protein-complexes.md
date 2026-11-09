---
title: biomcp protein complexes
description: "The BioMCP 1.0.0 command biomcp protein complexes: Lists the
  Complex Portal complexes that hold one protein, five at a time."
goal: Look up the arguments, sections and examples of biomcp protein complexes.
reader: A BioMCP user or agent checking one command
sidebar:
  label: protein complexes
kind: reference
verified: 2026-10-28
---

`biomcp protein complexes` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It lists the Complex Portal complexes that hold one protein, five at a time. This page lists its arguments, examples and reply.

The program's help describes it:

> Page five source isoforms, exact-participant complexes or PDB links. Source identities stay separate. No structure, interaction or clinical meaning is inferred; no bioinformatics tool runs.

## Usage

```text
biomcp protein complexes [OPTIONS] <accession>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--limit` | integer | no | min 1, max 25 |  | Five rows by default, at most 25. |
| `--offset` | integer | no | min 0 |  | Native or retained page offset. |
| `--pool` | string | no |  |  | The ID of a stored list, from an earlier reply, so later pages read the same data. |
| `accession` (position 1) | string | yes |  |  | Exact source protein sequence identity. |

## Examples

The program's help gives these examples.

```sh
biomcp protein complexes P00533
```

```sh
biomcp protein complexes P00533 --limit 1
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.protein_complexes/2`](/reference/schemas/biomcp-protein-complexes-2/), whose page shows a real reply and its fields.

## Next

The [protein guide](/user-guide/protein/) shows every protein command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
