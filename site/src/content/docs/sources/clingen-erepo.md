---
title: ClinGen Evidence Repository (erepo)
description: "How BioMCP 1.0.0 uses ClinGen Evidence Repository: commands, key,
  use group and publisher terms."
goal: Find what BioMCP reads from ClinGen Evidence Repository, which commands
  reach it, and its terms.
reader: A BioMCP user checking one data source
sidebar:
  label: erepo
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 names this source `erepo`. BioMCP reads ClinGen expert-panel variant assertions.

| Field | Value |
| --- | --- |
| Source name | `erepo` |
| Use group | `restricted` |
| Terms | ClinGen sharing policy; record rights unverified |
| Terms page | <https://clinicalgenome.org/about/terms-of-use/> |
| Key | none |

## Commands

| Command | Description |
| --- | --- |
| [`biomcp variant assertions`](/reference/commands/variant-assertions/) | Page assertions from one explicitly selected evidence source. |
| [`biomcp get variant`](/reference/commands/get-variant/) | Read an rsID, CAid, ClinVar Variation ID or reference-qualified HGVS; chromosome notation requires --build. |

Select this source with `--source erepo` on the commands that list it.

## Turn it off

Leave this source out of one command with `--disable-sources erepo`, or add `"erepo"` to `disable_sources` in `config.json`. The group `restricted` switches off every source in that use group. See [keys and configuration](/getting-started/api-keys/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
