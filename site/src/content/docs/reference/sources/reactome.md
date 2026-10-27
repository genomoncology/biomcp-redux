---
title: Reactome (reactome)
description: "How BioMCP 1.0.0 uses Reactome: commands, key, use group and publisher terms."
goal: Find what BioMCP reads from Reactome, which commands reach it, and its terms.
reader: A BioMCP user checking one data source
sidebar:
  label: reactome
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 names this source `reactome`. BioMCP searches Reactome pathways and reads their genes and events, and the pathways section of a gene.

| Field | Value |
| --- | --- |
| Source name | `reactome` |
| Use group | `open` |
| Terms | CC0 1.0 database data |
| Terms page | <https://reactome.org/license> |
| Key | none |

## Commands

| Command | Description |
| --- | --- |
| [`biomcp search pathway`](/reference/commands/search-pathway/) | Search Reactome (default) or WikiPathways human pathways. |
| [`biomcp get pathway`](/reference/commands/get-pathway/) | Get an exact human Reactome R-HSA or WikiPathways WP card. |
| [`biomcp pathway genes`](/reference/commands/pathway-genes/) | Read Reactome exact references or WikiPathways GeneProduct data nodes. |
| [`biomcp pathway events`](/reference/commands/pathway-events/) | Read Reactome event objects and unresolved native integer references in source order. |
| [`biomcp get gene`](/reference/commands/get-gene/) | Get one to twenty human gene inputs in their original order. |

Select this source with `--source reactome` on the commands that list it.

## Turn it off

Leave this source out of one command with `--disable-sources reactome`, or add `"reactome"` to `disable_sources` in `config.json`. The group `open` switches off every source in that use group. See [keys and configuration](/keys-and-configuration/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
