---
title: MyVariant.info (myvariant)
description: "How BioMCP 1.0.0 uses MyVariant.info: commands, key, use group and
  publisher terms."
goal: Find what BioMCP reads from MyVariant.info, which commands reach it, and
  its terms.
reader: A BioMCP user checking one data source
sidebar:
  label: myvariant
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 names this source `myvariant`. BioMCP searches variants and reads predictor, conservation and historical COSMIC fields through MyVariant.info.

| Field | Value |
| --- | --- |
| Source name | `myvariant` |
| Use group | `restricted` |
| Terms | MyVariant.info terms and component data licenses |
| Terms page | <https://myvariant.info/terms> |
| Key | none |

## Commands

| Command | Description |
| --- | --- |
| [`biomcp search variant`](/reference/commands/search-variant/) | Discover source candidates using one declared selector, without interpreting free text. |
| [`biomcp get variant`](/reference/commands/get-variant/) | Read an rsID, CAid, ClinVar Variation ID or reference-qualified HGVS; chromosome notation requires --build. |

Select this source with `--source myvariant` on the commands that list it.

## Turn it off

Leave this source out of one command with `--disable-sources myvariant`, or add `"myvariant"` to `disable_sources` in `config.json`. The group `restricted` switches off every source in that use group. See [keys and configuration](/keys-and-configuration/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
