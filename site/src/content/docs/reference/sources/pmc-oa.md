---
title: PMC Open Access (pmc-oa)
description: "How BioMCP 1.0.0 uses PMC Open Access: commands, key, use group
  and publisher terms."
goal: Find what BioMCP reads from PMC Open Access, which commands reach it, and
  its terms.
reader: A BioMCP user checking one data source
sidebar:
  label: pmc-oa
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 names this source `pmc-oa`. BioMCP reads licensed full text, supplements and PDFs from the PMC Open Access subset.

| Field | Value |
| --- | --- |
| Source name | `pmc-oa` |
| Use group | `restricted` |
| Terms | PMC copyright; article-specific licenses |
| Terms page | <https://pmc.ncbi.nlm.nih.gov/about/copyright/> |
| Key | none |

## Commands

| Command | Description |
| --- | --- |
| [`biomcp get article`](/reference/commands/get-article/) | Get one to twenty article inputs in caller order. |
| [`biomcp article text`](/reference/commands/article-text/) | Read one bounded body part, find matching rows, or save the complete Markdown document. |
| [`biomcp article supplement`](/reference/commands/article-supplement/) | Read a checked supplement file or page its original rows. |
| [`biomcp article pdf`](/reference/commands/article-pdf/) | Download one publisher PDF to an explicit file or existing folder. |

## Turn it off

Leave this source out of one command with `--disable-sources pmc-oa`, or add `"pmc-oa"` to `disable_sources` in `config.json`. The group `restricted` switches off every source in that use group. See [keys and configuration](/keys-and-configuration/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
