---
title: biomcp schema
description: "The BioMCP 1.0.0 command biomcp schema: Prints one reply schema as
  JSON Schema."
goal: Look up the arguments, sections and examples of biomcp schema.
reader: A BioMCP user or agent checking one command
sidebar:
  label: schema
kind: reference
verified: 2026-11-09
---

`biomcp schema` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It prints one reply schema as JSON Schema. This page lists its arguments, examples and reply.

The program's help describes it:

> List every schema identity and description with biomcp schema, or print one schema artifact with biomcp schema &lt;id&gt;.

## Usage

```text
biomcp schema [artifact-id]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `artifact-id` (position 1) | string | no |  |  | Artifact ID to print; omit it to list available schemas. |

## Examples

The program's help gives these examples.

```sh
biomcp schema
```

```sh
biomcp schema biomcp.help/6
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.schema-catalog/4`](/reference/schemas/biomcp-schema-catalog-4/), whose page lists its fields.

## Next

[Reply schemas](/reference/schemas/) lists every reply schema. The [commands index](/reference/commands/) lists every command.
