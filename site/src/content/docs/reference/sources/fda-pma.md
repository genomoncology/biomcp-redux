---
title: openFDA premarket approval (fda-pma)
description: "How BioMCP 1.0.0 uses openFDA premarket approval: commands, key,
  use group and publisher terms."
goal: Find what BioMCP reads from openFDA premarket approval, which commands
  reach it, and its terms.
reader: A BioMCP user checking one data source
sidebar:
  label: fda-pma
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 names this source `fda-pma`. BioMCP reads one PMA submission, its regulatory statement and its supplements.

| Field | Value |
| --- | --- |
| Source name | `fda-pma` |
| Use group | `open` |
| Terms | openFDA terms and license; third-party exceptions |
| Terms page | <https://open.fda.gov/terms/> |
| Key | none |

## Commands

| Command | Description |
| --- | --- |
| [`biomcp get diagnostic`](/reference/commands/get-diagnostic/) | Get one exact source record or supported section. |

Select this source with `--source fda-pma` on the commands that list it.

## Turn it off

Leave this source out of one command with `--disable-sources fda-pma`, or add `"fda-pma"` to `disable_sources` in `config.json`. The group `open` switches off every source in that use group. See [keys and configuration](/keys-and-configuration/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
