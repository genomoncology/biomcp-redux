---
title: biomcp get adverse-event
description: "The BioMCP 1.0.0 command biomcp get adverse-event: Reads one
  adverse event report by its ID, and optionally one section such as its drugs
  or reactions."
goal: Look up the arguments, sections and examples of biomcp get adverse-event.
reader: A BioMCP user or agent checking one command
sidebar:
  label: get adverse-event
kind: reference
verified: 2026-10-27
---

`biomcp get adverse-event` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It reads one adverse event report by its ID, and optionally one section such as its drugs or reactions. A [section word](/concepts/progressive-disclosure/) after the identifier adds one more part of the record to the reply. This page lists its arguments, sections and examples.

The program's help describes it:

> Exact native report identity and an optional independently paged section.

## Usage

```text
biomcp get adverse-event [OPTIONS] <id> [section]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--drug-index` | value | no | min 0 |  | Absolute source-array index; required only for identifier sections. |
| `--section-limit` | integer | no | min 1, max 50 |  | Whole source rows per section page. |
| `--section-offset` | integer | no | min 0 |  | Offset within the selected native section. |
| `--snapshot` | value | no |  |  | Checked retained report snapshot for provider-free continuation. |
| `--source` | string | no |  | `faers`, `maude` | FAERS by default; MAUDE is explicit. |
| `id` (position 1) | string | yes |  |  | Published report ID with no source prefix. |
| `section` (position 2) | value | no |  |  | FAERS owns drugs, reactions and identifiers; MAUDE owns devices and narrative. Omitted means card only. |

## Sections

FAERS owns drugs, reactions and identifiers; MAUDE owns devices and narrative. Omitted means card only.

| Section | Description |
| --- | --- |
| `drugs` | Independent native drug rows. |
| `reactions` | Independent native reaction rows. |
| `unii` | Native UNII strings. |
| `rxcui` | Native RxCUI strings. |
| `product-ndc` | Native product NDC strings. |
| `package-ndc` | Native package NDC strings. |
| `spl-id` | Native SPL IDs. |
| `spl-set-id` | Native SPL set IDs. |
| `application-number` | Native application numbers. |
| `devices` | Native device rows. |
| `narrative` | Complete native narrative rows. |

## Sources

`--source` picks the source to ask: [faers](/sources/faers/), [maude](/sources/maude/).

## Examples

The program's help gives these examples.

```sh
biomcp get adverse-event 100
```

```sh
biomcp get adverse-event 100 reactions
```

## Next

The [adverse-event guide](/user-guide/adverse-event/) shows every adverse-event command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
