---
title: NCBI Genetic Testing Registry (gtr)
description: "How BioMCP 1.0.0 uses NCBI Genetic Testing Registry: commands,
  key, use group and publisher terms."
goal: Find what BioMCP reads from NCBI Genetic Testing Registry, which commands
  reach it, and its terms.
reader: A BioMCP user checking one data source
sidebar:
  label: gtr
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 names this source `gtr`. BioMCP reads GTR test listings with their genes, conditions and methods from a local bundle.

| Field | Value |
| --- | --- |
| Source name | `gtr` |
| Use group | `open` |
| Terms | NCBI usage policies; submitter and third-party rights retained |
| Terms page | <https://www.ncbi.nlm.nih.gov/home/about/policies/> |
| Key | none |
| Local bundle | `biomcp sync gtr` |

## Commands

| Command | Description |
| --- | --- |
| [`biomcp search diagnostic`](/reference/commands/search-diagnostic/) | Search one diagnostic source. |
| [`biomcp get diagnostic`](/reference/commands/get-diagnostic/) | Get one exact source record or supported section. |

Select this source with `--source gtr` on the commands that list it.

## Turn it off

Leave this source out of one command with `--disable-sources gtr`, or add `"gtr"` to `disable_sources` in `config.json`. The group `open` switches off every source in that use group. See [keys and configuration](/getting-started/api-keys/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
