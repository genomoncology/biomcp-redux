---
title: Reference
description: "Generated reference for BioMCP 1.0.0: commands, entities and
  sections, sources, global options, exit codes and MCP tools."
goal: Find the generated reference page for a command, entity, source, option,
  exit code or MCP tool.
reader: A BioMCP user or agent looking up an exact fact
kind: reference
verified: 2026-10-27
---

The BioMCP 1.0.0 reference is generated from the program's own help and registry. It covers:

- [Commands](/reference/commands/): all 85 commands, one page each.
- [Entities and sections](/reference/entities/): the 16 entities and their section words.
- [Sources](/reference/data-sources/): the 56 sources, their terms and keys.
- [Global options](/reference/global-options/): options every command accepts.
- [Reply envelope](/reference/envelope/): statuses, source states, reasons, use groups and error codes.
- [Exit codes](/reference/error-codes/): what each exit code means.
- [Reply schemas](/reference/schemas/): the 119 reply schemas and the commands that return them.
- [Command map from 0.9](/reference/upgrade-map/): what happened to each 0.9 command.
- [MCP tools](/reference/mcp-server/): the 3 tools the server offers.

Run `biomcp help` for the same command data as JSON, or `biomcp help get gene` for one command. `biomcp schema ID` prints a reply's JSON schema, such as `biomcp schema biomcp.gene_cards/4`.

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
