---
title: biomcp help exit
description: |-
  The BioMCP 1.0.0 command biomcp help exit: Process exit codes:
  0: complete or partial reply
  1: no matching record
  2: invalid command or arguments
  3: expired result pool
  4: source request failed
  5: local configuration, storage or output failed
  Error codes: usage.schema, usage.arguments, usage.mcp_una
goal: Look up the arguments, sections and examples of biomcp help exit.
reader: A BioMCP user or agent checking one command
sidebar:
  label: help exit
kind: reference
verified: 2026-11-09
---

`biomcp help exit` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It process exit codes:
0: complete or partial reply
1: no matching record
2: invalid command or arguments
3: expired result pool
4: source request failed
5: local configuration, storage or output failed
Error codes: usage.schema, usage.arguments, usage.mcp_unavailable, usage.unknown_command, usage.retired, local.failure, pool.expired, usage.pool_mismatch, usage.too_many_pmids, usage.too_broad, usage.unsupported_filter. This page lists its reply.

The program's help describes it:

> Process exit codes:
> 0: complete or partial reply
> 1: no matching record
> 2: invalid command or arguments
> 3: expired result pool
> 4: source request failed
> 5: local configuration, storage or output failed
> Error codes: usage.schema, usage.arguments, usage.mcp_unavailable, usage.unknown_command, usage.retired, local.failure, pool.expired, usage.pool_mismatch, usage.too_many_pmids, usage.too_broad, usage.unsupported_filter. Source problems and reasons appear in their reply fields.

## Usage

```text
biomcp help exit
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.help/6`](/reference/schemas/biomcp-help-6/), whose page lists its fields.

## Next

The [commands index](/reference/commands/) lists every command.
