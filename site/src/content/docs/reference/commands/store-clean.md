---
title: biomcp store clean
description: "The BioMCP 1.0.0 command biomcp store clean: Removes expired
  replies and stored searches from the local cache."
goal: Look up the arguments, sections and examples of biomcp store clean.
reader: A BioMCP user or agent checking one command
sidebar:
  label: store clean
kind: reference
verified: 2026-10-27
---

`biomcp store clean` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It removes expired replies and stored searches from the local cache. This page lists its reply.

The program's help describes it:

> Remove expired replies, pools and snapshots and older incompatible cache files. Return removal counts in one bounded reply. Fresh state and [bundles](/local-data/) remain.

## Usage

```text
biomcp store clean
```

## Arguments

The command takes no arguments beyond those in its usage line.

## Reply

The reply is one JSON object. Its schema is [`biomcp.store-clean/1`](/reference/schemas/biomcp-store-clean-1/), whose page shows a real reply and its fields.

## Next

[Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes. The [commands index](/reference/commands/) lists every command.
