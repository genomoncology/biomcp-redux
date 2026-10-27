---
title: biomcp get drug
description: "The BioMCP 1.0.0 command biomcp get drug: Reads one drug record by
  its ID in one source, and optionally a section such as its label or
  approvals."
goal: Look up the arguments, sections and examples of biomcp get drug.
reader: A BioMCP user or agent checking one command
sidebar:
  label: get drug
kind: reference
verified: 2026-10-27
---

`biomcp get drug` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It reads one drug record by its ID in one source, and optionally a section such as its label or approvals. A [section word](/concepts/progressive-disclosure/) after the identifier adds one more part of the record to the reply. This page lists its arguments, sections, examples and reply.

The program's help describes it:

> Exact native ID retrieval with an explicitly selected source.

## Usage

```text
biomcp get drug [OPTIONS] <input> [section...]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--civic-cursor` | value | no |  |  | Native continuation cursor for that exact drug, profile and claim collection. |
| `--civic-kind` | value | no |  |  | Independent native evidence or assertion collection within the selected profile. |
| `--civic-profile` | value | no | min 1, max 2147483647 |  | Selected native CIViC molecular profile for a drug-scoped claim stream. |
| `--label-section` | value | no |  |  | Exact native label heading, required only for the label section. |
| `--product-number` | value | no |  |  | Explicit native application product number, only for approvals. |
| `--section-limit` | value | no | min 1, max 50 |  | Named section page size, refused on an identity-only card. |
| `--section-offset` | value | no | min 0 |  | Named section continuation, refused on an identity-only card. |
| `--source` | string | yes |  | `who-pq`, `mychem`, `chembl`, `opentargets`, `civic`, `openfda-label`, `drugsfda`, `ema`, `ddinter` | Required source; there is no default exact-ID resolver. |
| `input` (position 1) | string | yes |  |  | Exact source-native ID; WHO references retain native spaces. No name or alias resolution. |
| `section` (position 2) | array | no |  | `xrefs`, `targets`, `indications`, `civic`, `label`, `approvals`, `regulatory`, `interactions` | Explicit named disclosures; empty means identity only. |

## Sections

Explicit named disclosures; empty means identity only.

| Section | Description |
| --- | --- |
| `xrefs` | All native identity cross-references and product links, mechanically paged. |
| `targets` | Native target and mechanism relations. |
| `indications` | Native indication wording. |
| `civic` | Native CIViC therapy evidence and profile context. |
| `label` | One exact native label heading. |
| `approvals` | Native application and product history rows. |
| `regulatory` | Source-scoped regulator events. |
| `interactions` | Native DDInter partner rows and levels; five rows per default page, with explicit section limits preserved. |

## Sources

`--source` picks the source to ask: [who-pq](/sources/who-prequalification/), [mychem](/sources/mychem/), [chembl](/sources/chembl/), [opentargets](/sources/opentargets/), [civic](/sources/civic/), [openfda-label](/sources/openfda/), [drugsfda](/sources/drugsfda/), [ema](/sources/ema/), [ddinter](/sources/ddinter/).

## Examples

The program's help gives these examples.

```sh
biomcp get drug EMEA/H/C/000406 --source ema
```

```sh
biomcp get drug EMEA/H/C/000406 regulatory --source ema
```

## Reply

The reply is one JSON object. Its schema is one of [`biomcp.drug_card/2`](/reference/schemas/biomcp-drug-card-2/), [`biomcp.drug_regulatory/2`](/reference/schemas/biomcp-drug-regulatory-2/), whose page shows a real reply and its fields.

## Next

The [drug guide](/user-guide/drug/) shows every drug command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
