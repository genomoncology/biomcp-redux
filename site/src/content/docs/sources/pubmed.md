---
title: PubMed (pubmed)
description: "How BioMCP 1.0.0 uses PubMed: commands, key, use group and publisher terms."
goal: Find what BioMCP reads from PubMed, which commands reach it, and its terms.
reader: A BioMCP user checking one data source
sidebar:
  label: pubmed
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 names this source `pubmed`. BioMCP searches PubMed and reads article citations, abstracts and MeSH indexing. It is the default article search source.

| Field | Value |
| --- | --- |
| Source name | `pubmed` |
| Use group | `restricted` |
| Terms | NCBI policies; publisher rights in abstracts |
| Terms page | <https://www.ncbi.nlm.nih.gov/home/about/policies/> |
| Key | optional or required key: `biomcp key set ncbi`, environment `NCBI_API_KEY`, configuration `keys.ncbi_api_key`. See [`help key ncbi`](/reference/commands/help-key-ncbi/). |

## Commands

| Command | Description |
| --- | --- |
| [`biomcp search article`](/reference/commands/search-article/) | Checked search selectors and pool cursor. |
| [`biomcp get article`](/reference/commands/get-article/) | Get one to twenty article inputs in caller order. |

Select this source with `--source pubmed` on the commands that list it.

## Turn it off

Leave this source out of one command with `--disable-sources pubmed`, or add `"pubmed"` to `disable_sources` in `config.json`. The group `restricted` switches off every source in that use group. See [keys and configuration](/getting-started/api-keys/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
