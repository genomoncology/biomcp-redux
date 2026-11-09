---
title: biomcp get
description: "The BioMCP 1.0.0 command biomcp get: Retrieve one entity and its
  source sections."
goal: Look up the arguments, sections and examples of biomcp get.
reader: A BioMCP user or agent checking one command
sidebar:
  label: get
kind: reference
verified: 2026-11-09
---

`biomcp get` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It retrieve one [entity](/concepts/progressive-disclosure/) and its source sections. This page lists its examples and reply.

The program's help describes it:

> Retrieve one entity and its source sections.

## Usage

```text
biomcp get
```

## Examples

The program's help gives this example.

```sh
biomcp get biomarker C96022
```

## Reply

The reply is one JSON object. Its schema is one of [`biomcp.adverse-event-devices/4`](/reference/schemas/biomcp-adverse-event-devices-4/), [`biomcp.adverse-event-drugs/4`](/reference/schemas/biomcp-adverse-event-drugs-4/), [`biomcp.adverse-event-faers-card/5`](/reference/schemas/biomcp-adverse-event-faers-card-5/), [`biomcp.adverse-event-identifiers/4`](/reference/schemas/biomcp-adverse-event-identifiers-4/), [`biomcp.adverse-event-maude-card/4`](/reference/schemas/biomcp-adverse-event-maude-card-4/), [`biomcp.adverse-event-narrative/4`](/reference/schemas/biomcp-adverse-event-narrative-4/), [`biomcp.adverse-event-reactions/6`](/reference/schemas/biomcp-adverse-event-reactions-6/), [`biomcp.article-cards/9`](/reference/schemas/biomcp-article-cards-9/), [`biomcp.author-card/6`](/reference/schemas/biomcp-author-card-6/), [`biomcp.biomarker-result/4`](/reference/schemas/biomcp-biomarker-result-4/), [`biomcp.cell-line-card/8`](/reference/schemas/biomcp-cell-line-card-8/), [`biomcp.cell-line-chembl/7`](/reference/schemas/biomcp-cell-line-chembl-7/), [`biomcp.cell-line-identity/6`](/reference/schemas/biomcp-cell-line-identity-6/), [`biomcp.cell-line-names/5`](/reference/schemas/biomcp-cell-line-names-5/), [`biomcp.cell-line-variants/7`](/reference/schemas/biomcp-cell-line-variants-7/), [`biomcp.cell-line-xrefs/6`](/reference/schemas/biomcp-cell-line-xrefs-6/), [`biomcp.diagnostic-card/4`](/reference/schemas/biomcp-diagnostic-card-4/), [`biomcp.diagnostic-conditions/5`](/reference/schemas/biomcp-diagnostic-conditions-5/), [`biomcp.diagnostic-genes/5`](/reference/schemas/biomcp-diagnostic-genes-5/), [`biomcp.diagnostic-methods/4`](/reference/schemas/biomcp-diagnostic-methods-4/), [`biomcp.diagnostic-regulatory/3`](/reference/schemas/biomcp-diagnostic-regulatory-3/), [`biomcp.diagnostic-summary/1`](/reference/schemas/biomcp-diagnostic-summary-1/), [`biomcp.diagnostic-supplements/3`](/reference/schemas/biomcp-diagnostic-supplements-3/), [`biomcp.disease-card/6`](/reference/schemas/biomcp-disease-card-6/), [`biomcp.disease-genes/9`](/reference/schemas/biomcp-disease-genes-9/), [`biomcp.disease-hierarchy/5`](/reference/schemas/biomcp-disease-hierarchy-5/), [`biomcp.disease-incidence/7`](/reference/schemas/biomcp-disease-incidence-7/), [`biomcp.disease-page/5`](/reference/schemas/biomcp-disease-page-5/), [`biomcp.disease-phenotypes/9`](/reference/schemas/biomcp-disease-phenotypes-9/), [`biomcp.disease-sites/6`](/reference/schemas/biomcp-disease-sites-6/), [`biomcp.disease-survival/7`](/reference/schemas/biomcp-disease-survival-7/), [`biomcp.disease-synonyms/5`](/reference/schemas/biomcp-disease-synonyms-5/), [`biomcp.disease-xrefs/5`](/reference/schemas/biomcp-disease-xrefs-5/), [`biomcp.drug-approvals/8`](/reference/schemas/biomcp-drug-approvals-8/), [`biomcp.drug-card/6`](/reference/schemas/biomcp-drug-card-6/), [`biomcp.drug-civic/11`](/reference/schemas/biomcp-drug-civic-11/), [`biomcp.drug-indications/8`](/reference/schemas/biomcp-drug-indications-8/), [`biomcp.drug-interactions/8`](/reference/schemas/biomcp-drug-interactions-8/), [`biomcp.drug-label/7`](/reference/schemas/biomcp-drug-label-7/), [`biomcp.drug-recall-card/3`](/reference/schemas/biomcp-drug-recall-card-3/), [`biomcp.drug-regulatory/6`](/reference/schemas/biomcp-drug-regulatory-6/), [`biomcp.drug-targets/8`](/reference/schemas/biomcp-drug-targets-8/), [`biomcp.drug-who-card/3`](/reference/schemas/biomcp-drug-who-card-3/), [`biomcp.drug-xrefs/5`](/reference/schemas/biomcp-drug-xrefs-5/), [`biomcp.gene-cards/13`](/reference/schemas/biomcp-gene-cards-13/), [`biomcp.gwas-ancestries/5`](/reference/schemas/biomcp-gwas-ancestries-5/), [`biomcp.gwas-association/6`](/reference/schemas/biomcp-gwas-association-6/), [`biomcp.gwas-loci/4`](/reference/schemas/biomcp-gwas-loci-4/), [`biomcp.gwas-page/8`](/reference/schemas/biomcp-gwas-page-8/), [`biomcp.gwas-study/6`](/reference/schemas/biomcp-gwas-study-6/), [`biomcp.nci-trial-cards/8`](/reference/schemas/biomcp-nci-trial-cards-8/), [`biomcp.pathway-card/10`](/reference/schemas/biomcp-pathway-card-10/), [`biomcp.pgx-annotation/5`](/reference/schemas/biomcp-pgx-annotation-5/), [`biomcp.pgx-annotations/5`](/reference/schemas/biomcp-pgx-annotations-5/), [`biomcp.pgx-card/4`](/reference/schemas/biomcp-pgx-card-4/), [`biomcp.pgx-diplotype/4`](/reference/schemas/biomcp-pgx-diplotype-4/), [`biomcp.pgx-guidelines/4`](/reference/schemas/biomcp-pgx-guidelines-4/), [`biomcp.pgx-recommendations/5`](/reference/schemas/biomcp-pgx-recommendations-5/), [`biomcp.phenotype-card/5`](/reference/schemas/biomcp-phenotype-card-5/), [`biomcp.phenotype-diseases/7`](/reference/schemas/biomcp-phenotype-diseases-7/), [`biomcp.phenotype-genes/7`](/reference/schemas/biomcp-phenotype-genes-7/), [`biomcp.phenotype-hierarchy/5`](/reference/schemas/biomcp-phenotype-hierarchy-5/), [`biomcp.phenotype-synonyms/4`](/reference/schemas/biomcp-phenotype-synonyms-4/), [`biomcp.phenotype-xrefs/4`](/reference/schemas/biomcp-phenotype-xrefs-4/), [`biomcp.protein-card/10`](/reference/schemas/biomcp-protein-card-10/), [`biomcp.protein-function/4`](/reference/schemas/biomcp-protein-function-4/), [`biomcp.study-card/7`](/reference/schemas/biomcp-study-card-7/), [`biomcp.study-files/3`](/reference/schemas/biomcp-study-files-3/), [`biomcp.study-mutations/1`](/reference/schemas/biomcp-study-mutations-1/), [`biomcp.study-profiles/4`](/reference/schemas/biomcp-study-profiles-4/), [`biomcp.study-sample-lists/4`](/reference/schemas/biomcp-study-sample-lists-4/), [`biomcp.study-samples/1`](/reference/schemas/biomcp-study-samples-1/), [`biomcp.trial-cards/9`](/reference/schemas/biomcp-trial-cards-9/), [`biomcp.variant-card/15`](/reference/schemas/biomcp-variant-card-15/), whose page lists its fields.

## Next

The [commands index](/reference/commands/) lists every command.
