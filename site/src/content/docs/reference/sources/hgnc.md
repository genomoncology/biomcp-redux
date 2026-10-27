---
title: HGNC (hgnc)
description: "How BioMCP 1.0.0 uses HGNC: commands, key, use group and publisher terms."
goal: Find what BioMCP reads from HGNC, which commands reach it, and its terms.
reader: A BioMCP user checking one data source
sidebar:
  label: hgnc
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 names this source `hgnc`. BioMCP resolves gene symbols, aliases and identifiers against a local HGNC bundle that it can synchronize when a read needs it.

| Field | Value |
| --- | --- |
| Source name | `hgnc` |
| Use group | `open` |
| Terms | CC0 1.0 |
| Terms page | <https://www.genenames.org/about/license/> |
| Key | none |
| Local bundle | `biomcp sync hgnc` |

## Commands

| Command | Description |
| --- | --- |
| [`biomcp search gene`](/reference/commands/search-gene/) | Search genes by text, cytoband, or their intersection. |
| [`biomcp get gene`](/reference/commands/get-gene/) | Get one to twenty human gene inputs in their original order. |

## Turn it off

Leave this source out of one command with `--disable-sources hgnc`, or add `"hgnc"` to `disable_sources` in `config.json`. The group `open` switches off every source in that use group. See [keys and configuration](/keys-and-configuration/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
