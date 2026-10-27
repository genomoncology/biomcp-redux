---
title: EMBL-EBI Ontology Lookup Service (ols)
description: "How BioMCP 1.0.0 uses EMBL-EBI Ontology Lookup Service: commands,
  key, use group and publisher terms."
goal: Find what BioMCP reads from EMBL-EBI Ontology Lookup Service, which
  commands reach it, and its terms.
reader: A BioMCP user checking one data source
sidebar:
  label: ols
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 names this source `ols`. BioMCP searches MONDO, DOID and NCIt disease terms through OLS. It is the default disease source.

| Field | Value |
| --- | --- |
| Source name | `ols` |
| Use group | `attribution` |
| Terms | EMBL-EBI Terms of Use; ontology-specific rights |
| Terms page | <https://www.ebi.ac.uk/about/terms-of-use/> |
| Key | none |

## Commands

| Command | Description |
| --- | --- |
| [`biomcp search disease`](/reference/commands/search-disease/) | Literal discovery text or an explicit SEER catalog page. |
| [`biomcp get disease`](/reference/commands/get-disease/) | Exact concept or native SEER site with independently paged section. |

Select this source with `--source ols` on the commands that list it.

## Turn it off

Leave this source out of one command with `--disable-sources ols`, or add `"ols"` to `disable_sources` in `config.json`. The group `attribution` switches off every source in that use group. See [keys and configuration](/keys-and-configuration/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
