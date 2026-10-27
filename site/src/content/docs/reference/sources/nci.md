---
title: NCI Clinical Trials Search (nci)
description: "How BioMCP 1.0.0 uses NCI Clinical Trials Search: commands, key,
  use group and publisher terms."
goal: Find what BioMCP reads from NCI Clinical Trials Search, which commands
  reach it, and its terms.
reader: A BioMCP user checking one data source
sidebar:
  label: nci
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 names this source `nci`. BioMCP searches the NCI Clinical Trials Search API and reads NCI trial records with your NCI key.

| Field | Value |
| --- | --- |
| Source name | `nci` |
| Use group | `restricted` |
| Terms | NCI Clinical Trials Search publisher terms; user's API key required |
| Terms page | <https://www.cancer.gov/syndication/api> |
| Key | optional or required key: `biomcp key set nci`, environment `NCI_API_KEY`, configuration `keys.nci_api_key`. See [`help key nci`](/reference/commands/help-key-nci/). |

## Commands

| Command | Description |
| --- | --- |
| [`biomcp search trial`](/reference/commands/search-trial/) | Search one provider with bounded pages and source-native counts; default ClinicalTrials.gov, explicit keyed NCI, never a fallback. |
| [`biomcp get trial`](/reference/commands/get-trial/) | Retrieve exact trial cards in input order; request bounded intact source sections with independent continuations. |

Select this source with `--source nci` on the commands that list it.

## Turn it off

Leave this source out of one command with `--disable-sources nci`, or add `"nci"` to `disable_sources` in `config.json`. The group `restricted` switches off every source in that use group. See [keys and configuration](/keys-and-configuration/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
