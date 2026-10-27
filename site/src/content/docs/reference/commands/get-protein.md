---
title: biomcp get protein
description: "Reference for the BioMCP 1.0.0 command biomcp get protein: Read
  source protein cards; a symbol returns candidates without choosing a protein."
goal: Look up the arguments, sections and examples of biomcp get protein.
reader: A BioMCP user or agent checking one command
sidebar:
  label: get protein
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 command `biomcp get protein`.

Read source protein cards; a symbol returns candidates without choosing a protein.
Sections disclose native facts with independent row bounds; function keeps all paragraphs.
No variant effect, structure, interaction or clinical meaning is inferred. No bioinformatics tool runs.

## Usage

```text
biomcp get protein [OPTIONS] <input...>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--section` | array | no |  | `features`, `domains`, `isoforms`, `complexes`, `structures`, `function`, `sequence`, `all` | Select native sections or all bounded first pages; function keeps one whole text record. |
| `--section-limit` | integer | no | min 1, max 25 |  | First-page row limit; function keeps one whole native text record and does not page its paragraphs. |
| `input` (position 1) | array | yes | min 1, max 20 |  | Source accessions or gene symbols, preserving caller order. |

## Sections

Select native sections or all bounded first pages; function keeps one whole text record.

| Section | Description |
| --- | --- |
| `features` | Native UniProt features. |
| `domains` | Native InterPro entry matches. |
| `isoforms` | Native alternative products. |
| `complexes` | Exact participant-admitted complex headers. |
| `structures` | Native structure cross-reference links. |
| `function` | One whole native function text record with all source paragraphs. |
| `sequence` | Direct exact-identity sequence and audit metadata. |
| `all` | Bounded first page of every named section. |

## Examples

```sh
biomcp get protein P00533 Q9UIF7 P38398-2
biomcp get protein EGFR
```

## Reply schema

The help examples of this command return [`biomcp.protein_card/3`](/reference/schemas/biomcp-protein-card-3/). Print the full JSON schema with `biomcp schema biomcp.protein_card/3`.

## Exit codes

See [exit codes](/reference/error-codes/). Options that apply to every command are in [global options](/reference/global-options/).

## Next

The [protein entity](/user-guide/protein/) page lists every protein command and links to a worked guide.

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
