---
title: biomcp search gwas
description: "The BioMCP 1.0.0 command biomcp search gwas: Finds GWAS Catalog
  associations by trait, gene, variant, study or PubMed ID, smallest p-value
  first."
goal: Look up the arguments, sections and examples of biomcp search gwas.
reader: A BioMCP user or agent checking one command
sidebar:
  label: search gwas
kind: reference
verified: 2026-10-27
---

`biomcp search gwas` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It finds GWAS Catalog associations by trait, gene, variant, study or PubMed ID, smallest p-value first. This page lists its arguments, examples and reply.

The program's help describes it:

> Source associations in default p-value-ascending order. Reports no personal risk, causal variant or gene, or clinical interpretation of a statistic.

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

The program's help gives these examples.

```sh
biomcp search gwas --pubmed 17529967
```

```sh
biomcp search gwas --variant rs2981582
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.gwas_page/3`](/reference/schemas/biomcp-gwas-page-3/), whose page shows a real reply and its fields.

## Next

The [gwas guide](/user-guide/gwas/) shows every gwas command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
