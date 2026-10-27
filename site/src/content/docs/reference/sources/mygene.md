---
title: MyGene.info (mygene)
description: "How BioMCP 1.0.0 uses MyGene.info: commands, key, use group and
  publisher terms."
goal: Find what BioMCP reads from MyGene.info, which commands reach it, and its
  terms.
reader: A BioMCP user checking one data source
sidebar:
  label: mygene
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 names this source `mygene`. BioMCP reads gene summaries through MyGene.info for the gene card.

| Field | Value |
| --- | --- |
| Source name | `mygene` |
| Use group | `restricted` |
| Terms | MyGene.info Terms of Use; upstream data terms |
| Terms page | <https://docs.mygene.info/en/latest/terms.html> |
| Key | none |

## Commands

| Command | Description |
| --- | --- |
| [`biomcp get gene`](/reference/commands/get-gene/) | Get one to twenty human gene inputs in their original order. |

## Turn it off

Leave this source out of one command with `--disable-sources mygene`, or add `"mygene"` to `disable_sources` in `config.json`. The group `restricted` switches off every source in that use group. See [keys and configuration](/keys-and-configuration/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
