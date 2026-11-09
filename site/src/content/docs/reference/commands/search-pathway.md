---
title: biomcp search pathway
description: "The BioMCP 1.0.0 command biomcp search pathway: Finds human
  pathways in Reactome, the default, or WikiPathways."
goal: Look up the arguments, sections and examples of biomcp search pathway.
reader: A BioMCP user or agent checking one command
sidebar:
  label: search pathway
kind: reference
verified: 2026-11-09
---

`biomcp search pathway` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It finds human pathways in Reactome, the default, or WikiPathways. This page lists its arguments, examples and reply.

The program's help describes it:

> Search human biological pathways. Sources: reactome, wikipathways.

## Usage

```text
biomcp search pathway [OPTIONS] [query]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--fresh` | boolean | no |  |  | Request a new retained observation. Value form: boolean. Example: --fresh. Command sources: reactome, wikipathways; source restrictions in the field description apply. |
| `--gene` | string | no |  |  | Human gene input for exact resolution and UniProt mapping; Reactome only. Value form: string. Example: --gene HGNC:1097. Command sources: reactome, wikipathways; source restrictions in the field description apply. |
| `--limit` | integer | no | min 1, max 50 |  | Five rows by default, one to fifty. Reactome rows equals this limit without refill. Value form: integer. Example: --limit 5. Command sources: reactome, wikipathways; source restrictions in the field description apply. |
| `--offset` | integer | no | min 0 |  | Source Reactome Start row or retained WikiPathways catalog offset. Value form: integer. Example: --offset 0. Command sources: reactome, wikipathways; source restrictions in the field description apply. |
| `--pool` | string | no |  |  | Retained snapshot identity for deterministic continuation. Value form: string. Example: --pool &lt;pool&gt; (syntax; replace the placeholder with your retained value). Command sources: reactome, wikipathways; source restrictions in the field description apply. |
| `--source` | array | no | min 1, max 2 | `reactome`, `wikipathways` | Sources in caller order; repeated sources take turns; help search pathway Sources in caller order; repeated sources take turns without title joins. Listed choices accept any letter case; replies use their listed spelling. Value form: repeatable values (repeat the flag). Example: --source reactome. Command sources: reactome, wikipathways; source restrictions in the field description apply. |
| `--top-level` | boolean | no |  |  | Source human Reactome top-level discovery, exclusive with text and gene. Value form: boolean. Example: --top-level. Command sources: reactome, wikipathways; source restrictions in the field description apply. |
| `query` (position 1) | string | no |  |  | Exact source query; exclusive with gene and top-level discovery. |

## Sources

`--source` picks the source to ask: [reactome](/sources/reactome/), [wikipathways](/sources/wikipathways/).

## Examples

The program's help gives these examples.

```sh
biomcp search pathway 'Signaling by EGFR'
```

```sh
biomcp search pathway Hemostasis
```

```sh
biomcp search pathway --gene HGNC:1097
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.pathway-page/8`](/reference/schemas/biomcp-pathway-page-8/), whose page lists its fields.

## Next

The [pathway guide](/user-guide/pathway/) shows every pathway command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
