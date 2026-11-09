---
title: MCP server tools
description: "The three MCP tools that BioMCP 1.0.0 serves: biomcp, search and
  fetch, with their input schemas."
goal: Look up the MCP tools, their arguments and descriptions.
reader: A developer connecting an MCP client to BioMCP
kind: reference
verified: 2026-11-09
---

`biomcp serve` runs BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources, as an MCP server. An agent that connects asks it for its tools with `tools/list`, and it answers with 3, listed below with their arguments and the description each one sends. [Connect an agent](/getting-started/mcp-clients/) shows how to add the server to a client.

## biomcp

| Argument | Type | Required |
| --- | --- | --- |
| `args` | array of string | yes |

Annotations: `destructiveHint` false, `openWorldHint` true, `readOnlyHint` false.

Description sent to the client, with its lines wrapped at 80 characters:

```text
Run a BioMCP CLI command. arguments has exactly one field: args, an array of
command words (strings), without the biomcp executable. Example:
{"args":["help"]}. Start with help for the command map; use help <command> for
options and schema <id> for a complete reply schema. Replies are JSON source
evidence with status, problems and next commands; inspect them before
interpreting results.
```

## search

| Argument | Type | Required |
| --- | --- | --- |
| `query` | string | yes |

Annotations: `destructiveHint` false, `openWorldHint` true, `readOnlyHint` true.

Description sent to the client, with its lines wrapped at 80 characters:

```text
Search BioMCP source headers. Prefix a CLI entity and its arguments, or supply
article query text.
```

## fetch

| Argument | Type | Required |
| --- | --- | --- |
| `id` | string | yes |

Annotations: `destructiveHint` false, `openWorldHint` true, `readOnlyHint` true.

Description sent to the client, with its lines wrapped at 80 characters:

```text
Fetch a search result using its exact id. Returns the CLI record as Markdown
with source metadata.
```

## Typed entity tools

`biomcp serve --tool-mode typed` exposes 17 tools. Each [entity](/concepts/progressive-disclosure/) tool takes an `action` and an object named for that action. `single` remains the default mode.

| Tool | Actions |
| --- | --- |
| `gene` | `search`, `get`, `pgx`, `pgx_get`, `gwas` |
| `variant` | `search`, `get`, `normalize`, `assertions`, `population`, `recurrence`, `gwas`, `gwas_get` |
| `disease` | `search`, `get`, `gwas` |
| `drug` | `search`, `get`, `pgx_get` |
| `trial` | `search`, `get`, `document`, `organization_search`, `biomarker_search`, `biomarker_get` |
| `article` | `search`, `get`, `text`, `pdf`, `supplement`, `citations`, `references`, `citation_evidence`, `recommendations`, `author_search`, `author_get`, `author_papers` |
| `protein` | `search`, `get`, `features`, `isoforms`, `domains`, `complexes`, `complex_members`, `structures`, `sequence` |
| `pathway` | `search`, `get`, `genes`, `events` |
| `phenotype` | `search`, `get` |
| `adverse_event` | `search`, `get` |
| `diagnostic` | `search`, `get` |
| `cell_line` | `search`, `get`, `assays` |
| `study` | `search`, `get`, `gwas`, `gwas_get` |
| `help` |  |
| `biomcp` |  |
| `search` |  |
| `fetch` |  |

The configured source restrictions apply in both modes.
