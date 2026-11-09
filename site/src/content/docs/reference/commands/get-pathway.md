---
title: biomcp get pathway
description: "The BioMCP 1.0.0 command biomcp get pathway: Reads one Reactome or
  WikiPathways pathway, and optionally its summary, genes or events."
goal: Look up the arguments, sections and examples of biomcp get pathway.
reader: A BioMCP user or agent checking one command
sidebar:
  label: get pathway
kind: reference
verified: 2026-11-09
---

`biomcp get pathway` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It reads one Reactome or WikiPathways pathway, and optionally its summary, genes or events. A [section word](/concepts/progressive-disclosure/) after the identifier adds one more part of the record to the reply. This page lists its arguments, sections, examples and reply.

The program's help describes it:

> Get human pathway details. Sources: reactome, wikipathways.

## Usage

```text
biomcp get pathway [OPTIONS] <id> [section]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--fresh` | boolean | no |  |  | Request a new source observation. Value form: boolean. Example: --fresh. Command sources: reactome, wikipathways; source restrictions in the field description apply. |
| `--pool` | string | no |  |  | Retained snapshot, when continuing a source relation. Value form: string. Example: --pool &lt;pool&gt; (syntax; replace the placeholder with your retained value). Command sources: reactome, wikipathways; source restrictions in the field description apply. |
| `--section-limit` | integer | no | min 1, max 50 |  | Bounded initial section size; each all section has its own first page. Value form: integer. Example: --section-limit 5. Command sources: reactome, wikipathways; source restrictions in the field description apply. |
| `--section-offset` | integer | no | min 0 |  | Source relation offset for a selected genes or events section. Value form: integer. Example: --section-offset 0. Command sources: reactome, wikipathways; source restrictions in the field description apply. |
| `id` (position 1) | string | yes |  |  | Exact human Reactome or WikiPathways identity without version suffixes. |
| `section` (position 2) | string | no |  | `summary`, `genes`, `events`, `all` | Omit for the card; WikiPathways has summary and genes; help get pathway Listed choices accept any letter case; replies use their listed spelling. |

## Sections

Omit for the card; WikiPathways has summary and genes sections.

| Section | Description |
| --- | --- |
| `summary` | Complete source description. |
| `genes` | Namespaced source participants with exact resolved gene joins. |
| `events` | Source Reactome event objects and unresolved database references. |
| `all` | Each source-supported section's bounded first page. |

## Examples

The program's help gives these examples.

```sh
biomcp get pathway R-HSA-9652817
```

```sh
biomcp get pathway R-HSA-177929
```

```sh
biomcp get pathway R-HSA-5673001 --format json
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.pathway-card/10`](/reference/schemas/biomcp-pathway-card-10/), whose page lists its fields.

## Next

The [pathway guide](/user-guide/pathway/) shows every pathway command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
