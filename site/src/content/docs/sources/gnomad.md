---
title: gnomAD (gnomad)
description: "How BioMCP 1.0.0 uses gnomAD: commands, key, use group and publisher terms."
goal: Find what BioMCP reads from gnomAD, which commands reach it, and its terms.
reader: A BioMCP user checking one data source
sidebar:
  label: gnomad
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 names this source `gnomad`. BioMCP reads gnomAD population counts for a variant and population constraint for a gene.

| Field | Value |
| --- | --- |
| Source name | `gnomad` |
| Use group | `open` |
| Terms | CC0 1.0 |
| Terms page | <https://gnomad.broadinstitute.org/policies> |
| Key | none |

## Commands

| Command | Description |
| --- | --- |
| [`biomcp variant population`](/reference/commands/variant-population/) | Read a bounded ancestry-detail page from the exact source-given GRCh38 allele. |
| [`biomcp get variant`](/reference/commands/get-variant/) | Read an rsID, CAid, ClinVar Variation ID or reference-qualified HGVS; chromosome notation requires --build. |
| [`biomcp get gene`](/reference/commands/get-gene/) | Get one to twenty human gene inputs in their original order. |

## Turn it off

Leave this source out of one command with `--disable-sources gnomad`, or add `"gnomad"` to `disable_sources` in `config.json`. The group `open` switches off every source in that use group. See [keys and configuration](/getting-started/api-keys/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
