---
title: WikiPathways (wikipathways)
description: "How BioMCP 1.0.0 uses WikiPathways: commands, key, use group and
  publisher terms."
goal: Find what BioMCP reads from WikiPathways, which commands reach it, and its
  terms.
reader: A BioMCP user checking one data source
sidebar:
  label: wikipathways
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 names this source `wikipathways`. BioMCP searches WikiPathways and reads pathway members from a local bundle.

| Field | Value |
| --- | --- |
| Source name | `wikipathways` |
| Use group | `open` |
| Terms | CC0 1.0 waiver |
| Terms page | <https://www.wikipathways.org/help.html> |
| Key | none |
| Local bundle | `biomcp sync wikipathways` |

## Commands

| Command | Description |
| --- | --- |
| [`biomcp search pathway`](/reference/commands/search-pathway/) | Search Reactome (default) or WikiPathways human pathways. |
| [`biomcp get pathway`](/reference/commands/get-pathway/) | Get an exact human Reactome R-HSA or WikiPathways WP card. |
| [`biomcp pathway genes`](/reference/commands/pathway-genes/) | Read Reactome exact references or WikiPathways GeneProduct data nodes. |

Select this source with `--source wikipathways` on the commands that list it.

## Turn it off

Leave this source out of one command with `--disable-sources wikipathways`, or add `"wikipathways"` to `disable_sources` in `config.json`. The group `open` switches off every source in that use group. See [keys and configuration](/keys-and-configuration/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
