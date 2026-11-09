---
title: Entities and sections
description: The 16 BioMCP 1.0.0 entities, their search and get commands,
  relation commands and section words.
goal: Find which commands and sections each BioMCP entity has.
reader: A BioMCP user or agent choosing a command
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources, groups its commands by [entity](/concepts/progressive-disclosure/). An **entity** is a kind of record, such as a gene, a trial or an article. BioMCP has 16. Each row lists the entity's commands and the [section words](/concepts/progressive-disclosure/) its `get` command accepts. A section word asks for one more part of the record.

| Entity | Commands | Sections of get |
| --- | --- | --- |
| author | [author papers](/reference/commands/author-papers/)<br/>[search author](/reference/commands/search-author/)<br/>[get author](/reference/commands/get-author/) |  |
| cell-line | [cell-line assay-count](/reference/commands/cell-line-assay-count/)<br/>[search cell-line](/reference/commands/search-cell-line/)<br/>[get cell-line](/reference/commands/get-cell-line/) | `names` `identity` `xrefs` `variants` `chembl` `all` |
| gwas | [get gwas](/reference/commands/get-gwas/)<br/>[search gwas](/reference/commands/search-gwas/) | `loci` `associations` `ancestries` |
| protein | [protein features](/reference/commands/protein-features/)<br/>[protein isoforms](/reference/commands/protein-isoforms/)<br/>[protein domains](/reference/commands/protein-domains/)<br/>[protein complexes](/reference/commands/protein-complexes/)<br/>[protein complex-members](/reference/commands/protein-complex-members/)<br/>[protein structures](/reference/commands/protein-structures/)<br/>[protein sequence](/reference/commands/protein-sequence/)<br/>[search protein](/reference/commands/search-protein/)<br/>[get protein](/reference/commands/get-protein/) | `features` `domains` `isoforms` `complexes` `structures` `function` `sequence` `all` |
| phenotype | [search phenotype](/reference/commands/search-phenotype/)<br/>[get phenotype](/reference/commands/get-phenotype/) | `synonyms` `xrefs` `hierarchy` `diseases` `genes` |
| pathway | [search pathway](/reference/commands/search-pathway/)<br/>[get pathway](/reference/commands/get-pathway/)<br/>[pathway genes](/reference/commands/pathway-genes/)<br/>[pathway events](/reference/commands/pathway-events/) | `summary` `genes` `events` `all` |
| study | [search study](/reference/commands/search-study/)<br/>[get study](/reference/commands/get-study/) | `profiles` `sample-lists` `files` `all` |
| pgx | [search pgx](/reference/commands/search-pgx/)<br/>[get pgx](/reference/commands/get-pgx/) | `recommendations` `guidelines` `diplotype` `annotations` |
| diagnostic | [search diagnostic](/reference/commands/search-diagnostic/)<br/>[get diagnostic](/reference/commands/get-diagnostic/) | `genes` `conditions` `methods` `regulatory` `supplements` |
| adverse-event | [search adverse-event](/reference/commands/search-adverse-event/)<br/>[get adverse-event](/reference/commands/get-adverse-event/) | `drugs` `reactions` `unii` `rxcui` `product-ndc` `package-ndc` `spl-id` `spl-set-id` `application-number` `devices` `narrative` |
| disease | [search disease](/reference/commands/search-disease/)<br/>[get disease](/reference/commands/get-disease/) | `synonyms` `xrefs` `hierarchy` `genes` `phenotypes` `incidence` `survival` |
| drug | [search drug](/reference/commands/search-drug/)<br/>[get drug](/reference/commands/get-drug/) | `xrefs` `targets` `indications` `civic` `label` `approvals` `regulatory` `interactions` |
| gene | [get gene](/reference/commands/get-gene/)<br/>[search gene](/reference/commands/search-gene/) | `protein` `civic` `clingen` `constraint` `expression` `pathways` `oncokb` `all` |
| article | [search article](/reference/commands/search-article/)<br/>[get article](/reference/commands/get-article/)<br/>[article supplement](/reference/commands/article-supplement/)<br/>[article pdf](/reference/commands/article-pdf/)<br/>[article text](/reference/commands/article-text/)<br/>[article citations](/reference/commands/article-citations/)<br/>[article references](/reference/commands/article-references/)<br/>[article citation-evidence](/reference/commands/article-citation-evidence/)<br/>[article recommendations](/reference/commands/article-recommendations/) | `annotations` `fulltext` `supplements` `tldr` `citations` `indexing` `pdf` `all` |
| trial | [search trial](/reference/commands/search-trial/)<br/>[get trial](/reference/commands/get-trial/)<br/>[trial document](/reference/commands/trial-document/) | `eligibility` `outcomes` `arms` `locations` `contacts` `references` `documents` `all` `diseases` `summary` |
| variant | [variant assertions](/reference/commands/variant-assertions/)<br/>[get variant](/reference/commands/get-variant/)<br/>[variant population](/reference/commands/variant-population/)<br/>[search variant](/reference/commands/search-variant/)<br/>[variant recurrence](/reference/commands/variant-recurrence/)<br/>[variant normalize](/reference/commands/variant-normalize/) | `clinvar` `erepo` `population` `gwas` `civic` `oncokb` `hotspots` `predict` `conservation` `cosmic` `all` |
