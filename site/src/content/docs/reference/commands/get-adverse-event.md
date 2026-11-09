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
verified: 2026-11-09
---

`biomcp get adverse-event` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It reads one adverse event report by its ID, and optionally one section such as its drugs or reactions. A [section word](/concepts/progressive-disclosure/) after the identifier adds one more part of the record to the reply. This page lists its arguments, sections, examples and reply.

The program's help describes it:

> Get a drug, device or vaccine adverse event report. Sources: faers, maude.

## Usage

```text
biomcp get adverse-event [OPTIONS] <id> [section]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--drug-index` | integer | no | min 0 |  | Absolute source-array index; required only for identifier sections. Value form: integer. Example: --drug-index 0. Command sources: faers, maude; source restrictions in the field description apply. |
| `--section-limit` | integer | no | min 1, max 50 |  | Whole source rows per section page. Value form: integer. Example: --section-limit 1. Command sources: faers, maude; source restrictions in the field description apply. |
| `--section-offset` | integer | no | min 0 |  | Offset within the selected source section. Value form: integer. Example: --section-offset 0. Command sources: faers, maude; source restrictions in the field description apply. |
| `--snapshot` | string | no |  |  | Checked retained report snapshot for source-free continuation. Value form: string. Example: --snapshot &lt;snapshot&gt; (syntax; replace the placeholder with your retained value). Command sources: faers, maude; source restrictions in the field description apply. |
| `--source` | string | no |  | `faers`, `maude` | FAERS by default; MAUDE is explicit; help get adverse-event FAERS by default; MAUDE is explicit. Listed choices accept any letter case; replies use their listed spelling. Value form: string. Example: --source maude. Command sources: faers, maude; source restrictions in the field description apply. |
| `id` (position 1) | string | yes |  |  | Published report ID with no source prefix. |
| `section` (position 2) | string | no |  | `drugs`, `reactions`, `unii`, `rxcui`, `product-ndc`, `package-ndc`, `spl-id`, `spl-set-id`, `application-number`, `devices`, `narrative` | FAERS owns drugs, reactions and identifiers; MAUDE owns; help get adverse-event Listed choices accept any letter case; replies use their listed spelling. |

## Sections

FAERS owns drugs, reactions and identifiers; MAUDE owns devices and narrative. Omitted means card only.

| Section | Description |
| --- | --- |
| `drugs` | Independent source drug rows. |
| `reactions` | Independent source reaction rows. |
| `unii` | Source UNII strings. |
| `rxcui` | Source RxCUI strings. |
| `product-ndc` | Source product NDC strings. |
| `package-ndc` | Source package NDC strings. |
| `spl-id` | Source SPL IDs. |
| `spl-set-id` | Source SPL set IDs. |
| `application-number` | Source application numbers. |
| `devices` | Source device rows. |
| `narrative` | Complete source narrative rows. |

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

```sh
biomcp get adverse-event 160 --source maude
```

```sh
biomcp get adverse-event 100 unii --drug-index 0
```

```sh
biomcp get adverse-event 100 drugs --source faers --section-limit 1
```

## Reply

The reply is one JSON object. Its schema is one of [`biomcp.adverse-event-devices/4`](/reference/schemas/biomcp-adverse-event-devices-4/), [`biomcp.adverse-event-drugs/4`](/reference/schemas/biomcp-adverse-event-drugs-4/), [`biomcp.adverse-event-faers-card/5`](/reference/schemas/biomcp-adverse-event-faers-card-5/), [`biomcp.adverse-event-identifiers/4`](/reference/schemas/biomcp-adverse-event-identifiers-4/), [`biomcp.adverse-event-maude-card/4`](/reference/schemas/biomcp-adverse-event-maude-card-4/), [`biomcp.adverse-event-narrative/4`](/reference/schemas/biomcp-adverse-event-narrative-4/), [`biomcp.adverse-event-reactions/6`](/reference/schemas/biomcp-adverse-event-reactions-6/), whose page lists its fields.

## Next

The [adverse-event guide](/user-guide/adverse-event/) shows every adverse-event command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
