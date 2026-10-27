---
title: ClinPGx (clinpgx)
description: "How BioMCP 1.0.0 uses ClinPGx: commands, key, use group and publisher terms."
goal: Find what BioMCP reads from ClinPGx, which commands reach it, and its terms.
reader: A BioMCP user checking one data source
sidebar:
  label: clinpgx
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 names this source `clinpgx`. BioMCP reads ClinPGx clinical, guideline and label annotations for a gene.

| Field | Value |
| --- | --- |
| Source name | `clinpgx` |
| Use group | `noncommercial` |
| Terms | ClinPGx data use policy; noncommercial access restriction; captured policy was a JavaScript shell |
| Terms page | <https://www.clinpgx.org/page/dataUsagePolicy> |
| Key | none |

## Commands

| Command | Description |
| --- | --- |
| [`biomcp get pgx`](/reference/commands/get-pgx/) | PGx card or section. |

Select this source with `--source clinpgx` on the commands that list it.

## Turn it off

Leave this source out of one command with `--disable-sources clinpgx`, or add `"clinpgx"` to `disable_sources` in `config.json`. The group `noncommercial` switches off every source in that use group. See [keys and configuration](/keys-and-configuration/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
