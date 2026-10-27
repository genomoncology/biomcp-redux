---
title: biomcp help key oncokb
description: "The BioMCP 1.0.0 command biomcp help key oncokb: Shows how to set
  the OncoKB key: the key name and its environment variable."
goal: Look up the arguments, sections and examples of biomcp help key oncokb.
reader: A BioMCP user or agent checking one command
sidebar:
  label: help key oncokb
kind: reference
verified: 2026-10-27
---

`biomcp help key oncokb` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It shows how to set the OncoKB key: the key name and its environment variable. This page lists its reply.

The program's help describes it:

> Show credential registration and environment fallback names.

## Usage

```text
biomcp help key oncokb
```

## Arguments

The command takes no arguments beyond those in its usage line.

## The key

`biomcp help key oncokb` reports these settings for the oncokb key:

| Setting | Value |
| --- | --- |
| Set it with | `biomcp key set oncokb` |
| Source it unlocks | [oncokb](/sources/oncokb/) |
| Field in config.json | `keys.oncokb_api_key` |
| Environment variable | `ONCOKB_TOKEN` |

[Keys and configuration](/getting-started/api-keys/) shows how to set it.

## Reply

The reply is one JSON object. Its schema is [`biomcp.key/2`](/reference/schemas/biomcp-key-2/), whose page shows a real reply and its fields.

## Next

The [commands index](/reference/commands/) lists every command.
