---
title: Mutalyzer (mutalyzer)
description: "How BioMCP 1.0.0 uses Mutalyzer: commands, key, use group and
  publisher terms."
goal: Find what BioMCP reads from Mutalyzer, which commands reach it, and its terms.
reader: A BioMCP user checking one data source
sidebar:
  label: mutalyzer
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 names this source `mutalyzer`. BioMCP sends reference-qualified HGVS to Mutalyzer for normalization.

| Field | Value |
| --- | --- |
| Source name | `mutalyzer` |
| Use group | `restricted` |
| Terms | Mutalyzer service output terms require review |
| Terms page | <https://mutalyzer.nl/> |
| Key | none |

## Commands

| Command | Description |
| --- | --- |
| [`biomcp variant normalize`](/reference/commands/variant-normalize/) | Send unchanged reference-qualified notation to explicitly named services. |

Select this source with `--source mutalyzer` on the commands that list it.

## Turn it off

Leave this source out of one command with `--disable-sources mutalyzer`, or add `"mutalyzer"` to `disable_sources` in `config.json`. The group `restricted` switches off every source in that use group. See [keys and configuration](/getting-started/api-keys/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
