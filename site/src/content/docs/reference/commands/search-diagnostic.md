---
title: biomcp search diagnostic
description: "The BioMCP 1.0.0 command biomcp search diagnostic: Finds
  diagnostic tests in one source, such as GTR, the FDA companion diagnostic list
  or WHO's prequalified tests."
goal: Look up the arguments, sections and examples of biomcp search diagnostic.
reader: A BioMCP user or agent checking one command
sidebar:
  label: search diagnostic
kind: reference
verified: 2026-10-27
---

`biomcp search diagnostic` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It finds diagnostic tests in one source, such as GTR, the FDA companion diagnostic list or WHO's prequalified tests. This page lists its arguments, examples and reply.

The program's help describes it:

> Search one diagnostic source. A listing is not authorization; a companion row is not a treatment recommendation.

## Usage

```text
biomcp search diagnostic [OPTIONS]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--biomarker` | value | no |  |  | Exact table-0 biomarker cell; unsupported in the group table. |
| `--bundle-revision` | value | no |  |  | Immutable stored [bundle](/local-data/) revision retained by a continuation. |
| `--condition-id` | value | no |  |  | Exact MedGen:CUI identifier only. |
| `--device-id` | value | no |  |  | Exact PMA application, application/supplement or native K identity. |
| `--drug-application` | value | no |  |  | Cell-owned typed NDA/BLA application. |
| `--gene` | value | no |  |  | Exact source gene_symbol field, without alias expansion. |
| `--gene-id` | value | no |  |  | Exact source GeneID, exclusive with gene symbol. |
| `--limit` | integer | no | min 1, max 50 |  | Five results by default; at most fifty source rows or product headers. |
| `--manufacturer` | value | no |  |  | Literal whole-word phrase in WHO's Manufacturer name cell. |
| `--marker` | value | no |  |  | Literal whole-word phrase in WHO's Pathogen/Disease/Marker cell. |
| `--offset` | integer | no | min 0 |  | Zero-based source result offset. |
| `--source` | string | yes |  | `gtr`, `fda-cdx`, `who-ivd` | Explicit native GTR, WHO IVD or FDA companion discovery. |
| `--type` | value | no |  |  | Exact native test_type value. |

## Sources

`--source` picks the source to ask: [gtr](/sources/gtr/), [fda-cdx](/sources/fda-cdx/), [who-ivd](/sources/who-ivd/).

## Examples

The program's help gives these examples.

```sh
biomcp search diagnostic --source gtr --gene BRCA1
```

```sh
biomcp search diagnostic --source gtr --condition-id MedGen:C2676676
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.diagnostic_page/2`](/reference/schemas/biomcp-diagnostic-page-2/), whose page shows a real reply and its fields.

## Next

[Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes. The [diagnostic guide](/user-guide/diagnostic/) shows every diagnostic command at work.
