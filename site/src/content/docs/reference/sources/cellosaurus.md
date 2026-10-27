---
title: Cellosaurus (cellosaurus)
description: "How BioMCP 1.0.0 uses Cellosaurus: commands, key, use group and
  publisher terms."
goal: Find what BioMCP reads from Cellosaurus, which commands reach it, and its
  terms.
reader: A BioMCP user checking one data source
sidebar:
  label: cellosaurus
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 names this source `cellosaurus`. BioMCP reads cell-line cards, names, identity, cross-references and variants from Cellosaurus.

| Field | Value |
| --- | --- |
| Source name | `cellosaurus` |
| Use group | `open` |
| Terms | Cellosaurus CC BY 4.0; cite Bairoch (2018) |
| Terms page | <https://www.cellosaurus.org/description.html> |
| Key | none |

## Commands

| Command | Description |
| --- | --- |
| [`biomcp search cell-line`](/reference/commands/search-cell-line/) | One literal cell line search, with native order and no total claim. |
| [`biomcp get cell-line`](/reference/commands/get-cell-line/) | One exact identity and an optional bounded named section. |

## Turn it off

Leave this source out of one command with `--disable-sources cellosaurus`, or add `"cellosaurus"` to `disable_sources` in `config.json`. The group `open` switches off every source in that use group. See [keys and configuration](/keys-and-configuration/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
