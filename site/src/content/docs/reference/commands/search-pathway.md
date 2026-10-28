---
title: biomcp search pathway
description: "The BioMCP 1.0.0 command biomcp search pathway: Finds human
  pathways in Reactome, the default, or WikiPathways."
goal: Look up the arguments, sections and examples of biomcp search pathway.
reader: A BioMCP user or agent checking one command
sidebar:
  label: search pathway
kind: reference
verified: 2026-10-28
---

`biomcp search pathway` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It finds human pathways in Reactome, the default, or WikiPathways. This page lists its arguments, examples and reply.

The program's help describes it:

> Search Reactome (default) or WikiPathways human pathways. Membership names a source and release; it implies no therapy, response or actionability. No enrichment is calculated.

## Usage

```text
biomcp search pathway [OPTIONS] [query]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--fresh` | boolean | no |  |  | Request a new retained observation. |
| `--gene` | string | no |  |  | Human gene input for exact resolution and UniProt mapping; Reactome only. |
| `--limit` | integer | no | min 1, max 50 |  | Five rows by default, one to fifty. Reactome rows equals this limit without refill. |
| `--offset` | integer | no | min 0 |  | Native Reactome Start row or retained WikiPathways catalog offset. |
| `--pool` | string | no |  |  | Retained snapshot identity for deterministic continuation. |
| `--source` | array | no | min 1, max 2 | `reactome`, `wikipathways` | Sources in caller order; repeated sources take turns without title joins. |
| `--top-level` | boolean | no |  |  | Native human Reactome top-level discovery, exclusive with text and gene. |
| `query` (position 1) | string | no |  |  | Literal source query; exclusive with gene and top-level discovery. |

## Sources

`--source` picks the source to ask: [reactome](/sources/reactome/), [wikipathways](/sources/wikipathways/).

## Examples

The program's help gives these examples.

```sh
biomcp search pathway 'Signaling by EGFR'
```

```sh
biomcp search pathway Hemostasis
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.pathway_page/3`](/reference/schemas/biomcp-pathway-page-3/), whose page shows a real reply and its fields.

## Next

The [pathway guide](/user-guide/pathway/) shows every pathway command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
