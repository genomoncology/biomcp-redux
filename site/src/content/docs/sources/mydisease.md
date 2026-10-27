---
title: MyDisease.info (mydisease)
description: "How BioMCP 1.0.0 uses MyDisease.info: commands, key, use group and
  publisher terms."
goal: Find what BioMCP reads from MyDisease.info, which commands reach it, and
  its terms.
reader: A BioMCP user checking one data source
sidebar:
  label: mydisease
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 names this source `mydisease`. BioMCP reads disease records through MyDisease.info and resolves trial condition identifiers.

| Field | Value |
| --- | --- |
| Source name | `mydisease` |
| Use group | `attribution` |
| Terms | MyDisease source-specific terms |
| Terms page | <https://docs.mydisease.info/en/latest/doc/data.html> |
| Key | none |

## Commands

| Command | Description |
| --- | --- |
| [`biomcp search disease`](/reference/commands/search-disease/) | Literal discovery text or an explicit SEER catalog page. |
| [`biomcp get disease`](/reference/commands/get-disease/) | Exact concept or native SEER site with independently paged section. |
| [`biomcp search trial`](/reference/commands/search-trial/) | Search one provider with bounded pages and source-native counts; default ClinicalTrials.gov, explicit keyed NCI, never a fallback. |

Select this source with `--source mydisease` on the commands that list it.

## Turn it off

Leave this source out of one command with `--disable-sources mydisease`, or add `"mydisease"` to `disable_sources` in `config.json`. The group `attribution` switches off every source in that use group. See [keys and configuration](/getting-started/api-keys/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
