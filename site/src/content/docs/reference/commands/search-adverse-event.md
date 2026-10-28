---
title: biomcp search adverse-event
description: "The BioMCP 1.0.0 command biomcp search adverse-event: Finds
  adverse event reports in FAERS, VAERS or MAUDE, five at a time."
goal: Look up the arguments, sections and examples of biomcp search adverse-event.
reader: A BioMCP user or agent checking one command
sidebar:
  label: search adverse-event
kind: reference
verified: 2026-10-28
---

`biomcp search adverse-event` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It finds adverse event reports in FAERS, VAERS or MAUDE, five at a time. A [section word](/concepts/progressive-disclosure/) after the identifier adds one more part of the record to the reply. This page lists its arguments, sections, examples and reply.

The program's help describes it:

> Search five report headers by default. VAERS accepts MMR, COVID19, COVID-19 vaccine or one exact qualified native influenza code, matched without case; use influenza to list codes and labels. Tables remain separate and are never summed. One report may list several vaccines. VAERS returns five reported symptom rows by default; seriousness and age select independent tables. Counts are reports, not people. Reports do not show that a vaccine caused an event and cannot measure risk or compare vaccine safety.

## Usage

```text
biomcp search adverse-event [OPTIONS] <text> [section]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--limit` | integer | no | min 1, max 50 |  | Native report headers or VAERS rows per page; default five. |
| `--offset` | integer | no | min 0 |  | Native row offset. VAERS continuation requires its retained snapshot. |
| `--reaction` | value | no |  |  | FAERS MedDRA term phrase match, not exact equality. |
| `--received-from` | value | no |  |  | Inclusive first FDA receipt lower bound, YYYY-MM-DD. |
| `--received-to` | value | no |  |  | Inclusive first FDA receipt upper bound, YYYY-MM-DD. |
| `--serious` | value | no |  |  | FAERS report flag: death or hospitalization; never a reaction outcome. |
| `--snapshot` | value | no |  |  | Retained VAERS table for provider-free continuation. |
| `--source` | string | no |  | `faers`, `maude`, `vaers` | FAERS by default; VAERS accepts only exact supported vaccine names. |
| `text` (position 1) | string | yes |  |  | Literal drug text, MAUDE device phrase or exact supported VAERS vaccine. |
| `section` (position 2) | value | no |  |  | Independent VAERS table; omitted means reported symptoms. |

## Sections

Independent VAERS table; omitted means reported symptoms.

| Section | Description |
| --- | --- |
| `symptoms` | Native reported symptom rows. |
| `seriousness` | Reports classified by native seriousness. |
| `age` | Native age groups. |

## Sources

`--source` picks the source to ask: [faers](/sources/faers/), [maude](/sources/maude/), [vaers](/sources/vaers/).

## Examples

The program's help gives these examples.

```sh
biomcp search adverse-event pembrolizumab
```

```sh
biomcp search adverse-event pembrolizumab --received-from 2024-01-01 \
  --received-to 2024-12-31
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.adverse_event_page/2`](/reference/schemas/biomcp-adverse-event-page-2/), whose page shows a real reply and its fields.

## Next

The [adverse-event guide](/user-guide/adverse-event/) shows every adverse-event command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
