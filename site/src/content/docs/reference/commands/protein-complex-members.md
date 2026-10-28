---
title: biomcp protein complex-members
description: "The BioMCP 1.0.0 command biomcp protein complex-members: Lists the
  members of one Complex Portal complex that holds the protein, five at a time."
goal: Look up the arguments, sections and examples of biomcp protein complex-members.
reader: A BioMCP user or agent checking one command
sidebar:
  label: protein complex-members
kind: reference
verified: 2026-10-28
---

`biomcp protein complex-members` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It lists the members of one Complex Portal complex that holds the protein, five at a time. This page lists its arguments, examples and reply.

The program's help describes it:

> Page five native participants of an exact Complex Portal complex containing the anchor. No interaction or clinical meaning is inferred; no bioinformatics tool runs.

## Usage

```text
biomcp protein complex-members [OPTIONS] <accession>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--complex` | string | yes |  |  | Exact complex whose native participants contain the anchor. |
| `--limit` | integer | no | min 1, max 25 |  | Five rows by default, at most 25. |
| `--offset` | integer | no | min 0 |  | Native or retained page offset. |
| `--pool` | string | no |  |  | The ID of a stored list, from an earlier reply, so later pages read the same data. |
| `accession` (position 1) | string | yes |  |  | Exact source protein sequence identity. |

## Examples

The program's help gives these examples.

```sh
biomcp protein complex-members P00533 --complex CPX-715
```

```sh
biomcp protein complex-members P00533 --complex CPX-715 --limit 1
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.protein_complex_members/1`](/reference/schemas/biomcp-protein-complex-members-1/), whose page shows a real reply and its fields.

## Next

The [protein guide](/user-guide/protein/) shows every protein command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
