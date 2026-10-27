---
title: GWAS Catalog (gwas)
description: "How BioMCP 1.0.0 uses GWAS Catalog: commands, key, use group and
  publisher terms."
goal: Find what BioMCP reads from GWAS Catalog, which commands reach it, and its
  terms.
reader: A BioMCP user checking one data source
sidebar:
  label: gwas
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 names this source `gwas`. BioMCP reads GWAS Catalog studies, associations, loci and ancestries, and the gwas section of a variant.

| Field | Value |
| --- | --- |
| Source name | `gwas` |
| Use group | `restricted` |
| Terms | GWAS Catalog terms and dataset rights |
| Terms page | <https://www.ebi.ac.uk/about/terms-of-use/> |
| Key | none |

## Commands

| Command | Description |
| --- | --- |
| [`biomcp search gwas`](/reference/commands/search-gwas/) | Source associations in default p-value-ascending order. |
| [`biomcp get gwas`](/reference/commands/get-gwas/) | Source association or study and native context. |

## Turn it off

Leave this source out of one command with `--disable-sources gwas`, or add `"gwas"` to `disable_sources` in `config.json`. The group `restricted` switches off every source in that use group. See [keys and configuration](/keys-and-configuration/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
