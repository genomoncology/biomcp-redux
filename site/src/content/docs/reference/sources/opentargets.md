---
title: Open Targets Platform (opentargets)
description: "How BioMCP 1.0.0 uses Open Targets Platform: commands, key, use
  group and publisher terms."
goal: Find what BioMCP reads from Open Targets Platform, which commands reach
  it, and its terms.
reader: A BioMCP user checking one data source
sidebar:
  label: opentargets
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 names this source `opentargets`. BioMCP reads Open Targets drug records, targets and indications.

| Field | Value |
| --- | --- |
| Source name | `opentargets` |
| Use group | `open` |
| Terms | Open Targets CC0; underlying source provenance retained |
| Terms page | <https://platform-docs.opentargets.org/licence> |
| Key | none |

## Commands

| Command | Description |
| --- | --- |
| [`biomcp search drug`](/reference/commands/search-drug/) | Literal single-source candidate discovery, without an implicit identity selection. |
| [`biomcp get drug`](/reference/commands/get-drug/) | Exact native ID retrieval with an explicitly selected source. |

Select this source with `--source opentargets` on the commands that list it.

## Turn it off

Leave this source out of one command with `--disable-sources opentargets`, or add `"opentargets"` to `disable_sources` in `config.json`. The group `open` switches off every source in that use group. See [keys and configuration](/keys-and-configuration/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
