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
verified: 2026-10-27
---

`biomcp get pgx` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It reads pharmacogenomic records from CPIC or ClinPGx, such as guidelines, recommendations or annotations. A [section word](/concepts/progressive-disclosure/) after the identifier adds one more part of the record to the reply. This page lists its arguments, sections, examples and reply.

The program's help describes it:

> PGx card or section. No phenotype, phase or dose is calculated.

## Usage

```text
biomcp get pgx [OPTIONS] [section]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--annotation-id` | string | no |  |  | Checked ClinPGx accession, never a shared PaId. |
| `--diplotype` | value | no |  |  | Unchanged declared expression required for the diplotype section only. |
| `--drug-id` | value | no |  |  | Exact published CPIC drug key; recommendations only. |
| `--gene` | string | no |  |  | Explicit source gene symbol. |
| `--kind` | value | no |  |  | One ClinPGx collection; clinical is the default only on annotations. |
| `--section-limit` | integer | no | min 1, max 50 |  | Rows per page: three clinical, five elsewhere. |
| `--section-offset` | integer | no | min 0 |  | Offset within the selected section. |
| `--snapshot` | value | no |  |  | Retained collection for local continuation. |
| `--source` | string | yes |  | `cpic`, `clinpgx` | Required source, checked against the section. |
| `section` (position 1) | value | no |  |  | One named section; omitted means the CPIC gene card. |

## Sections

One named section; omitted means the CPIC gene card.

| Section | Description |
| --- | --- |
| `recommendations` | Complete native recommendation contexts. |
| `guidelines` | Native guideline references. |
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

The reply is one JSON object. Its schema is one of [`biomcp.pgx_card/1`](/reference/schemas/biomcp-pgx-card-1/), [`biomcp.pgx_diplotype/1`](/reference/schemas/biomcp-pgx-diplotype-1/), whose page shows a real reply and its fields.

## Next

The [pgx guide](/user-guide/pgx/) shows every pgx command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
