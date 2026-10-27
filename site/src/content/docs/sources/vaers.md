---
title: VAERS through CDC WONDER (vaers)
description: "How BioMCP 1.0.0 uses VAERS through CDC WONDER: commands, key, use
  group and publisher terms."
goal: Find what BioMCP reads from VAERS through CDC WONDER, which commands reach
  it, and its terms.
reader: A BioMCP user checking one data source
sidebar:
  label: vaers
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 names this source `vaers`. BioMCP reads VAERS report summaries for MMR, COVID-19 and named influenza vaccines from CDC WONDER.

| Field | Value |
| --- | --- |
| Source name | `vaers` |
| Use group | `restricted` |
| Terms | CDC WONDER data use restrictions |
| Terms page | <https://wonder.cdc.gov/datause.html> |
| Key | none |

## Commands

| Command | Description |
| --- | --- |
| [`biomcp search adverse-event`](/reference/commands/search-adverse-event/) | Search five report headers by default. |

Select this source with `--source vaers` on the commands that list it.

## Turn it off

Leave this source out of one command with `--disable-sources vaers`, or add `"vaers"` to `disable_sources` in `config.json`. The group `restricted` switches off every source in that use group. See [keys and configuration](/getting-started/api-keys/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
