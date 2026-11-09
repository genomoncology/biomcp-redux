---
title: biomcp get phenotype
description: "The BioMCP 1.0.0 command biomcp get phenotype: Reads one HP term
  from HPO or Monarch, and optionally its synonyms, links, related diseases or
  genes."
goal: Look up the arguments, sections and examples of biomcp get phenotype.
reader: A BioMCP user or agent checking one command
sidebar:
  label: get phenotype
kind: reference
verified: 2026-11-09
---

`biomcp get phenotype` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It reads one HP term from HPO or Monarch, and optionally its synonyms, links, related diseases or genes. A [section word](/concepts/progressive-disclosure/) after the identifier adds one more part of the record to the reply. This page lists its arguments, sections, examples and reply.

The program's help describes it:

> Get human phenotype details and relationships. Sources: hpo, monarch.

## Usage

```text
biomcp get phenotype [OPTIONS] <id> [section]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--section-limit` | integer | no | min 1, max 50 |  | Default five text/link rows or one complete association row. Value form: integer. Example: --section-limit 1. Command sources: hpo, monarch; source restrictions in the field description apply. |
| `--section-offset` | integer | no | min 0 |  | Source section row offset. Value form: integer. Example: --section-offset 0. Command sources: hpo, monarch; source restrictions in the field description apply. |
| `--snapshot` | string | no |  |  | Retained exact source reply for local text and link pages; unsupported for diseases and genes. Value form: string. Example: --snapshot &lt;snapshot&gt; (syntax; replace the placeholder with your retained value). Command sources: hpo, monarch; source restrictions in the field description apply. |
| `--source` | string | no |  | `hpo`, `monarch` | HPO or Monarch for cards, synonyms and xrefs; Monarch only; help get phenotype HPO or Monarch for cards, synonyms and xrefs; Monarch only for hierarchy, diseases and genes. Defaults to HPO for cards, synonyms and xrefs, otherwise Monarch. Listed choices accept any letter case; replies use their listed spelling. Value form: string. Example: --source monarch. Command sources: hpo, monarch; source restrictions in the field description apply. |
| `id` (position 1) | string | yes |  |  | Exact HP identifier. |
| `section` (position 2) | string | no |  | `synonyms`, `xrefs`, `hierarchy`, `diseases`, `genes` | Absent means the source identity card; help get phenotype Listed choices accept any letter case; replies use their listed spelling. |

## Sections

Absent means the source identity card.

| Section | Description |
| --- | --- |
| `synonyms` | Source synonyms. |
| `xrefs` | Source cross references. |
| `hierarchy` | Source direct parent and child links. |
| `diseases` | Source disease association rows. |
| `genes` | Source human gene association rows. |

## Sources

`--source` picks the source to ask: [hpo](/sources/hpo/), [monarch](/sources/monarch-initiative/).

## Examples

The program's help gives these examples.

```sh
biomcp get phenotype HP:0000001
```

```sh
biomcp get phenotype HP:0000001 --source monarch
```

## Reply

The reply is one JSON object. Its schema is one of [`biomcp.phenotype-card/5`](/reference/schemas/biomcp-phenotype-card-5/), [`biomcp.phenotype-diseases/7`](/reference/schemas/biomcp-phenotype-diseases-7/), [`biomcp.phenotype-genes/7`](/reference/schemas/biomcp-phenotype-genes-7/), [`biomcp.phenotype-hierarchy/5`](/reference/schemas/biomcp-phenotype-hierarchy-5/), [`biomcp.phenotype-synonyms/4`](/reference/schemas/biomcp-phenotype-synonyms-4/), [`biomcp.phenotype-xrefs/4`](/reference/schemas/biomcp-phenotype-xrefs-4/), whose page lists its fields.

## Next

The [phenotype guide](/user-guide/phenotype/) shows every phenotype command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
