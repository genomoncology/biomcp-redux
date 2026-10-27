---
title: biomcp search drug
description: "The BioMCP 1.0.0 command biomcp search drug: Finds drugs by name
  in one source."
goal: Look up the arguments, sections and examples of biomcp search drug.
reader: A BioMCP user or agent checking one command
sidebar:
  label: search drug
kind: reference
verified: 2026-10-27
---

`biomcp search drug` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It finds drugs by name in one source. This page lists its arguments, examples and reply.

The program's help describes it:

> Literal single-source candidate discovery, without an implicit identity selection.

## Usage

```text
biomcp search drug [OPTIONS] <query>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--limit` | integer | no | min 1, max 50 |  | Native candidate rows per page. |
| `--name-selector` | string | no |  | `preferred`, `synonym` | Explicit ChEMBL native name selector; omitted means preferred name. |
| `--offset` | integer | no | min 0 |  | Native rows to skip. |
| `--source` | string | no |  | `who-pq`, `mychem`, `chembl`, `opentargets`, `civic`, `openfda-label`, `drugsfda`, `ema`, `ddinter` | Exactly one source; default MyChem never fans out. |
| `query` (position 1) | string | yes |  |  | Literal source name selector, unchanged by the consumer. |

## Sources

`--source` picks the source to ask: [who-pq](/sources/who-prequalification/), [mychem](/sources/mychem/), [chembl](/sources/chembl/), [opentargets](/sources/opentargets/), [civic](/sources/civic/), [openfda-label](/sources/openfda/), [drugsfda](/sources/drugsfda/), [ema](/sources/ema/), [ddinter](/sources/ddinter/).

## Examples

The program's help gives these examples.

```sh
biomcp search drug Tibsovo --source ema --limit 1
```

```sh
biomcp search drug Tibsovo --source ema --limit 1 --offset 1 --fresh
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.drug_page/3`](/reference/schemas/biomcp-drug-page-3/), whose page shows a real reply and its fields.

## Next

[Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes. The [drug guide](/user-guide/drug/) shows every drug command at work.
