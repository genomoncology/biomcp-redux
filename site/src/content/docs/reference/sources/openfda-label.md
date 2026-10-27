---
title: openFDA drug labels (openfda-label)
description: "How BioMCP 1.0.0 uses openFDA drug labels: commands, key, use
  group and publisher terms."
goal: Find what BioMCP reads from openFDA drug labels, which commands reach it,
  and its terms.
reader: A BioMCP user checking one data source
sidebar:
  label: openfda-label
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 names this source `openfda-label`. BioMCP reads one exact heading of an FDA drug label.

| Field | Value |
| --- | --- |
| Source name | `openfda-label` |
| Use group | `open` |
| Terms | openFDA terms; original label rights retained |
| Terms page | <https://open.fda.gov/terms/> |
| Key | optional or required key: `biomcp key set openfda`, environment `OPENFDA_API_KEY`, configuration `keys.openfda_api_key`. See [`help key openfda`](/reference/commands/help-key-openfda/). |

## Commands

| Command | Description |
| --- | --- |
| [`biomcp search drug`](/reference/commands/search-drug/) | Literal single-source candidate discovery, without an implicit identity selection. |
| [`biomcp get drug`](/reference/commands/get-drug/) | Exact native ID retrieval with an explicitly selected source. |

Select this source with `--source openfda-label` on the commands that list it.

## Turn it off

Leave this source out of one command with `--disable-sources openfda-label`, or add `"openfda-label"` to `disable_sources` in `config.json`. The group `open` switches off every source in that use group. See [keys and configuration](/keys-and-configuration/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
