---
title: ClinicalTrials.gov (ctgov)
description: "How BioMCP 1.0.0 uses ClinicalTrials.gov: commands, key, use group
  and publisher terms."
goal: Find what BioMCP reads from ClinicalTrials.gov, which commands reach it,
  and its terms.
reader: A BioMCP user checking one data source
sidebar:
  label: ctgov
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 names this source `ctgov`. BioMCP searches ClinicalTrials.gov and reads trial records, sections and posted documents. It is the default trial source.

| Field | Value |
| --- | --- |
| Source name | `ctgov` |
| Use group | `restricted` |
| Terms | ClinicalTrials.gov attribution and source processing date; publisher terms |
| Terms page | <https://clinicaltrials.gov/about-site/terms-conditions> |
| Key | none |

## Commands

| Command | Description |
| --- | --- |
| [`biomcp search trial`](/reference/commands/search-trial/) | Search one provider with bounded pages and source-native counts; default ClinicalTrials.gov, explicit keyed NCI, never a fallback. |
| [`biomcp get trial`](/reference/commands/get-trial/) | Retrieve exact trial cards in input order; request bounded intact source sections with independent continuations. |
| [`biomcp trial document`](/reference/commands/trial-document/) | Download one exact ClinicalTrials.gov manifest PDF to an explicit caller path; never read its text. |

Select this source with `--source ctgov` on the commands that list it.

## Turn it off

Leave this source out of one command with `--disable-sources ctgov`, or add `"ctgov"` to `disable_sources` in `config.json`. The group `restricted` switches off every source in that use group. See [keys and configuration](/getting-started/api-keys/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
