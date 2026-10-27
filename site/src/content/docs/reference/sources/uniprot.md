---
title: UniProt (uniprot)
description: "How BioMCP 1.0.0 uses UniProt: commands, key, use group and publisher terms."
goal: Find what BioMCP reads from UniProt, which commands reach it, and its terms.
reader: A BioMCP user checking one data source
sidebar:
  label: uniprot
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 names this source `uniprot`. BioMCP reads reviewed human protein records, features, isoforms, function text and sequences from UniProt.

| Field | Value |
| --- | --- |
| Source name | `uniprot` |
| Use group | `attribution` |
| Terms | CC BY 4.0 |
| Terms page | <https://www.uniprot.org/help/license> |
| Key | none |

## Commands

| Command | Description |
| --- | --- |
| [`biomcp search protein`](/reference/commands/search-protein/) | Search five reviewed human UniProt headers by literal text or exact gene. |
| [`biomcp get protein`](/reference/commands/get-protein/) | Read source protein cards; a symbol returns candidates without choosing a protein. |
| [`biomcp protein features`](/reference/commands/protein-features/) | Page five native UniProt features by exact type and displayed residue. |
| [`biomcp protein isoforms`](/reference/commands/protein-isoforms/) | Page five source isoforms, exact-participant complexes or PDB links. |
| [`biomcp protein sequence`](/reference/commands/protein-sequence/) | Read one complete native sequence for an exact source accession. |
| [`biomcp protein structures`](/reference/commands/protein-structures/) | Page five source isoforms, exact-participant complexes or PDB links. |
| [`biomcp get gene`](/reference/commands/get-gene/) | Get one to twenty human gene inputs in their original order. |

## Turn it off

Leave this source out of one command with `--disable-sources uniprot`, or add `"uniprot"` to `disable_sources` in `config.json`. The group `attribution` switches off every source in that use group. See [keys and configuration](/keys-and-configuration/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
