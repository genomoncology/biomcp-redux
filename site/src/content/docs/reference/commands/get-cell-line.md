---
title: biomcp get cell-line
description: "The BioMCP 1.0.0 command biomcp get cell-line: Reads one cell line
  by its ID, and optionally one named section such as its variants."
goal: Look up the arguments, sections and examples of biomcp get cell-line.
reader: A BioMCP user or agent checking one command
sidebar:
  label: get cell-line
kind: reference
verified: 2026-10-27
---

`biomcp get cell-line` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It reads one cell line by its ID, and optionally one named section such as its variants. A [section word](/concepts/progressive-disclosure/) after the identifier adds one more part of the record to the reply. This page lists its arguments, sections, examples and reply.

The program's help describes it:

> One exact identity and an optional bounded named section. An assay count is not an activity or sensitivity count. No treatment response is inferred; no bioinformatics tool runs.

## Usage

```text
biomcp get cell-line [OPTIONS] <id> [section]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--section-limit` | integer | no | min 1, max 25 |  | Explicit row limit for each requested collection; native variation defaults to three. |
| `--section-offset` | integer | no | min 0 |  | Offset within each independently requested collection. |
| `id` (position 1) | string | yes |  |  | One exact primary accession or qualified source ID. |
| `section` (position 2) | string | no |  | `names`, `identity`, `xrefs`, `variants`, `chembl`, `all` | Omit for the default card; all returns bounded first pages without an assay count. |

## Sections

Omit for the default card; all returns bounded first pages without an assay count.

| Section | Description |
| --- | --- |
| `names` | Native display names. |
| `identity` | Independent species, diseases and qualified warnings. |
| `xrefs` | Complete source cross-reference rows. |
| `variants` | Complete native variation rows. |
| `chembl` | Exact accession-linked ChEMBL cell metadata. |
| `all` | Bounded first page of every named section. |

## Examples

The program's help gives these examples.

```sh
biomcp get cell-line CVCL_2119
```

```sh
biomcp get cell-line CVCL_0004
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.cell_line_card/3`](/reference/schemas/biomcp-cell-line-card-3/), whose page shows a real reply and its fields.

## Next

[Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes. The [cell-line guide](/user-guide/cell-line/) shows every cell-line command at work.
