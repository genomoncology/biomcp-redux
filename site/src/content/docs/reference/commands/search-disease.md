---
title: biomcp search disease
description: "The BioMCP 1.0.0 command biomcp search disease: Finds disease
  concepts by text, or lists SEER cancer sites."
goal: Look up the arguments, sections and examples of biomcp search disease.
reader: A BioMCP user or agent checking one command
sidebar:
  label: search disease
kind: reference
verified: 2026-11-09
---

`biomcp search disease` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It finds disease concepts by text, or lists SEER cancer sites. This page lists its arguments, examples and reply.

The program's help describes it:

> Search diseases and their identifiers. Sources: monarch, mydisease, ols, seer.

## Usage

```text
biomcp search disease [OPTIONS] [text]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--limit` | integer | no | min 1, max 50 |  | Maximum source rows on one page. Value form: integer. Example: --limit 1. Command sources: monarch, mydisease, ols, seer; source restrictions in the field description apply. |
| `--offset` | integer | no | min 0 |  | Source row offset. Value form: integer. Example: --offset 1. Command sources: monarch, mydisease, ols, seer; source restrictions in the field description apply. |
| `--ontology` | string | no |  | `mondo`, `doid`, `ncit` | Exactly one ontology, default MONDO; help search disease Exactly one ontology, default MONDO. Listed choices accept any letter case; replies use their listed spelling. Value form: string. Example: --ontology mondo. Command sources: monarch, mydisease, ols, seer; source restrictions in the field description apply. |
| `--source` | string | no |  | `ols`, `mydisease`, `monarch`, `seer` | One explicit source, default OLS; help search disease One explicit source, default OLS. Listed choices accept any letter case; replies use their listed spelling. Value form: string. Example: --source seer. Command sources: monarch, mydisease, ols, seer; source restrictions in the field description apply. |
| `text` (position 1) | string | no |  |  | Exact source search text; optional site-name filter for SEER. |

## Sources

`--source` picks the source to ask: [ols](/sources/ols/), [mydisease](/sources/mydisease/), [monarch](/sources/monarch-initiative/), [seer](/sources/seer/).

## Examples

The program's help gives these examples.

```sh
biomcp search disease --source seer --limit 1
```

```sh
biomcp search disease --source seer --limit 1 --offset 1
```

```sh
biomcp search disease 'breast cancer' --source ols --ontology mondo --limit 1
```

## Reply

The reply is one JSON object. Its schema is one of [`biomcp.disease-page/5`](/reference/schemas/biomcp-disease-page-5/), [`biomcp.disease-sites/6`](/reference/schemas/biomcp-disease-sites-6/), whose page lists its fields.

## Next

The [disease guide](/user-guide/disease/) shows every disease command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
