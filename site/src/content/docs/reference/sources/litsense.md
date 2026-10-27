---
title: LitSense2 (litsense)
description: "How BioMCP 1.0.0 uses LitSense2: commands, key, use group and
  publisher terms."
goal: Find what BioMCP reads from LitSense2, which commands reach it, and its terms.
reader: A BioMCP user checking one data source
sidebar:
  label: litsense
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 names this source `litsense`. BioMCP finds sentences that resemble the query text through LitSense2.

| Field | Value |
| --- | --- |
| Source name | `litsense` |
| Use group | `restricted` |
| Terms | NCBI policies; publisher rights in input text |
| Terms page | <https://www.ncbi.nlm.nih.gov/home/about/policies/> |
| Key | none |

## Commands

| Command | Description |
| --- | --- |
| [`biomcp search article`](/reference/commands/search-article/) | Checked search selectors and pool cursor. |

Select this source with `--source litsense` on the commands that list it.

## Turn it off

Leave this source out of one command with `--disable-sources litsense`, or add `"litsense"` to `disable_sources` in `config.json`. The group `restricted` switches off every source in that use group. See [keys and configuration](/keys-and-configuration/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
