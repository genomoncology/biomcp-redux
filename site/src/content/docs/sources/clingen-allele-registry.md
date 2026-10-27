---
title: ClinGen Allele Registry (car)
description: "How BioMCP 1.0.0 uses ClinGen Allele Registry: commands, key, use
  group and publisher terms."
goal: Find what BioMCP reads from ClinGen Allele Registry, which commands reach
  it, and its terms.
reader: A BioMCP user checking one data source
sidebar:
  label: car
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 names this source `car`. BioMCP resolves variant identities and normalizes HGVS through the ClinGen Allele Registry.

| Field | Value |
| --- | --- |
| Source name | `car` |
| Use group | `restricted` |
| Terms | ClinGen Allele Registry service terms; output rights require review |
| Terms page | <https://clinicalgenome.org/about/terms-of-use/> |
| Key | none |

## Commands

| Command | Description |
| --- | --- |
| [`biomcp variant normalize`](/reference/commands/variant-normalize/) | Send unchanged reference-qualified notation to explicitly named services. |
| [`biomcp get variant`](/reference/commands/get-variant/) | Read an rsID, CAid, ClinVar Variation ID or reference-qualified HGVS; chromosome notation requires --build. |

Select this source with `--source car` on the commands that list it.

## Turn it off

Leave this source out of one command with `--disable-sources car`, or add `"car"` to `disable_sources` in `config.json`. The group `restricted` switches off every source in that use group. See [keys and configuration](/getting-started/api-keys/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
