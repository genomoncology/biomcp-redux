---
title: Monarch Initiative (monarch)
description: "How BioMCP 1.0.0 uses Monarch Initiative: commands, key, use group
  and publisher terms."
goal: Find what BioMCP reads from Monarch Initiative, which commands reach it,
  and its terms.
reader: A BioMCP user checking one data source
sidebar:
  label: monarch
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 names this source `monarch`. BioMCP reads disease and phenotype associations with genes from the Monarch Initiative.

| Field | Value |
| --- | --- |
| Source name | `monarch` |
| Use group | `attribution` |
| Terms | Monarch source-specific licensing |
| Terms page | <https://monarch-app.monarchinitiative.org/Licensing/> |
| Key | none |

## Commands

| Command | Description |
| --- | --- |
| [`biomcp search phenotype`](/reference/commands/search-phenotype/) | Search literal human HP term candidates through Monarch with bounded native pages. |
| [`biomcp get phenotype`](/reference/commands/get-phenotype/) | Read an exact HP term from HPO or Monarch, or page native source sections. |
| [`biomcp search disease`](/reference/commands/search-disease/) | Literal discovery text or an explicit SEER catalog page. |
| [`biomcp get disease`](/reference/commands/get-disease/) | Exact concept or native SEER site with independently paged section. |

Select this source with `--source monarch` on the commands that list it.

## Turn it off

Leave this source out of one command with `--disable-sources monarch`, or add `"monarch"` to `disable_sources` in `config.json`. The group `attribution` switches off every source in that use group. See [keys and configuration](/keys-and-configuration/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
