---
title: OncoKB (oncokb)
description: "How BioMCP 1.0.0 uses OncoKB: commands, key, use group and publisher terms."
goal: Find what BioMCP reads from OncoKB, which commands reach it, and its terms.
reader: A BioMCP user checking one data source
sidebar:
  label: oncokb
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 names this source `oncokb`. BioMCP reads OncoKB gene and variant annotations with your OncoKB key.

| Field | Value |
| --- | --- |
| Source name | `oncokb` |
| Use group | `restricted` |
| Terms | OncoKB license |
| Terms page | <https://faq.oncokb.org/licensing> |
| Key | optional or required key: `biomcp key set oncokb`, environment `ONCOKB_TOKEN`, configuration `keys.oncokb_api_key`. See [`help key oncokb`](/reference/commands/help-key-oncokb/). |

## Commands

| Command | Description |
| --- | --- |
| [`biomcp get gene`](/reference/commands/get-gene/) | Get one to twenty human gene inputs in their original order. |
| [`biomcp get variant`](/reference/commands/get-variant/) | Read an rsID, CAid, ClinVar Variation ID or reference-qualified HGVS; chromosome notation requires --build. |

## Turn it off

Leave this source out of one command with `--disable-sources oncokb`, or add `"oncokb"` to `disable_sources` in `config.json`. The group `restricted` switches off every source in that use group. See [keys and configuration](/getting-started/api-keys/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
