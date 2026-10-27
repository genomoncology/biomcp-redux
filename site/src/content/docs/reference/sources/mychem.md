---
title: MyChem.info (mychem)
description: "How BioMCP 1.0.0 uses MyChem.info: commands, key, use group and
  publisher terms."
goal: Find what BioMCP reads from MyChem.info, which commands reach it, and its
  terms.
reader: A BioMCP user checking one data source
sidebar:
  label: mychem
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 names this source `mychem`. BioMCP searches drugs through MyChem.info. It is the default drug source.

| Field | Value |
| --- | --- |
| Source name | `mychem` |
| Use group | `attribution` |
| Terms | BioThings and upstream source terms |
| Terms page | <https://docs.mychem.info/en/latest/doc/data_source.html> |
| Key | none |

## Commands

| Command | Description |
| --- | --- |
| [`biomcp search drug`](/reference/commands/search-drug/) | Literal single-source candidate discovery, without an implicit identity selection. |
| [`biomcp get drug`](/reference/commands/get-drug/) | Exact native ID retrieval with an explicitly selected source. |

Select this source with `--source mychem` on the commands that list it.

## Turn it off

Leave this source out of one command with `--disable-sources mychem`, or add `"mychem"` to `disable_sources` in `config.json`. The group `attribution` switches off every source in that use group. See [keys and configuration](/keys-and-configuration/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
