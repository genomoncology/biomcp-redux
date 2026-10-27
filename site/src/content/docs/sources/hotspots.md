---
title: Cancer Hotspots (hotspots)
description: "How BioMCP 1.0.0 uses Cancer Hotspots: commands, key, use group
  and publisher terms."
goal: Find what BioMCP reads from Cancer Hotspots, which commands reach it, and
  its terms.
reader: A BioMCP user checking one data source
sidebar:
  label: hotspots
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 names this source `hotspots`. BioMCP reads Cancer Hotspots residue counts for a variant's position.

| Field | Value |
| --- | --- |
| Source name | `hotspots` |
| Use group | `restricted` |
| Terms | Publisher-specific terms; data reuse unverified |
| Terms page | <https://www.cancerhotspots.org/> |
| Key | none |

## Commands

| Command | Description |
| --- | --- |
| [`biomcp get variant`](/reference/commands/get-variant/) | Read an rsID, CAid, ClinVar Variation ID or reference-qualified HGVS; chromosome notation requires --build. |

## Turn it off

Leave this source out of one command with `--disable-sources hotspots`, or add `"hotspots"` to `disable_sources` in `config.json`. The group `restricted` switches off every source in that use group. See [keys and configuration](/getting-started/api-keys/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
