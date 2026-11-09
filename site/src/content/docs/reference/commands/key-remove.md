---
title: biomcp key remove
description: "The BioMCP 1.0.0 command biomcp key remove: Removes a stored key."
goal: Look up the arguments, sections and examples of biomcp key remove.
reader: A BioMCP user or agent checking one command
sidebar:
  label: key remove
kind: reference
verified: 2026-11-09
---

`biomcp key remove` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It removes a stored key. This page lists its arguments, examples and reply.

The program's help describes it:

> Remove a stored credential while preserving other settings.

## Usage

```text
biomcp key remove <source>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `source` (position 1) | string | yes |  | `ncbi`, `semantic-scholar`, `orcid`, `oncokb`, `alphagenome`, `nci`, `openfda` | Source credential name to register or remove. Listed choices accept any letter case; replies use their listed spelling. |

## Examples

The program's help gives this example.

```sh
biomcp key remove oncokb
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.registered-keys/5`](/reference/schemas/biomcp-registered-keys-5/), whose page lists its fields.

## Next

[Keys and configuration](/getting-started/api-keys/) sets keys, a proxy and a certificate [bundle](/local-data/). The [commands index](/reference/commands/) lists every command.
