---
title: Human Protein Atlas (hpa)
description: "How BioMCP 1.0.0 uses Human Protein Atlas: commands, key, use
  group and publisher terms."
goal: Find what BioMCP reads from Human Protein Atlas, which commands reach it,
  and its terms.
reader: A BioMCP user checking one data source
sidebar:
  label: hpa
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 names this source `hpa`. BioMCP reads tissue expression for a gene from the Human Protein Atlas.

| Field | Value |
| --- | --- |
| Source name | `hpa` |
| Use group | `attribution` |
| Terms | CC BY 4.0; third-party conditions |
| Terms page | <https://www.proteinatlas.org/about/licence> |
| Key | none |

## Commands

| Command | Description |
| --- | --- |
| [`biomcp get gene`](/reference/commands/get-gene/) | Get one to twenty human gene inputs in their original order. |

## Turn it off

Leave this source out of one command with `--disable-sources hpa`, or add `"hpa"` to `disable_sources` in `config.json`. The group `attribution` switches off every source in that use group. See [keys and configuration](/keys-and-configuration/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
