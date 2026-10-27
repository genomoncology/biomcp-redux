---
title: biomcp get drug
description: "Reference for the BioMCP 1.0.0 command biomcp get drug: Exact
  native ID retrieval with an explicitly selected source."
goal: Look up the arguments, sections and examples of biomcp get drug.
reader: A BioMCP user or agent checking one command
sidebar:
  label: get drug
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 command `biomcp get drug`.

Exact native ID retrieval with an explicitly selected source.

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

`--source` accepts these sources: [who-pq](/reference/sources/who-pq/), [mychem](/reference/sources/mychem/), [chembl](/reference/sources/chembl/), [opentargets](/reference/sources/opentargets/), [civic](/reference/sources/civic/), [openfda-label](/reference/sources/openfda-label/), [drugsfda](/reference/sources/drugsfda/), [ema](/reference/sources/ema/), [ddinter](/reference/sources/ddinter/).

## Examples

```sh
biomcp get drug EMEA/H/C/000406 --source ema
biomcp get drug EMEA/H/C/000406 regulatory --source ema
```

## Exit codes

See [exit codes](/reference/exit-codes/). Options that apply to every command are in [global options](/reference/global-options/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
