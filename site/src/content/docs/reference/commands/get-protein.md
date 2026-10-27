---
title: biomcp get protein
description: "The BioMCP 1.0.0 command biomcp get protein: Reads UniProt protein
  records by accession, and optionally sections such as domains or sequence."
goal: Look up the arguments, sections and examples of biomcp get protein.
reader: A BioMCP user or agent checking one command
sidebar:
  label: get protein
kind: reference
verified: 2026-10-27
---

`biomcp get protein` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It reads UniProt protein records by accession, and optionally sections such as domains or sequence. A [section word](/concepts/progressive-disclosure/) after the identifier adds one more part of the record to the reply. This page lists its arguments, sections, examples and reply.

The program's help describes it:

> Read source protein cards; a symbol returns candidates without choosing a protein.
> Sections disclose native facts with independent row bounds; function keeps all paragraphs.
> No variant effect, structure, interaction or clinical meaning is inferred. No bioinformatics tool runs.

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

The program's help gives these examples.

```sh
biomcp get protein P00533 Q9UIF7 P38398-2
```

```sh
biomcp get protein EGFR
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.protein_card/3`](/reference/schemas/biomcp-protein-card-3/), whose page shows a real reply and its fields.

## Next

[Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes. The [protein guide](/user-guide/protein/) shows every protein command at work.
