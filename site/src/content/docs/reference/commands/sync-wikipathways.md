---
title: biomcp sync wikipathways
description: "Reference for the BioMCP 1.0.0 command biomcp sync wikipathways:
  Replace WikiPathways search/get metadata catalogs atomically; reads never
  sync."
goal: Look up the arguments, sections and examples of biomcp sync wikipathways.
reader: A BioMCP user or agent checking one command
sidebar:
  label: sync wikipathways
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 command `biomcp sync wikipathways`.

Replace WikiPathways search/get metadata catalogs atomically; reads never sync. Membership names a source and release; it implies no therapy, response or actionability. No enrichment is calculated.

## Usage

```text
biomcp sync wikipathways
```

## Arguments

The help lists no arguments for this command.

## Examples

```sh
biomcp sync wikipathways
biomcp sync wikipathways --fresh
```

## Reply schema

The help examples of this command return [`biomcp.wikipathways_sync/1`](/reference/schemas/biomcp-wikipathways-sync-1/). Print the full JSON schema with `biomcp schema biomcp.wikipathways_sync/1`.

## Exit codes

See [exit codes](/reference/error-codes/). Options that apply to every command are in [global options](/reference/global-options/).

## Next

The [commands index](/reference/commands/) lists every command.

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
