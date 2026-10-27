---
title: LitVar2 (litvar)
description: "How BioMCP 1.0.0 uses LitVar2: commands, key, use group and publisher terms."
goal: Find what BioMCP reads from LitVar2, which commands reach it, and its terms.
reader: A BioMCP user checking one data source
sidebar:
  label: litvar
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 names this source `litvar`. BioMCP finds publications that mention an exact variant through LitVar2.

| Field | Value |
| --- | --- |
| Source name | `litvar` |
| Use group | `open` |
| Terms | NCBI public-domain metadata policy |
| Terms page | <https://www.ncbi.nlm.nih.gov/home/about/policies/> |
| Key | none |

## Commands

| Command | Description |
| --- | --- |
| [`biomcp search article`](/reference/commands/search-article/) | Checked search selectors and pool cursor. |

Select this source with `--source litvar` on the commands that list it.

## Turn it off

Leave this source out of one command with `--disable-sources litvar`, or add `"litvar"` to `disable_sources` in `config.json`. The group `open` switches off every source in that use group. See [keys and configuration](/getting-started/api-keys/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
