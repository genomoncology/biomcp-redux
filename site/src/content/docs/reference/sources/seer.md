---
title: SEER (seer)
description: "How BioMCP 1.0.0 uses SEER: commands, key, use group and publisher terms."
goal: Find what BioMCP reads from SEER, which commands reach it, and its terms.
reader: A BioMCP user checking one data source
sidebar:
  label: seer
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 names this source `seer`. BioMCP reads SEER cancer sites, stage incidence and survival rows.

| Field | Value |
| --- | --- |
| Source name | `seer` |
| Use group | `open` |
| Terms | SEER/NCI Using This Website |
| Terms page | <https://seer.cancer.gov/about/using-website.html> |
| Key | none |

## Commands

| Command | Description |
| --- | --- |
| [`biomcp search disease`](/reference/commands/search-disease/) | Literal discovery text or an explicit SEER catalog page. |
| [`biomcp get disease`](/reference/commands/get-disease/) | Exact concept or native SEER site with independently paged section. |

Select this source with `--source seer` on the commands that list it.

## Turn it off

Leave this source out of one command with `--disable-sources seer`, or add `"seer"` to `disable_sources` in `config.json`. The group `open` switches off every source in that use group. See [keys and configuration](/keys-and-configuration/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
