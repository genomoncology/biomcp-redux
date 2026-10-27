---
title: biomcp get pathway
description: "The BioMCP 1.0.0 command biomcp get pathway: Reads one Reactome or
  WikiPathways pathway, and optionally its summary, genes or events."
goal: Look up the arguments, sections and examples of biomcp get pathway.
reader: A BioMCP user or agent checking one command
sidebar:
  label: get pathway
kind: reference
verified: 2026-10-27
---

`biomcp get pathway` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It reads one Reactome or WikiPathways pathway, and optionally its summary, genes or events. A [section word](/concepts/progressive-disclosure/) after the identifier adds one more part of the record to the reply. This page lists its arguments, sections and examples.

The program's help describes it:

> Get an exact human Reactome R-HSA or WikiPathways WP card. Read whole summary, genes, Reactome events or bounded all sections. Membership names a source and release; it implies no therapy, response or actionability. No enrichment is calculated.

## Usage

```text
biomcp get pathway [OPTIONS] <id> [section]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--fresh` | boolean | no |  |  | Request a new source observation. |
| `--pool` | string | no |  |  | Retained snapshot, when continuing a source relation. |
| `--section-limit` | integer | no | min 1, max 50 |  | Bounded initial section size; each all section has its own first page. |
| `--section-offset` | integer | no | min 0 |  | Native relation offset for a selected genes or events section. |
| `id` (position 1) | string | yes |  |  | Exact human Reactome or WikiPathways identity without version suffixes. |
| `section` (position 2) | string | no |  | `summary`, `genes`, `events`, `all` | Omit for the card; WikiPathways has summary and genes sections. |

## Sections

Omit for the card; WikiPathways has summary and genes sections.

| Section | Description |
| --- | --- |
| `summary` | Complete native source description. |
| `genes` | Namespaced native participants with exact resolved gene joins. |
| `events` | Native Reactome event objects and unresolved database references. |
| `all` | Each source-supported section's bounded first page. |

## Examples

The program's help gives these examples.

```sh
biomcp get pathway R-HSA-9652817
```

```sh
biomcp get pathway R-HSA-177929
```

## Next

The [pathway guide](/user-guide/pathway/) shows every pathway command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
