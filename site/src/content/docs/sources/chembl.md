---
title: ChEMBL (chembl)
description: "How BioMCP 1.0.0 uses ChEMBL: commands, key, use group and publisher terms."
goal: Find what BioMCP reads from ChEMBL, which commands reach it, and its terms.
reader: A BioMCP user checking one data source
sidebar:
  label: chembl
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 names this source `chembl`. BioMCP reads ChEMBL drug targets and indications, and ChEMBL cell metadata and assay counts.

| Field | Value |
| --- | --- |
| Source name | `chembl` |
| Use group | `attribution` |
| Terms | ChEMBL CC BY-SA 3.0 |
| Terms page | <https://chembl.github.io/chembl-licensing/> |
| Key | none |

## Commands

| Command | Description |
| --- | --- |
| [`biomcp search drug`](/reference/commands/search-drug/) | Literal single-source candidate discovery, without an implicit identity selection. |
| [`biomcp get drug`](/reference/commands/get-drug/) | Exact native ID retrieval with an explicitly selected source. |
| [`biomcp cell-line assay-count`](/reference/commands/cell-line-assay-count/) | A separate assay-count request, never implicit in metadata or all. |
| [`biomcp get cell-line`](/reference/commands/get-cell-line/) | One exact identity and an optional bounded named section. |

Select this source with `--source chembl` on the commands that list it.

## Turn it off

Leave this source out of one command with `--disable-sources chembl`, or add `"chembl"` to `disable_sources` in `config.json`. The group `attribution` switches off every source in that use group. See [keys and configuration](/getting-started/api-keys/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
