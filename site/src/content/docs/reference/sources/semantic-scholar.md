---
title: Semantic Scholar (semantic-scholar)
description: "How BioMCP 1.0.0 uses Semantic Scholar: commands, key, use group
  and publisher terms."
goal: Find what BioMCP reads from Semantic Scholar, which commands reach it, and
  its terms.
reader: A BioMCP user checking one data source
sidebar:
  label: semantic-scholar
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 names this source `semantic-scholar`. BioMCP searches Semantic Scholar papers and authors and reads citations, references, recommendations and summaries.

| Field | Value |
| --- | --- |
| Source name | `semantic-scholar` |
| Use group | `noncommercial` |
| Terms | Semantic Scholar API License Agreement |
| Terms page | <https://api.semanticscholar.org/license/> |
| Key | optional or required key: `biomcp key set semantic-scholar`, environment `S2_API_KEY`, configuration `keys.semantic_scholar_api_key`. See [`help key semantic-scholar`](/reference/commands/help-key-semantic-scholar/). |

## Commands

| Command | Description |
| --- | --- |
| [`biomcp search author`](/reference/commands/search-author/) | Literal author name query and native window; positional text maps to query. |
| [`biomcp search article`](/reference/commands/search-article/) | Checked search selectors and pool cursor. |
| [`biomcp article citations`](/reference/commands/article-citations/) | Page the citing or referenced PubMed papers of one checked PMID. |
| [`biomcp article references`](/reference/commands/article-references/) | Page the citing or referenced PubMed papers of one checked PMID. |
| [`biomcp get author`](/reference/commands/get-author/) | Exact qualified records in caller order, with no name reconciliation. |
| [`biomcp author papers`](/reference/commands/author-papers/) | One exact provider record's publication window. |
| [`biomcp article citation-evidence`](/reference/commands/article-citation-evidence/) | Read every source context and optional explicit full-text citation paragraph. |
| [`biomcp article recommendations`](/reference/commands/article-recommendations/) | Recommendations in the provider's recent-paper pool order. |
| [`biomcp get article`](/reference/commands/get-article/) | Get one to twenty article inputs in caller order. |

Select this source with `--source semantic-scholar` on the commands that list it.

## Turn it off

Leave this source out of one command with `--disable-sources semantic-scholar`, or add `"semantic-scholar"` to `disable_sources` in `config.json`. The group `noncommercial` switches off every source in that use group. See [keys and configuration](/keys-and-configuration/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
