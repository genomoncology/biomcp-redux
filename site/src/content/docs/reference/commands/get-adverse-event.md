---
title: biomcp get adverse-event
description: "Reference for the BioMCP 1.0.0 command biomcp get adverse-event:
  Exact native report identity and an optional independently paged section."
goal: Look up the arguments, sections and examples of biomcp get adverse-event.
reader: A BioMCP user or agent checking one command
sidebar:
  label: get adverse-event
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 command `biomcp get adverse-event`.

Exact native report identity and an optional independently paged section.

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

`--source` accepts these sources: [faers](/sources/faers/), [maude](/sources/maude/).

## Examples

```sh
biomcp get adverse-event 100
biomcp get adverse-event 100 reactions
```

## Exit codes

See [exit codes](/reference/error-codes/). Options that apply to every command are in [global options](/reference/global-options/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
