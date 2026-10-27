---
title: biomcp pathway genes
description: "The BioMCP 1.0.0 command biomcp pathway genes: Lists the genes and
  gene products of one pathway."
goal: Look up the arguments, sections and examples of biomcp pathway genes.
reader: A BioMCP user or agent checking one command
sidebar:
  label: pathway genes
kind: reference
verified: 2026-10-27
---

`biomcp pathway genes` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It lists the genes and gene products of one pathway. This page lists its arguments, examples and reply.

The program's help describes it:

> Read Reactome exact references or WikiPathways GeneProduct data nodes. The details layer discloses complete Reactome physical contexts. Membership names a source and release; it implies no therapy, response or actionability. No enrichment is calculated.

## Usage

```text
biomcp pathway genes [OPTIONS] <id> [details]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--limit` | integer | no | min 1, max 50 |  | Five complete relation rows by default, at most fifty. |
| `--offset` | integer | no | min 0 |  | Offset within the retained selected relation. |
| `--pool` | string | no |  |  | Immutable relation snapshot. |
| `id` (position 1) | string | yes |  |  | Checked source-owned human pathway identity. |
| `details` (position 2) | string | no |  | `details` | Include every physical entity context for these references; never clips source text. |

## Examples

The program's help gives these examples.

```sh
biomcp pathway genes R-HSA-5673001
```

```sh
biomcp pathway genes R-HSA-5673001 details
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.pathway_genes/4`](/reference/schemas/biomcp-pathway-genes-4/), whose page shows a real reply and its fields.

## Next

[Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes. The [pathway guide](/user-guide/pathway/) shows every pathway command at work.
