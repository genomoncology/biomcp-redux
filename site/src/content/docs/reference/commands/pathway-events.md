---
title: biomcp pathway events
description: "The BioMCP 1.0.0 command biomcp pathway events: Lists the events,
  such as reactions, of one Reactome pathway."
goal: Look up the arguments, sections and examples of biomcp pathway events.
reader: A BioMCP user or agent checking one command
sidebar:
  label: pathway events
kind: reference
verified: 2026-10-27
---

`biomcp pathway events` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It lists the events, such as reactions, of one Reactome pathway. This page lists its arguments, examples and reply.

The program's help describes it:

> Read Reactome event objects and unresolved native integer references in source order. Membership names a source and release; it implies no therapy, response or actionability. No enrichment is calculated.

## Usage

```text
biomcp pathway events [OPTIONS] <id>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--limit` | integer | no | min 1, max 50 |  | Five complete relation rows by default, at most fifty. |
| `--offset` | integer | no | min 0 |  | Offset within the retained selected relation. |
| `--pool` | string | no |  |  | Immutable relation snapshot. |
| `id` (position 1) | string | yes |  |  | Checked source-owned human pathway identity. |

## Examples

The program's help gives these examples.

```sh
biomcp pathway events R-HSA-5673001
```

```sh
biomcp pathway events R-HSA-5673001 --limit 50
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.pathway_events/2`](/reference/schemas/biomcp-pathway-events-2/), whose page shows a real reply and its fields.

## Next

The [pathway guide](/user-guide/pathway/) shows every pathway command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
