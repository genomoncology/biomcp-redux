---
title: Reference
description: "The BioMCP 1.0.0 reference: commands, entities and sections,
  sources, global options, exit codes, reply schemas and MCP tools."
goal: Find the reference page for a command, entity, source, option, exit code
  or MCP tool.
reader: A BioMCP user or agent looking up an exact fact
kind: reference
verified: 2026-10-27
---

The reference of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources gives the exact facts behind each guide, taken from the program's own help and source list. It covers:

- [Commands](/reference/commands/): all 86 commands, one page each.
- [Entities and sections](/reference/entities/): the 16 kinds of record and the parts each one can add.
- [Sources](/reference/data-sources/): the 57 sources, their terms and keys.
- [Global options](/reference/global-options/): options every command accepts.
- [Reply envelope](/reference/envelope/): statuses, source states, reasons, use groups and error codes.
- [Exit codes](/reference/error-codes/): what each exit code means.
- [Reply schemas](/reference/schemas/): the 119 reply schemas and the commands that return them.
- [Command map from 0.9](/reference/upgrade-map/): what happened to each 0.9 command.
- [MCP tools](/reference/mcp-server/): the 3 tools the server offers.

Run `biomcp help` for the same command data as JSON, or `biomcp help get gene` for one command. `biomcp schema ID` prints a reply's JSON schema, such as `biomcp schema biomcp.gene_cards/4`.
