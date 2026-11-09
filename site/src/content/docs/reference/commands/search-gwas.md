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
verified: 2026-11-09
---

`biomcp search gwas` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It finds GWAS Catalog associations by trait, gene, variant, study or PubMed ID, smallest p-value first. This page lists its arguments, examples and reply.

The program's help describes it:

> Search genetic associations in GWAS Catalog. Sources: gwas.

## Usage

```text
biomcp search gwas [OPTIONS] [query]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--extended-geneset` | boolean | no |  |  | Source extended gene set; requires gene. Value form: boolean. Example: --extended-geneset. Command sources: gwas; source restrictions in the field description apply. |
| `--gene` | string | no |  |  | Exact source mapped gene; legacy positional query aliases this field. Value form: string. Example: --gene FGFR2. Command sources: gwas; source restrictions in the field description apply. |
| `--include-child-traits` | boolean | no |  |  | Source child-trait expansion; requires trait_id. Value form: boolean. Example: --include-child-traits. Command sources: gwas; source restrictions in the field description apply. |
| `--limit` | integer | no | min 1, max 50 |  | Maximum association rows. Value form: integer. Example: --limit 3. Command sources: gwas; source restrictions in the field description apply. |
| `--offset` | integer | no | min 0 |  | Source association offset. Value form: integer. Example: --offset 3. Command sources: gwas; source restrictions in the field description apply. |
| `--p-value` | string | no |  |  | Inclusive positive probability in scientific notation. Requires the default p-value-ascending order; no sort option is available. The qualifying total stays unknown until the ordered walk reaches its end. Value form: string. Example: --p-value 4e-254. Command sources: gwas; source restrictions in the field description apply. |
| `--pool` | string | no |  |  | Retained threshold walk identity; requires a threshold. Value form: string. Example: --pool &lt;pool&gt; (syntax; replace the placeholder with your retained value). Command sources: gwas; source restrictions in the field description apply. |
| `--pubmed` | string | no |  |  | Exact source publication identity. Value form: string. Example: --pubmed 17529967. Command sources: gwas; source restrictions in the field description apply. |
| `--studies` | boolean | no |  |  | Return study records for the selected publication instead of associations. Value form: boolean. Example: --studies. Command sources: gwas; source restrictions in the field description apply. |
| `--study` | string | no |  |  | Exact source study accession. Value form: string. Example: --study GCST000035. Command sources: gwas; source restrictions in the field description apply. |
| `--trait` | string | no |  |  | Exact source efo_trait label; never searches reported_trait prose. Value form: string. Example: --trait 'breast carcinoma'. Command sources: gwas; source restrictions in the field description apply. |
| `--trait-id` | string | no |  |  | Exact source ontology identity. Value form: string. Example: --trait-id MONDO_0004989. Command sources: gwas; source restrictions in the field description apply. |
| `--variant` | string | no |  |  | Exact source rsID. Value form: string. Example: --variant rs2981582. Command sources: gwas; source restrictions in the field description apply. |
| `query` (position 1) | string | no |  |  | Exact source mapped gene; legacy positional query aliases this field. |

## Examples

The program's help gives these examples.

```sh
biomcp search gwas --pubmed 17529967 --limit 3
```

```sh
biomcp search gwas --variant rs2981582
```

```sh
biomcp search gwas --pubmed 17529967 --p-value 4e-254
```

```sh
biomcp search gwas --trait 'breast carcinoma'
```

```sh
biomcp search gwas --trait-id MONDO_0004989
```

```sh
biomcp search gwas --trait-id MONDO_0004989 --include-child-traits
```

```sh
biomcp search gwas --gene FGFR2
```

```sh
biomcp search gwas --gene FGFR2 --extended-geneset
```

```sh
biomcp search gwas --study GCST000035
```

```sh
biomcp search gwas --pubmed 17529967 --offset 3
```

```sh
biomcp search gwas --pubmed 17554300 --studies --limit 50
```

## Reply

The reply is one JSON object. Its schema is one of [`biomcp.gwas-page/8`](/reference/schemas/biomcp-gwas-page-8/), [`biomcp.gwas-study-page/1`](/reference/schemas/biomcp-gwas-study-page-1/), whose page lists its fields.

## Next

The [gwas guide](/user-guide/gwas/) shows every gwas command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
