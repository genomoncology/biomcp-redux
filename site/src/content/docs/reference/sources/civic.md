---
title: CIViC (civic)
description: "How BioMCP 1.0.0 uses CIViC: commands, key, use group and publisher terms."
goal: Find what BioMCP reads from CIViC, which commands reach it, and its terms.
reader: A BioMCP user checking one data source
sidebar:
  label: civic
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 names this source `civic`. BioMCP reads CIViC gene curation, variant molecular-profile evidence and drug therapy evidence.

| Field | Value |
| --- | --- |
| Source name | `civic` |
| Use group | `open` |
| Terms | CC0 1.0 |
| Terms page | <https://civicdb.org/releases/licensing> |
| Key | none |

## Commands

| Command | Description |
| --- | --- |
| [`biomcp search drug`](/reference/commands/search-drug/) | Literal single-source candidate discovery, without an implicit identity selection. |
| [`biomcp get drug`](/reference/commands/get-drug/) | Exact native ID retrieval with an explicitly selected source. |
| [`biomcp get gene`](/reference/commands/get-gene/) | Get one to twenty human gene inputs in their original order. |
| [`biomcp get variant`](/reference/commands/get-variant/) | Read an rsID, CAid, ClinVar Variation ID or reference-qualified HGVS; chromosome notation requires --build. |

Select this source with `--source civic` on the commands that list it.

## Turn it off

Leave this source out of one command with `--disable-sources civic`, or add `"civic"` to `disable_sources` in `config.json`. The group `open` switches off every source in that use group. See [keys and configuration](/keys-and-configuration/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
