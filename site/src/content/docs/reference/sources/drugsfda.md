---
title: Drugs@FDA (drugsfda)
description: "How BioMCP 1.0.0 uses Drugs@FDA: commands, key, use group and
  publisher terms."
goal: Find what BioMCP reads from Drugs@FDA, which commands reach it, and its terms.
reader: A BioMCP user checking one data source
sidebar:
  label: drugsfda
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 names this source `drugsfda`. BioMCP reads FDA application and product history for a drug.

| Field | Value |
| --- | --- |
| Source name | `drugsfda` |
| Use group | `open` |
| Terms | openFDA terms; original label rights retained |
| Terms page | <https://open.fda.gov/terms/> |
| Key | none |

## Commands

| Command | Description |
| --- | --- |
| [`biomcp search drug`](/reference/commands/search-drug/) | Literal single-source candidate discovery, without an implicit identity selection. |
| [`biomcp get drug`](/reference/commands/get-drug/) | Exact native ID retrieval with an explicitly selected source. |

Select this source with `--source drugsfda` on the commands that list it.

## Turn it off

Leave this source out of one command with `--disable-sources drugsfda`, or add `"drugsfda"` to `disable_sources` in `config.json`. The group `open` switches off every source in that use group. See [keys and configuration](/keys-and-configuration/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
