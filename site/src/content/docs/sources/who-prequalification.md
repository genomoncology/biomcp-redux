---
title: WHO prequalified finished products (who-pq)
description: "How BioMCP 1.0.0 uses WHO prequalified finished products:
  commands, key, use group and publisher terms."
goal: Find what BioMCP reads from WHO prequalified finished products, which
  commands reach it, and its terms.
reader: A BioMCP user checking one data source
sidebar:
  label: who-pq
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 names this source `who-pq`. BioMCP reads WHO prequalified medicines from a local copy of the WHO export.

| Field | Value |
| --- | --- |
| Source name | `who-pq` |
| Use group | `restricted` |
| Terms | WHO copyright policy; export-specific reuse grant unverified |
| Terms page | <https://www.who.int/about/policies/publishing/copyright> |
| Key | none |
| Local bundle | `biomcp sync who-pq` |

## Commands

| Command | Description |
| --- | --- |
| [`biomcp search drug`](/reference/commands/search-drug/) | Literal single-source candidate discovery, without an implicit identity selection. |
| [`biomcp get drug`](/reference/commands/get-drug/) | Exact native ID retrieval with an explicitly selected source. |

Select this source with `--source who-pq` on the commands that list it.

## Turn it off

Leave this source out of one command with `--disable-sources who-pq`, or add `"who-pq"` to `disable_sources` in `config.json`. The group `restricted` switches off every source in that use group. See [keys and configuration](/getting-started/api-keys/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
