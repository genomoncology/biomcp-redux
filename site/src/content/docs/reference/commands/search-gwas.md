---
title: biomcp search gwas
description: "Reference for the BioMCP 1.0.0 command biomcp search gwas: Source
  associations in default p-value-ascending order."
goal: Look up the arguments, sections and examples of biomcp search gwas.
reader: A BioMCP user or agent checking one command
sidebar:
  label: search gwas
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 command `biomcp search gwas`.

Source associations in default p-value-ascending order. Reports no personal risk, causal variant or gene, or clinical interpretation of a statistic.

## Usage

```text
biomcp search gwas [OPTIONS] [query]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--extended-geneset` | boolean | no |  |  | Source extended gene set; requires gene. |
| `--gene` | string | no |  |  | Exact native mapped gene; legacy positional query aliases this field. |
| `--include-child-traits` | boolean | no |  |  | Source child-trait expansion; requires trait_id. |
| `--limit` | integer | no | min 1, max 50 |  | Maximum association rows. |
| `--offset` | integer | no | min 0 |  | Native association offset. |
| `--p-value` | string | no |  |  | Inclusive positive probability in scientific notation. Requires the default p-value-ascending order; no sort option is available. The qualifying total stays unknown until the ordered walk reaches its end. |
| `--pool` | string | no |  |  | Retained threshold walk identity; requires a threshold. |
| `--pubmed` | string | no |  |  | Exact source publication identity. |
| `--study` | string | no |  |  | Exact source study accession. |
| `--trait` | string | no |  |  | Literal native efo_trait label; never searches reported_trait prose. |
| `--trait-id` | string | no |  |  | Exact native ontology identity. |
| `--variant` | string | no |  |  | Exact source rsID. |
| `query` (position 1) | string | no |  |  | Exact native mapped gene; legacy positional query aliases this field. |

## Examples

```sh
biomcp search gwas --pubmed 17529967
biomcp search gwas --variant rs2981582
```

## Reply schema

The help examples of this command return [`biomcp.gwas_page/3`](/reference/schemas/biomcp-gwas-page-3/). Print the full JSON schema with `biomcp schema biomcp.gwas_page/3`.

## Exit codes

See [exit codes](/reference/error-codes/). Options that apply to every command are in [global options](/reference/global-options/).

## Next

The [gwas entity](/user-guide/gwas/) page lists every gwas command and links to a worked guide.

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
