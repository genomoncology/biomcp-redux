---
title: NIH iCite (icite)
description: "How BioMCP 1.0.0 uses NIH iCite: commands, key, use group and
  publisher terms."
goal: Find what BioMCP reads from NIH iCite, which commands reach it, and its terms.
reader: A BioMCP user checking one data source
sidebar:
  label: icite
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 names this source `icite`. BioMCP reads citation counts and citation links from NIH iCite. It is the default citation source.

| Field | Value |
| --- | --- |
| Source name | `icite` |
| Use group | `open` |
| Terms | NIH public-domain data |
| Terms page | <https://support.icite.nih.gov/hc/en-us/articles/9066501755291-Citations-Module-Overview> |
| Key | none |

## Commands

| Command | Description |
| --- | --- |
| [`biomcp article citations`](/reference/commands/article-citations/) | Page the citing or referenced PubMed papers of one checked PMID. |
| [`biomcp article references`](/reference/commands/article-references/) | Page the citing or referenced PubMed papers of one checked PMID. |
| [`biomcp get article`](/reference/commands/get-article/) | Get one to twenty article inputs in caller order. |

Select this source with `--source icite` on the commands that list it.

## Turn it off

Leave this source out of one command with `--disable-sources icite`, or add `"icite"` to `disable_sources` in `config.json`. The group `open` switches off every source in that use group. See [keys and configuration](/getting-started/api-keys/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
