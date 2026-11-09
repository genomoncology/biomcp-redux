---
title: biomcp help key
description: "The BioMCP 1.0.0 command biomcp help key: Register and manage
  source credentials without showing values."
goal: Look up the arguments, sections and examples of biomcp help key.
reader: A BioMCP user or agent checking one command
sidebar:
  label: help key
kind: reference
verified: 2026-11-09
---

`biomcp help key` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It register and manage source credentials without showing values. This page lists its arguments and reply.

The program's help describes it:

> Register and manage source credentials without showing values.

## Usage

```text
biomcp help key [source]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `source` (position 1) | string | no |  | `ncbi`, `semantic-scholar`, `orcid`, `oncokb`, `alphagenome`, `nci`, `openfda` | Credential source to describe. Listed choices accept any letter case; replies use their listed spelling. |

## Reply

The reply is one JSON object. Its schema is [`biomcp.key/5`](/reference/schemas/biomcp-key-5/), whose page lists its fields.

## Next

[Keys and configuration](/getting-started/api-keys/) sets keys, a proxy and a certificate [bundle](/local-data/). The [commands index](/reference/commands/) lists every command.
