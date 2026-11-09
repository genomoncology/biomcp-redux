---
title: biomcp get pgx
description: "The BioMCP 1.0.0 command biomcp get pgx: Reads pharmacogenomic
  records from CPIC or ClinPGx, such as guidelines, recommendations or
  annotations."
goal: Look up the arguments, sections and examples of biomcp get pgx.
reader: A BioMCP user or agent checking one command
sidebar:
  label: get pgx
kind: reference
verified: 2026-11-09
---

`biomcp get pgx` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It reads pharmacogenomic records from CPIC or ClinPGx, such as guidelines, recommendations or annotations. A [section word](/concepts/progressive-disclosure/) after the identifier adds one more part of the record to the reply. This page lists its arguments, sections, examples and reply.

The program's help describes it:

> Get pharmacogenomic evidence and source sections. Sources: clinpgx, cpic.

## Usage

```text
biomcp get pgx [OPTIONS] [section]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--annotation-id` | string | no |  |  | Checked ClinPGx accession, never a shared PaId. Value form: string. Example: --annotation-id &lt;annotation-id&gt; (syntax; replace the placeholder with your retained value). Command sources: clinpgx, cpic; source restrictions in the field description apply. |
| `--diplotype` | string | no |  |  | Unchanged declared expression required for the diplotype section only. Value form: string. Example: --diplotype '*1/*2A'. Command sources: clinpgx, cpic; source restrictions in the field description apply. |
| `--drug-id` | string | no |  |  | Exact published CPIC drug key; recommendations only. Value form: string. Example: --drug-id &lt;drug-id&gt; (syntax; replace the placeholder with your retained value). Command sources: clinpgx, cpic; source restrictions in the field description apply. |
| `--gene` | string | no |  |  | Explicit source gene symbol. Value form: string. Example: --gene DPYD. Command sources: clinpgx, cpic; source restrictions in the field description apply. |
| `--kind` | string | no |  | `clinical`, `guideline`, `label` | One ClinPGx collection; clinical is the default only on; help get pgx One ClinPGx collection; clinical is the default only on annotations. Listed choices accept any letter case; replies use their listed spelling. Value form: string. Example: --kind clinical. Command sources: clinpgx, cpic; source restrictions in the field description apply. |
| `--section-limit` | integer | no | min 1, max 50 |  | Rows per page: three clinical, five elsewhere. Value form: integer. Example: --section-limit 1. Command sources: clinpgx, cpic; source restrictions in the field description apply. |
| `--section-offset` | integer | no | min 0 |  | Offset within the selected section. Value form: integer. Example: --section-offset 0. Command sources: clinpgx, cpic; source restrictions in the field description apply. |
| `--snapshot` | string | no |  |  | Retained collection for local continuation. Value form: string. Example: --snapshot &lt;snapshot&gt; (syntax; replace the placeholder with your retained value). Command sources: clinpgx, cpic; source restrictions in the field description apply. |
| `--source` | string | yes |  | `cpic`, `clinpgx` | Required source, checked against the section; help get pgx Required source, checked against the section. Listed choices accept any letter case; replies use their listed spelling. Value form: string. Example: --source cpic. Command sources: clinpgx, cpic; source restrictions in the field description apply. |
| `section` (position 1) | string | no |  | `recommendations`, `guidelines`, `diplotype`, `annotations` | One named section; omitted means the CPIC gene card; help get pgx Listed choices accept any letter case; replies use their listed spelling. |

## Sections

One named section; omitted means the CPIC gene card.

| Section | Description |
| --- | --- |
| `recommendations` | Complete source recommendation contexts. |
| `guidelines` | Source guideline references. |
| `diplotype` | Declared source diplotype lookup. |
| `annotations` | One ClinPGx annotation kind. |

## Sources

`--source` picks the source to ask: [cpic](/sources/cpic/), [clinpgx](/sources/pharmgkb/).

## Examples

The program's help gives these examples.

```sh
biomcp get pgx --gene DPYD --source cpic
```

```sh
biomcp get pgx --gene DPYD --source cpic diplotype --diplotype '*1/*2A'
```

## Reply

The reply is one JSON object. Its schema is one of [`biomcp.pgx-annotation/5`](/reference/schemas/biomcp-pgx-annotation-5/), [`biomcp.pgx-annotations/5`](/reference/schemas/biomcp-pgx-annotations-5/), [`biomcp.pgx-card/4`](/reference/schemas/biomcp-pgx-card-4/), [`biomcp.pgx-diplotype/4`](/reference/schemas/biomcp-pgx-diplotype-4/), [`biomcp.pgx-guidelines/4`](/reference/schemas/biomcp-pgx-guidelines-4/), [`biomcp.pgx-recommendations/5`](/reference/schemas/biomcp-pgx-recommendations-5/), whose page lists its fields.

## Next

The [pgx guide](/user-guide/pgx/) shows every pgx command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
