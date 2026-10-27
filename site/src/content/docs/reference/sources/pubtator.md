---
title: PubTator3 (pubtator)
description: "How BioMCP 1.0.0 uses PubTator3: commands, key, use group and
  publisher terms."
goal: Find what BioMCP reads from PubTator3, which commands reach it, and its terms.
reader: A BioMCP user checking one data source
sidebar:
  label: pubtator
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 names this source `pubtator`. BioMCP searches PubTator3 entities and reads text-mined annotations for an article.

| Field | Value |
| --- | --- |
| Source name | `pubtator` |
| Use group | `restricted` |
| Terms | NCBI policies; publisher rights in input text |
| Terms page | <https://www.ncbi.nlm.nih.gov/home/about/policies/> |
| Key | none |

## Commands

| Command | Description |
| --- | --- |
| [`biomcp search article`](/reference/commands/search-article/) | Checked search selectors and pool cursor. |
| [`biomcp get article`](/reference/commands/get-article/) | Get one to twenty article inputs in caller order. |

Select this source with `--source pubtator` on the commands that list it.

## Turn it off

Leave this source out of one command with `--disable-sources pubtator`, or add `"pubtator"` to `disable_sources` in `config.json`. The group `restricted` switches off every source in that use group. See [keys and configuration](/keys-and-configuration/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
