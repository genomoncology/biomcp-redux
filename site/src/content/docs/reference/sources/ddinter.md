---
title: DDInter (ddinter)
description: "How BioMCP 1.0.0 uses DDInter: commands, key, use group and publisher terms."
goal: Find what BioMCP reads from DDInter, which commands reach it, and its terms.
reader: A BioMCP user checking one data source
sidebar:
  label: ddinter
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 names this source `ddinter`. BioMCP reads drug-drug interaction partners and levels from a local bundle.

| Field | Value |
| --- | --- |
| Source name | `ddinter` |
| Use group | `noncommercial` |
| Terms | DDInter CC BY-NC-SA 4.0 |
| Terms page | <https://ddinter.scbdd.com/terms/> |
| Key | none |
| Local bundle | `biomcp sync ddinter` |

## Commands

| Command | Description |
| --- | --- |
| [`biomcp search drug`](/reference/commands/search-drug/) | Literal single-source candidate discovery, without an implicit identity selection. |
| [`biomcp get drug`](/reference/commands/get-drug/) | Exact native ID retrieval with an explicitly selected source. |

Select this source with `--source ddinter` on the commands that list it.

## Turn it off

Leave this source out of one command with `--disable-sources ddinter`, or add `"ddinter"` to `disable_sources` in `config.json`. The group `noncommercial` switches off every source in that use group. See [keys and configuration](/keys-and-configuration/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
