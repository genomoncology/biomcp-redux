---
title: biomcp search protein
description: "The BioMCP 1.0.0 command biomcp search protein: Finds reviewed
  human UniProt proteins by text or by gene, five at a time."
goal: Look up the arguments, sections and examples of biomcp search protein.
reader: A BioMCP user or agent checking one command
sidebar:
  label: search protein
kind: reference
verified: 2026-10-27
---

`biomcp search protein` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It finds reviewed human UniProt proteins by text or by gene, five at a time. This page lists its arguments, examples and reply.

The program's help describes it:

> Search five reviewed human UniProt headers by literal text or exact gene.
> No variant effect, structure, interaction or clinical meaning is inferred. No bioinformatics tool runs.

## Usage

```text
biomcp search protein [OPTIONS] [query]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--gene` | string | no |  |  | Exact native gene field, without identifier inference. |
| `--limit` | integer | no | min 1, max 25 |  | Five rows by default; no more than 25. |
| `--offset` | integer | no | min 0 |  | Source/retained result offset. |
| `--pool` | string | no |  |  | Exact retained collection identity. |
| `--reviewed` | boolean | no |  |  | Reviewed human scope stays enabled; unreviewed search is deferred. |
| `query` (position 1) | string | no |  |  | Literal text, exclusive with gene. |

## Examples

The program's help gives these examples.

```sh
biomcp search protein --gene BRAF
```

```sh
biomcp search protein EGFR
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.protein_page/1`](/reference/schemas/biomcp-protein-page-1/), whose page shows a real reply and its fields.

## Next

[Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes. The [protein guide](/user-guide/protein/) shows every protein command at work.
