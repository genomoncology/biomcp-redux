---
title: biomcp sync wikipathways
description: "The BioMCP 1.0.0 command biomcp sync wikipathways: Downloads the
  WikiPathways bundle that pathway searches read."
goal: Look up the arguments, sections and examples of biomcp sync wikipathways.
reader: A BioMCP user or agent checking one command
sidebar:
  label: sync wikipathways
kind: reference
verified: 2026-10-27
---

`biomcp sync wikipathways` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It downloads the WikiPathways [bundle](/local-data/) that pathway searches read. This page lists its examples and reply.

The program's help describes it:

> Replace WikiPathways search/get metadata catalogs atomically; reads never sync. Membership names a source and release; it implies no therapy, response or actionability. No enrichment is calculated.

## Usage

```text
biomcp sync wikipathways
```

## Source

The bundle it downloads is the local copy that [WikiPathways](/sources/wikipathways/) reads. Search and get commands for that source answer only after this command has run once. [Local data](/local-data/#bundle-sizes) lists each bundle's size.

## Examples

The program's help gives these examples.

```sh
biomcp sync wikipathways
```

```sh
biomcp sync wikipathways --fresh
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.wikipathways_sync/1`](/reference/schemas/biomcp-wikipathways-sync-1/), whose page shows a real reply and its fields.

## Next

[Local data](/local-data/) downloads bundles and cleans the store. The [commands index](/reference/commands/) lists every command.
