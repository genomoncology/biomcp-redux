---
title: biomcp search adverse-event
description: "The BioMCP 1.0.0 command biomcp search adverse-event: Finds
  adverse event reports in FAERS, VAERS or MAUDE, five at a time."
goal: Look up the arguments, sections and examples of biomcp search adverse-event.
reader: A BioMCP user or agent checking one command
sidebar:
  label: search adverse-event
kind: reference
verified: 2026-11-09
---

`biomcp search adverse-event` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It finds adverse event reports in FAERS, VAERS or MAUDE, five at a time. A [section word](/concepts/progressive-disclosure/) after the identifier adds one more part of the record to the reply. This page lists its arguments, sections, examples and reply.

The program's help describes it:

> Search drug, device and vaccine adverse events. Sources: faers, maude, vaers.

## Usage

```text
biomcp search adverse-event [OPTIONS] [text] [section]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--aggregate` | string | no |  | `reactions` | FAERS reaction counts; help search adverse-event FAERS reaction counts. Use reactions for source count buckets across matching reports. Counts are not rates or causal associations; a report may contribute to several terms. Requires MedDRA and offset zero. Listed choices accept any letter case; replies use their listed spelling. Value form: string. Example: --aggregate reactions. Command sources: faers, maude, vaers; source restrictions in the field description apply. |
| `--drug` | array | no |  |  | FAERS drug names; OR. FAERS exact drug name; repeat --drug for OR. A standalone OR joining names inside one --drug value is refused. Other filters AND with the names. BioMCP caps the escaped source query, including filters, at FAERS_QUERY_MAX_BYTES (4096 bytes). Value form: repeatable values (repeat the flag). Example: --drug "synthetic\\B". Command sources: faers, maude, vaers; source restrictions in the field description apply. |
| `--limit` | integer | no | min 1, max 50 |  | Source report headers, counted FAERS reaction terms or VAERS rows; default five. Value form: integer. Example: --limit 1. Command sources: faers, maude, vaers; source restrictions in the field description apply. |
| `--manufacturer` | string | no |  |  | MAUDE manufacturer. MAUDE manufacturer phrase on device.manufacturer_d_name. A report may list several devices; filters may match different devices. Value form: string. Example: --manufacturer 'Synthetic maker'. Command sources: faers, maude, vaers; source restrictions in the field description apply. |
| `--offset` | integer | no | min 0 |  | Source report row offset; reaction counts require zero. VAERS needs its snapshot. Value form: integer. Example: --offset 0. Command sources: faers, maude, vaers; source restrictions in the field description apply. |
| `--problem` | string | no |  |  | MAUDE product problem. MAUDE phrase on source product_problems. Value form: string. Example: --problem 'Synthetic problem'. Command sources: faers, maude, vaers; source restrictions in the field description apply. |
| `--product-code` | array | no |  |  | MAUDE product codes; OR. MAUDE exact source device.device_report_product_code; repeated codes join with OR. Value form: repeatable values (repeat the flag). Example: --product-code ABC. Command sources: faers, maude, vaers; source restrictions in the field description apply. |
| `--reaction` | string | no |  |  | FAERS MedDRA phrase. FAERS MedDRA term phrase match, not exact equality. Value form: string. Example: --reaction 'synthetic reaction'. Command sources: faers, maude, vaers; source restrictions in the field description apply. |
| `--received-from` | string | no |  |  | First date, YYYY-MM[-DD]. Inclusive FAERS or MAUDE FDA receipt lower bound, YYYY-MM-DD or YYYY-MM; a month starts on its first day. Value form: string. Example: --received-from 2024-01-01. Command sources: faers, maude, vaers; source restrictions in the field description apply. |
| `--received-to` | string | no |  |  | Last date, YYYY-MM[-DD]. Inclusive FAERS or MAUDE FDA receipt upper bound, YYYY-MM-DD or YYYY-MM; a month ends on its last day. Value form: string. Example: --received-to 2024-12-31. Command sources: faers, maude, vaers; source restrictions in the field description apply. |
| `--serious` | string | no |  | `any`, `death`, `hospitalization`, `lifethreatening`, `disabling`, `congenital-anomaly`, `other` | FAERS seriousness flag; help search adverse-event FAERS seriousness flag. FAERS report flag: any uses the source seriousness flag; the specific flags can overlap. Never a reaction outcome. Listed choices accept any letter case; replies use their listed spelling. Value form: string. Example: --serious death. Command sources: faers, maude, vaers; source restrictions in the field description apply. |
| `--snapshot` | string | no |  |  | VAERS snapshot. Retained VAERS table for source-free continuation. Value form: string. Example: --snapshot &lt;snapshot&gt; (syntax; replace the placeholder with your retained value). Command sources: faers, maude, vaers; source restrictions in the field description apply. |
| `--source` | string | no |  | `faers`, `maude`, `vaers` | Source; default FAERS; help search adverse-event Source; default FAERS. FAERS by default; VAERS accepts only exact supported vaccine names. Listed choices accept any letter case; replies use their listed spelling. Value form: string. Example: --source maude. Command sources: faers, maude, vaers; source restrictions in the field description apply. |
| `text` (position 1) | string | no |  |  | Drug, device or vaccine. Exact drug text, MAUDE device phrase or exact supported VAERS vaccine. |
| `section` (position 2) | string | no |  | `symptoms`, `seriousness`, `age` | VAERS table; help search adverse-event Listed choices accept any letter case; replies use their listed spelling. |

## Sections

Independent VAERS table; omitted means reported symptoms.

| Section | Description |
| --- | --- |
| `symptoms` | Source reported symptom rows. |
| `seriousness` | Reports classified by source seriousness. |
| `age` | Source age groups. |

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

```sh
biomcp search adverse-event --source maude --manufacturer 'Synthetic maker' \
  --product-code ABC --product-code XYZ --problem 'Synthetic problem'
```

```sh
biomcp search adverse-event 'imatinib OR nilotinib' --limit 1
```

```sh
biomcp search adverse-event 'synthetic "A"' --drug 'synthetic\B' \
  --reaction 'synthetic reaction' --serious death --received-from 2024-01 \
  --received-to 2024-12 --limit 1
```

```sh
biomcp search adverse-event 'Synthetic medicine' --aggregate reactions \
  --limit 2
```

```sh
biomcp search adverse-event pembrolizumab --source faers --limit 1 --offset 0
```

## Reply

The reply is one JSON object. Its schema is one of [`biomcp.adverse-event-page/6`](/reference/schemas/biomcp-adverse-event-page-6/), [`biomcp.vaers-summary/5`](/reference/schemas/biomcp-vaers-summary-5/), whose page lists its fields.

## Next

The [adverse-event guide](/user-guide/adverse-event/) shows every adverse-event command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
