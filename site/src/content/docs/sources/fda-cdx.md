---
title: FDA companion diagnostics list (fda-cdx)
description: "How BioMCP 1.0.0 uses FDA companion diagnostics list: commands,
  key, use group and publisher terms."
goal: Find what BioMCP reads from FDA companion diagnostics list, which commands
  reach it, and its terms.
reader: A BioMCP user checking one data source
sidebar:
  label: fda-cdx
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 names this source `fda-cdx`. BioMCP reads the FDA list of cleared or approved companion diagnostic devices from a local bundle.

| Field | Value |
| --- | --- |
| Source name | `fda-cdx` |
| Use group | `open` |
| Terms | FDA website policies; third-party exceptions retained |
| Terms page | <https://www.fda.gov/about-fda/about-website/website-policies> |
| Key | none |
| Local bundle | `biomcp sync fda-cdx` |

## Commands

| Command | Description |
| --- | --- |
| [`biomcp search diagnostic`](/reference/commands/search-diagnostic/) | Search one diagnostic source. |

Select this source with `--source fda-cdx` on the commands that list it.

## Turn it off

Leave this source out of one command with `--disable-sources fda-cdx`, or add `"fda-cdx"` to `disable_sources` in `config.json`. The group `open` switches off every source in that use group. See [keys and configuration](/getting-started/api-keys/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
