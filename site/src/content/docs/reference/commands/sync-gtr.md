---
title: biomcp sync gtr
description: "Reference for the BioMCP 1.0.0 command biomcp sync gtr: Replace
  the complete validated local bundle atomically."
goal: Look up the arguments, sections and examples of biomcp sync gtr.
reader: A BioMCP user or agent checking one command
sidebar:
  label: sync gtr
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 command `biomcp sync gtr`.

Replace the complete validated local bundle atomically. Reads use stored data and never synchronize. Failed replacement preserves the last usable bundle.

## Usage

```text
biomcp sync gtr
```

## Arguments

The help lists no arguments for this command.

## Examples

```sh
biomcp sync gtr
biomcp sync gtr --fresh
```

## Reply schema

The help examples of this command return [`biomcp.sync/4`](/reference/schemas/biomcp-sync-4/). Print the full JSON schema with `biomcp schema biomcp.sync/4`.

## Exit codes

See [exit codes](/reference/error-codes/). Options that apply to every command are in [global options](/reference/global-options/).

## Next

The [commands index](/reference/commands/) lists every command.

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
