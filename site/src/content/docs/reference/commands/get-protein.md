---
title: biomcp get protein
description: "The BioMCP 1.0.0 command biomcp get protein: Reads UniProt protein
  records by accession, and optionally sections such as domains or sequence."
goal: Look up the arguments, sections and examples of biomcp get protein.
reader: A BioMCP user or agent checking one command
sidebar:
  label: get protein
kind: reference
verified: 2026-11-09
---

`biomcp get protein` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It reads UniProt protein records by accession, and optionally sections such as domains or sequence. A [section word](/concepts/progressive-disclosure/) after the identifier adds one more part of the record to the reply. This page lists its arguments, sections, examples and reply.

The program's help describes it:

> Get protein details and source annotations. Sources: complex-portal, hgnc, interpro, uniprot.

## Usage

```text
biomcp get protein [OPTIONS] <input...>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--section` | array | no |  | `features`, `domains`, `isoforms`, `complexes`, `structures`, `function`, `sequence`, `all` | Select source sections or all bounded first pages; function; help get protein Listed choices accept any letter case; replies use their listed spelling. Value form: repeatable values (repeat the flag). Example: --section function. Command sources: complex-portal, hgnc, interpro, uniprot; source restrictions in the field description apply. |
| `--section-limit` | integer | no | min 1, max 25 |  | First-page row limit; function keeps one whole source text record and does not page its paragraphs. Value form: integer. Example: --section-limit 2. Command sources: complex-portal, hgnc, interpro, uniprot; source restrictions in the field description apply. |
| `input` (position 1) | array | yes | min 1, max 20 |  | Source accessions or gene symbols, preserving caller order. |

## Sections

Select source sections or all bounded first pages; function keeps one whole text record.

| Section | Description |
| --- | --- |
| `features` | Source UniProt features. |
| `domains` | Source InterPro entry matches. |
| `isoforms` | Source alternative products. |
| `complexes` | Exact participant-admitted complex headers. |
| `structures` | Source structure cross-reference links. |
| `function` | One whole source function text record with all source paragraphs. |
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

```sh
biomcp get protein P00533 isoforms --section-limit 2
```

```sh
biomcp get protein P00533 --section function --section-limit 2
```

## Reply

The reply is one JSON object. Its schema is one of [`biomcp.protein-card/10`](/reference/schemas/biomcp-protein-card-10/), [`biomcp.protein-function/4`](/reference/schemas/biomcp-protein-function-4/), whose page lists its fields.

## Next

The [protein guide](/user-guide/protein/) shows every protein command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
