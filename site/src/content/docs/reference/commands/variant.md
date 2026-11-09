---
title: biomcp variant
description: "The BioMCP 1.0.0 command biomcp variant: Read source variant
  assertions, populations and recurrence."
goal: Look up the arguments, sections and examples of biomcp variant.
reader: A BioMCP user or agent checking one command
sidebar:
  label: variant
kind: reference
verified: 2026-11-09
---

`biomcp variant` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It read source variant assertions, populations and recurrence. This page lists its examples and reply.

The program's help describes it:

> Read source variant assertions, populations and recurrence.

## Usage

```text
biomcp variant
```

## Examples

The program's help gives this example.

```sh
biomcp variant assertions clinvar:13961 --source clinvar
```

## Reply

The reply is one JSON object. Its schema is one of [`biomcp.variant-assertions/8`](/reference/schemas/biomcp-variant-assertions-8/), [`biomcp.variant-normalization/8`](/reference/schemas/biomcp-variant-normalization-8/), [`biomcp.variant-population/13`](/reference/schemas/biomcp-variant-population-13/), [`biomcp.variant-recurrence/11`](/reference/schemas/biomcp-variant-recurrence-11/), whose page lists its fields.

## Next

The [variant guide](/user-guide/variant/) shows every variant command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
