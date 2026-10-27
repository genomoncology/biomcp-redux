---
title: Complex Portal (complex-portal)
description: "How BioMCP 1.0.0 uses Complex Portal: commands, key, use group and
  publisher terms."
goal: Find what BioMCP reads from Complex Portal, which commands reach it, and
  its terms.
reader: A BioMCP user checking one data source
sidebar:
  label: complex-portal
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 names this source `complex-portal`. BioMCP reads the protein complexes a protein takes part in and their members.

| Field | Value |
| --- | --- |
| Source name | `complex-portal` |
| Use group | `open` |
| Terms | CC0 |
| Terms page | <https://pmc.ncbi.nlm.nih.gov/articles/PMC11701666/> |
| Key | none |

## Commands

| Command | Description |
| --- | --- |
| [`biomcp protein complexes`](/reference/commands/protein-complexes/) | Page five source isoforms, exact-participant complexes or PDB links. |
| [`biomcp protein complex-members`](/reference/commands/protein-complex-members/) | Page five native participants of an exact Complex Portal complex containing the anchor. |
| [`biomcp get protein`](/reference/commands/get-protein/) | Read source protein cards; a symbol returns candidates without choosing a protein. |

## Turn it off

Leave this source out of one command with `--disable-sources complex-portal`, or add `"complex-portal"` to `disable_sources` in `config.json`. The group `open` switches off every source in that use group. See [keys and configuration](/getting-started/api-keys/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
