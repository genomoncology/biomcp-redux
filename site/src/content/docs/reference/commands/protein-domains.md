---
title: biomcp protein domains
description: "The BioMCP 1.0.0 command biomcp protein domains: Lists the
  InterPro domains of one protein, five at a time."
goal: Look up the arguments, sections and examples of biomcp protein domains.
reader: A BioMCP user or agent checking one command
sidebar:
  label: protein domains
kind: reference
verified: 2026-10-28
---

`biomcp protein domains` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It lists the InterPro domains of one protein, five at a time. This page lists its arguments, examples and reply.

The program's help describes it:

> Page five native InterPro matches with original types and fragments. No structure or clinical meaning is inferred; no bioinformatics tool runs.

## Usage

```text
biomcp protein domains [OPTIONS] <accession>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--limit` | integer | no | min 1, max 25 |  | Five rows by default, at most 25. |
| `--offset` | integer | no | min 0 |  | Native or retained page offset. |
| `--pool` | string | no |  |  | The ID of a stored list, from an earlier reply, so later pages read the same data. |
| `--type` | string | no |  |  | Exact native InterPro type, not a normalized domain label. |
| `accession` (position 1) | string | yes |  |  | Exact source protein sequence identity. |

## Examples

The program's help gives these examples.

```sh
biomcp protein domains P00533
```

```sh
biomcp protein domains P00533 --limit 1
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.protein_domains/2`](/reference/schemas/biomcp-protein-domains-2/), whose page shows a real reply and its fields.

## Next

The [protein guide](/user-guide/protein/) shows every protein command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
