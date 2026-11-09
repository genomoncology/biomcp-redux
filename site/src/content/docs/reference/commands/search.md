---
title: biomcp search
description: "The BioMCP 1.0.0 command biomcp search: Search source records by
  entity and checked filters."
goal: Look up the arguments, sections and examples of biomcp search.
reader: A BioMCP user or agent checking one command
sidebar:
  label: search
kind: reference
verified: 2026-11-09
---

`biomcp search` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It search source records by [entity](/concepts/progressive-disclosure/) and checked filters. This page lists its examples and reply.

The program's help describes it:

> Search source records by entity and checked filters.

## Usage

```text
biomcp search
```

## Examples

The program's help gives this example.

```sh
biomcp search biomarker --source nci --name PD-L1
```

## Reply

The reply is one JSON object. Its schema is one of [`biomcp.adverse-event-page/6`](/reference/schemas/biomcp-adverse-event-page-6/), [`biomcp.article-page/9`](/reference/schemas/biomcp-article-page-9/), [`biomcp.author-page/5`](/reference/schemas/biomcp-author-page-5/), [`biomcp.biomarker-page/4`](/reference/schemas/biomcp-biomarker-page-4/), [`biomcp.cell-line-filtered-page/1`](/reference/schemas/biomcp-cell-line-filtered-page-1/), [`biomcp.cell-line-page/7`](/reference/schemas/biomcp-cell-line-page-7/), [`biomcp.diagnostic-companion/4`](/reference/schemas/biomcp-diagnostic-companion-4/), [`biomcp.diagnostic-page/5`](/reference/schemas/biomcp-diagnostic-page-5/), [`biomcp.disease-page/5`](/reference/schemas/biomcp-disease-page-5/), [`biomcp.disease-sites/6`](/reference/schemas/biomcp-disease-sites-6/), [`biomcp.drug-fda-page/7`](/reference/schemas/biomcp-drug-fda-page-7/), [`biomcp.drug-page/7`](/reference/schemas/biomcp-drug-page-7/), [`biomcp.drug-recall-page/4`](/reference/schemas/biomcp-drug-recall-page-4/), [`biomcp.drug-who-page/5`](/reference/schemas/biomcp-drug-who-page-5/), [`biomcp.gene-page/8`](/reference/schemas/biomcp-gene-page-8/), [`biomcp.gwas-page/8`](/reference/schemas/biomcp-gwas-page-8/), [`biomcp.gwas-study-page/1`](/reference/schemas/biomcp-gwas-study-page-1/), [`biomcp.nci-trial-page/10`](/reference/schemas/biomcp-nci-trial-page-10/), [`biomcp.organization-page/4`](/reference/schemas/biomcp-organization-page-4/), [`biomcp.pathway-page/8`](/reference/schemas/biomcp-pathway-page-8/), [`biomcp.pgx-page/6`](/reference/schemas/biomcp-pgx-page-6/), [`biomcp.phenotype-search/5`](/reference/schemas/biomcp-phenotype-search-5/), [`biomcp.protein-page/5`](/reference/schemas/biomcp-protein-page-5/), [`biomcp.study-page/5`](/reference/schemas/biomcp-study-page-5/), [`biomcp.trial-page/14`](/reference/schemas/biomcp-trial-page-14/), [`biomcp.vaers-summary/5`](/reference/schemas/biomcp-vaers-summary-5/), [`biomcp.variant-page/16`](/reference/schemas/biomcp-variant-page-16/), whose page lists its fields.

## Next

The [commands index](/reference/commands/) lists every command.
