---
title: biomcp trial
description: "The BioMCP 1.0.0 command biomcp trial: Read trial groups and
  public documents."
goal: Look up the arguments, sections and examples of biomcp trial.
reader: A BioMCP user or agent checking one command
sidebar:
  label: trial
kind: reference
verified: 2026-11-09
---

`biomcp trial` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It read trial groups and public documents. This page lists its examples and reply.

The program's help describes it:

> Read trial groups and public documents.

## Usage

```text
biomcp trial
```

## Examples

The program's help gives this example.

```sh
biomcp trial document NCT04303780 SAP_001.pdf --save protocol.pdf --replace
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.trial-document/7`](/reference/schemas/biomcp-trial-document-7/), whose page lists its fields.

## Next

The [trial guide](/user-guide/trial/) shows every trial command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
