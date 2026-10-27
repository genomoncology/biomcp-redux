---
title: openFDA 510(k) (fda-510k)
description: "How BioMCP 1.0.0 uses openFDA 510(k): commands, key, use group and
  publisher terms."
goal: Find what BioMCP reads from openFDA 510(k), which commands reach it, and
  its terms.
reader: A BioMCP user checking one data source
sidebar:
  label: fda-510k
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 names this source `fda-510k`. BioMCP reads one 510(k) clearance record.

| Field | Value |
| --- | --- |
| Source name | `fda-510k` |
| Use group | `open` |
| Terms | openFDA terms and license; third-party exceptions |
| Terms page | <https://open.fda.gov/terms/> |
| Key | none |

## Commands

| Command | Description |
| --- | --- |
| [`biomcp get diagnostic`](/reference/commands/get-diagnostic/) | Get one exact source record or supported section. |

Select this source with `--source fda-510k` on the commands that list it.

## Turn it off

Leave this source out of one command with `--disable-sources fda-510k`, or add `"fda-510k"` to `disable_sources` in `config.json`. The group `open` switches off every source in that use group. See [keys and configuration](/keys-and-configuration/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
