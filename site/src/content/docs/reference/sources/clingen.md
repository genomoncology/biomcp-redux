---
title: ClinGen gene validity and dosage (clingen)
description: "How BioMCP 1.0.0 uses ClinGen gene validity and dosage: commands,
  key, use group and publisher terms."
goal: Find what BioMCP reads from ClinGen gene validity and dosage, which
  commands reach it, and its terms.
reader: A BioMCP user checking one data source
sidebar:
  label: clingen
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 names this source `clingen`. BioMCP reads ClinGen gene-disease validity and dosage curation from a local bundle that it can synchronize when a read needs it.

| Field | Value |
| --- | --- |
| Source name | `clingen` |
| Use group | `open` |
| Terms | CC0 1.0 |
| Terms page | <https://clinicalgenome.org/docs/terms-of-use/> |
| Key | none |
| Local bundle | `biomcp sync clingen` |

## Commands

| Command | Description |
| --- | --- |
| [`biomcp get gene`](/reference/commands/get-gene/) | Get one to twenty human gene inputs in their original order. |

## Turn it off

Leave this source out of one command with `--disable-sources clingen`, or add `"clingen"` to `disable_sources` in `config.json`. The group `open` switches off every source in that use group. See [keys and configuration](/keys-and-configuration/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
