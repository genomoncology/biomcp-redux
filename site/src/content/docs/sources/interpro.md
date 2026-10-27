---
title: InterPro (interpro)
description: "How BioMCP 1.0.0 uses InterPro: commands, key, use group and publisher terms."
goal: Find what BioMCP reads from InterPro, which commands reach it, and its terms.
reader: A BioMCP user checking one data source
sidebar:
  label: interpro
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 names this source `interpro`. BioMCP reads InterPro domain and family matches for a protein.

| Field | Value |
| --- | --- |
| Source name | `interpro` |
| Use group | `open` |
| Terms | CC0 |
| Terms page | <https://interpro-documentation.readthedocs.io/en/latest/license.html> |
| Key | none |

## Commands

| Command | Description |
| --- | --- |
| [`biomcp protein domains`](/reference/commands/protein-domains/) | Page five native InterPro matches with original types and fragments. |
| [`biomcp get protein`](/reference/commands/get-protein/) | Read source protein cards; a symbol returns candidates without choosing a protein. |

## Turn it off

Leave this source out of one command with `--disable-sources interpro`, or add `"interpro"` to `disable_sources` in `config.json`. The group `open` switches off every source in that use group. See [keys and configuration](/getting-started/api-keys/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
