---
title: biomcp get gwas
description: "Reference for the BioMCP 1.0.0 command biomcp get gwas: Source
  association or study and native context."
goal: Look up the arguments, sections and examples of biomcp get gwas.
reader: A BioMCP user or agent checking one command
sidebar:
  label: get gwas
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 command `biomcp get gwas`.

Source association or study and native context. Reports no personal risk, causal variant or gene, or clinical interpretation of a statistic.

## Usage

```text
biomcp get gwas [OPTIONS] <id> [section]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--section-limit` | integer | no | min 1, max 50 |  | Maximum native relation rows. |
| `--section-offset` | integer | no | min 0 |  | Native relation row offset. |
| `--snapshot` | string | no |  |  | Retained ancestry reply for a local page. |
| `id` (position 1) | string | yes |  |  | Qualified association ID or exact GCST study accession. |
| `section` (position 2) | string | no |  | `loci`, `associations`, `ancestries` | Association loci or study associations and ancestries. |

## Sections

Association loci or study associations and ancestries.

| Section | Description |
| --- | --- |
| `loci` | Native association loci. |
| `associations` | Native study association pages. |
| `ancestries` | Native study ancestry rows. |

## Examples

```sh
biomcp get gwas association:6114
biomcp get gwas GCST000035
```

## Reply schema

The help examples of this command return [`biomcp.gwas_association/1`](/reference/schemas/biomcp-gwas-association-1/), [`biomcp.gwas_study/1`](/reference/schemas/biomcp-gwas-study-1/). Print the full JSON schema with `biomcp schema biomcp.gwas_association/1`.

## Exit codes

See [exit codes](/reference/error-codes/). Options that apply to every command are in [global options](/reference/global-options/).

## Next

The [gwas entity](/user-guide/gwas/) page lists every gwas command and links to a worked guide.

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
