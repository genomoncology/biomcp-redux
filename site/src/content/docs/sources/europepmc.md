---
title: Europe PMC (europepmc)
description: "How BioMCP 1.0.0 uses Europe PMC: commands, key, use group and
  publisher terms."
goal: Find what BioMCP reads from Europe PMC, which commands reach it, and its terms.
reader: A BioMCP user checking one data source
sidebar:
  label: europepmc
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 names this source `europepmc`. BioMCP searches Europe PMC, including open full text and preprints, and resolves article identifiers.

| Field | Value |
| --- | --- |
| Source name | `europepmc` |
| Use group | `restricted` |
| Terms | Europe PMC copyright; record-specific licenses |
| Terms page | <https://europepmc.org/help#copyright> |
| Key | none |

## Commands

| Command | Description |
| --- | --- |
| [`biomcp search article`](/reference/commands/search-article/) | Checked search selectors and pool cursor. |
| [`biomcp get article`](/reference/commands/get-article/) | Get one to twenty article inputs in caller order. |

Select this source with `--source europepmc` on the commands that list it.

## Turn it off

Leave this source out of one command with `--disable-sources europepmc`, or add `"europepmc"` to `disable_sources` in `config.json`. The group `restricted` switches off every source in that use group. See [keys and configuration](/getting-started/api-keys/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
