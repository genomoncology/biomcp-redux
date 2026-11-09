---
title: biomcp get gwas
description: "The BioMCP 1.0.0 command biomcp get gwas: Reads one GWAS Catalog
  association or study, and optionally its loci, associations or ancestries."
goal: Look up the arguments, sections and examples of biomcp get gwas.
reader: A BioMCP user or agent checking one command
sidebar:
  label: get gwas
kind: reference
verified: 2026-11-09
---

`biomcp get gwas` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It reads one GWAS Catalog association or study, and optionally its loci, associations or ancestries. A [section word](/concepts/progressive-disclosure/) after the identifier adds one more part of the record to the reply. This page lists its arguments, sections, examples and reply.

The program's help describes it:

> Get GWAS Catalog associations or studies. Sources: gwas.

## Usage

```text
biomcp get gwas [OPTIONS] <id> [section]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--section-limit` | integer | no | min 1, max 50 |  | Maximum source relation rows. Value form: integer. Example: --section-limit 1. Command sources: gwas; source restrictions in the field description apply. |
| `--section-offset` | integer | no | min 0 |  | Source relation row offset. Value form: integer. Example: --section-offset 0. Command sources: gwas; source restrictions in the field description apply. |
| `--snapshot` | string | no |  |  | Retained ancestry reply for a local page. Value form: string. Example: --snapshot &lt;snapshot&gt; (syntax; replace the placeholder with your retained value). Command sources: gwas; source restrictions in the field description apply. |
| `id` (position 1) | string | yes |  |  | Qualified association ID or exact GCST study accession. |
| `section` (position 2) | string | no |  | `loci`, `associations`, `ancestries` | Association loci or study associations and ancestries; help get gwas Listed choices accept any letter case; replies use their listed spelling. |

## Sections

Association loci or study associations and ancestries.

| Section | Description |
| --- | --- |
| `loci` | Source association loci. |
| `associations` | Source study association pages. |
| `ancestries` | Source study ancestry rows. |

## Examples

The program's help gives these examples.

```sh
biomcp get gwas association:6114
```

```sh
biomcp get gwas GCST000035
```

## Reply

The reply is one JSON object. Its schema is one of [`biomcp.gwas-ancestries/5`](/reference/schemas/biomcp-gwas-ancestries-5/), [`biomcp.gwas-association/6`](/reference/schemas/biomcp-gwas-association-6/), [`biomcp.gwas-loci/4`](/reference/schemas/biomcp-gwas-loci-4/), [`biomcp.gwas-page/8`](/reference/schemas/biomcp-gwas-page-8/), [`biomcp.gwas-study/6`](/reference/schemas/biomcp-gwas-study-6/), whose page lists its fields.

## Next

The [gwas guide](/user-guide/gwas/) shows every gwas command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
