---
title: FDA Adverse Event Reporting System (faers)
description: "How BioMCP 1.0.0 uses FDA Adverse Event Reporting System:
  commands, key, use group and publisher terms."
goal: Find what BioMCP reads from FDA Adverse Event Reporting System, which
  commands reach it, and its terms.
reader: A BioMCP user checking one data source
sidebar:
  label: faers
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 names this source `faers`. BioMCP searches and reads FAERS drug adverse event reports through openFDA. It is the default adverse event source.

| Field | Value |
| --- | --- |
| Source name | `faers` |
| Use group | `open` |
| Terms | openFDA terms and license; third-party exceptions |
| Terms page | <https://open.fda.gov/terms/> |
| Key | none |

## Commands

| Command | Description |
| --- | --- |
| [`biomcp search adverse-event`](/reference/commands/search-adverse-event/) | Search five report headers by default. |
| [`biomcp get adverse-event`](/reference/commands/get-adverse-event/) | Exact native report identity and an optional independently paged section. |

Select this source with `--source faers` on the commands that list it.

## Turn it off

Leave this source out of one command with `--disable-sources faers`, or add `"faers"` to `disable_sources` in `config.json`. The group `open` switches off every source in that use group. See [keys and configuration](/getting-started/api-keys/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
