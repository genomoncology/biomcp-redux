---
title: CPIC (cpic)
description: "How BioMCP 1.0.0 uses CPIC: commands, key, use group and publisher terms."
goal: Find what BioMCP reads from CPIC, which commands reach it, and its terms.
reader: A BioMCP user checking one data source
sidebar:
  label: cpic
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 names this source `cpic`. BioMCP reads CPIC gene cards, recommendations, guidelines and diplotype lookups.

| Field | Value |
| --- | --- |
| Source name | `cpic` |
| Use group | `noncommercial` |
| Terms | ClinPGx data use policy; noncommercial access restriction; captured policy was a JavaScript shell |
| Terms page | <https://www.clinpgx.org/page/dataUsagePolicy> |
| Key | none |

## Commands

| Command | Description |
| --- | --- |
| [`biomcp search pgx`](/reference/commands/search-pgx/) | Checked gene-keyed CPIC pair search. |
| [`biomcp get pgx`](/reference/commands/get-pgx/) | PGx card or section. |

Select this source with `--source cpic` on the commands that list it.

## Turn it off

Leave this source out of one command with `--disable-sources cpic`, or add `"cpic"` to `disable_sources` in `config.json`. The group `noncommercial` switches off every source in that use group. See [keys and configuration](/getting-started/api-keys/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
