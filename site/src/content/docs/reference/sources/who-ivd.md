---
title: WHO prequalified in vitro diagnostics (who-ivd)
description: "How BioMCP 1.0.0 uses WHO prequalified in vitro diagnostics:
  commands, key, use group and publisher terms."
goal: Find what BioMCP reads from WHO prequalified in vitro diagnostics, which
  commands reach it, and its terms.
reader: A BioMCP user checking one data source
sidebar:
  label: who-ivd
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 names this source `who-ivd`. BioMCP reads WHO prequalified diagnostic products from a local copy of the WHO export.

| Field | Value |
| --- | --- |
| Source name | `who-ivd` |
| Use group | `noncommercial` |
| Terms | WHO terms of use; export has no narrower license; commercial and substantial reproduction reserved |
| Terms page | <https://www.who.int/about/policies/terms-of-use> |
| Key | none |
| Local bundle | `biomcp sync who-ivd` |

## Commands

| Command | Description |
| --- | --- |
| [`biomcp search diagnostic`](/reference/commands/search-diagnostic/) | Search one diagnostic source. |
| [`biomcp get diagnostic`](/reference/commands/get-diagnostic/) | Get one exact source record or supported section. |

Select this source with `--source who-ivd` on the commands that list it.

## Turn it off

Leave this source out of one command with `--disable-sources who-ivd`, or add `"who-ivd"` to `disable_sources` in `config.json`. The group `noncommercial` switches off every source in that use group. See [keys and configuration](/keys-and-configuration/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
