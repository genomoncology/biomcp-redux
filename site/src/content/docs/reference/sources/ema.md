---
title: European Medicines Agency (ema)
description: "How BioMCP 1.0.0 uses European Medicines Agency: commands, key,
  use group and publisher terms."
goal: Find what BioMCP reads from European Medicines Agency, which commands
  reach it, and its terms.
reader: A BioMCP user checking one data source
sidebar:
  label: ema
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 names this source `ema`. BioMCP reads EMA medicine records and regulatory events from a local bundle.

| Field | Value |
| --- | --- |
| Source name | `ema` |
| Use group | `attribution` |
| Terms | EMA reproduction and attribution terms; third-party exceptions |
| Terms page | <https://www.ema.europa.eu/en/about-us/about-website/legal-notice> |
| Key | none |
| Local bundle | `biomcp sync ema` |

## Commands

| Command | Description |
| --- | --- |
| [`biomcp search drug`](/reference/commands/search-drug/) | Literal single-source candidate discovery, without an implicit identity selection. |
| [`biomcp get drug`](/reference/commands/get-drug/) | Exact native ID retrieval with an explicitly selected source. |

Select this source with `--source ema` on the commands that list it.

## Turn it off

Leave this source out of one command with `--disable-sources ema`, or add `"ema"` to `disable_sources` in `config.json`. The group `attribution` switches off every source in that use group. See [keys and configuration](/keys-and-configuration/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
