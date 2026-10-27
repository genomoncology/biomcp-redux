---
title: ORCID (orcid)
description: "How BioMCP 1.0.0 uses ORCID: commands, key, use group and publisher terms."
goal: Find what BioMCP reads from ORCID, which commands reach it, and its terms.
reader: A BioMCP user checking one data source
sidebar:
  label: orcid
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 names this source `orcid`. BioMCP reads one ORCID record and its public works.

| Field | Value |
| --- | --- |
| Source name | `orcid` |
| Use group | `noncommercial` |
| Terms | ORCID Public API Terms of Service |
| Terms page | <https://info.orcid.org/public-client-terms-of-service/> |
| Key | optional or required key: `biomcp key set orcid`, environment `ORCID_ACCESS_TOKEN`, configuration `keys.orcid_access_token`. See [`help key orcid`](/reference/commands/help-key-orcid/). |

## Commands

| Command | Description |
| --- | --- |
| [`biomcp get author`](/reference/commands/get-author/) | Exact qualified records in caller order, with no name reconciliation. |
| [`biomcp author papers`](/reference/commands/author-papers/) | One exact provider record's publication window. |

## Turn it off

Leave this source out of one command with `--disable-sources orcid`, or add `"orcid"` to `disable_sources` in `config.json`. The group `noncommercial` switches off every source in that use group. See [keys and configuration](/keys-and-configuration/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
