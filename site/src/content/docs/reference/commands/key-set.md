---
title: biomcp key set
description: "The BioMCP 1.0.0 command biomcp key set: Stores a key for one
  source, read from a hidden prompt or standard input."
goal: Look up the arguments, sections and examples of biomcp key set.
reader: A BioMCP user or agent checking one command
sidebar:
  label: key set
kind: reference
verified: 2026-11-09
---

`biomcp key set` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It stores a key for one source, read from a hidden prompt or standard input. This page lists its arguments, examples and reply.

The program's help describes it:

> Register one credential from a no-echo prompt or one standard-input line. Use --config PATH to choose the configuration file.

## Usage

```text
biomcp key set [OPTIONS] <source>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--replace` | boolean | no |  | `true`, `false` | Replace an existing stored credential. Value form: boolean. Example: --replace. Local command option; independent of source selection. |
| `source` (position 1) | string | yes |  | `ncbi`, `semantic-scholar`, `orcid`, `oncokb`, `alphagenome`, `nci`, `openfda` | Source credential name to register or remove. Listed choices accept any letter case; replies use their listed spelling. |

## Examples

The program's help gives this example.

```sh
biomcp key set oncokb
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.registered-keys/5`](/reference/schemas/biomcp-registered-keys-5/), whose page lists its fields.

## Next

[Keys and configuration](/getting-started/api-keys/) sets keys, a proxy and a certificate [bundle](/local-data/). The [commands index](/reference/commands/) lists every command.
