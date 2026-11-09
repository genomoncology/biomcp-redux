---
title: biomcp search protein
description: "The BioMCP 1.0.0 command biomcp search protein: Finds reviewed
  human UniProt proteins by text or by gene, five at a time."
goal: Look up the arguments, sections and examples of biomcp search protein.
reader: A BioMCP user or agent checking one command
sidebar:
  label: search protein
kind: reference
verified: 2026-11-09
---

`biomcp search protein` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It finds reviewed human UniProt proteins by text or by gene, five at a time. This page lists its arguments, examples and reply.

The program's help describes it:

> Search reviewed human proteins in UniProt. Sources: uniprot.

## Usage

```text
biomcp search protein [OPTIONS] [query]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--gene` | string | no |  |  | Exact source gene field, without identifier inference. Value form: string. Example: --gene BRAF. Command sources: uniprot; source restrictions in the field description apply. |
| `--limit` | integer | no | min 1, max 25 |  | Five rows by default; no more than 25. Value form: integer. Example: --limit 25. Command sources: uniprot; source restrictions in the field description apply. |
| `--offset` | integer | no | min 0 |  | Source/retained result offset. Value form: integer. Example: --offset 0. Command sources: uniprot; source restrictions in the field description apply. |
| `--pool` | string | no |  |  | Exact retained collection identity. Value form: string. Example: --pool &lt;pool&gt; (syntax; replace the placeholder with your retained value). Command sources: uniprot; source restrictions in the field description apply. |
| `--reviewed` | boolean | no |  |  | Reviewed human scope stays enabled; unreviewed search is deferred. Value form: boolean. Example: --reviewed. Command sources: uniprot; source restrictions in the field description apply. |
| `query` (position 1) | string | no |  |  | Exact text, exclusive with gene. |

## Examples

The program's help gives these examples.

```sh
biomcp search protein --gene BRAF
```

```sh
biomcp search protein EGFR
```

```sh
biomcp search protein --gene EGFR --reviewed
```

```sh
biomcp search protein EGFR --limit 25
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.protein-page/5`](/reference/schemas/biomcp-protein-page-5/), whose page lists its fields.

## Next

The [protein guide](/user-guide/protein/) shows every protein command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
