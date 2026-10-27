---
title: ClinVar (clinvar)
description: "How BioMCP 1.0.0 uses ClinVar: commands, key, use group and publisher terms."
goal: Find what BioMCP reads from ClinVar, which commands reach it, and its terms.
reader: A BioMCP user checking one data source
sidebar:
  label: clinvar
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 names this source `clinvar`. BioMCP reads ClinVar assertions for a variant.

| Field | Value |
| --- | --- |
| Source name | `clinvar` |
| Use group | `restricted` |
| Terms | NCBI policy; submission rights remain source-specific |
| Terms page | <https://www.ncbi.nlm.nih.gov/clinvar/docs/maintenance_use/> |
| Key | none |

## Commands

| Command | Description |
| --- | --- |
| [`biomcp variant assertions`](/reference/commands/variant-assertions/) | Page assertions from one explicitly selected evidence source. |
| [`biomcp get variant`](/reference/commands/get-variant/) | Read an rsID, CAid, ClinVar Variation ID or reference-qualified HGVS; chromosome notation requires --build. |

Select this source with `--source clinvar` on the commands that list it.

## Turn it off

Leave this source out of one command with `--disable-sources clinvar`, or add `"clinvar"` to `disable_sources` in `config.json`. The group `restricted` switches off every source in that use group. See [keys and configuration](/getting-started/api-keys/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
