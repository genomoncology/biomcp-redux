---
title: biomcp pathway
description: "The BioMCP 1.0.0 command biomcp pathway: Read pathway genes and
  source events."
goal: Look up the arguments, sections and examples of biomcp pathway.
reader: A BioMCP user or agent checking one command
sidebar:
  label: pathway
kind: reference
verified: 2026-11-09
---

`biomcp pathway` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It read pathway genes and source events. This page lists its examples and reply.

The program's help describes it:

> Read pathway genes and source events.

## Usage

```text
biomcp pathway
```

## Examples

The program's help gives this example.

```sh
biomcp pathway genes R-HSA-5673001
```

## Reply

The reply is one JSON object. Its schema is one of [`biomcp.pathway-events/6`](/reference/schemas/biomcp-pathway-events-6/), [`biomcp.pathway-genes/9`](/reference/schemas/biomcp-pathway-genes-9/), whose page lists its fields.

## Next

The [pathway guide](/user-guide/pathway/) shows every pathway command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
