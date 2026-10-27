---
title: Human Phenotype Ontology (hpo)
description: "How BioMCP 1.0.0 uses Human Phenotype Ontology: commands, key, use
  group and publisher terms."
goal: Find what BioMCP reads from Human Phenotype Ontology, which commands reach
  it, and its terms.
reader: A BioMCP user checking one data source
sidebar:
  label: hpo
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 names this source `hpo`. BioMCP searches and reads HPO phenotype terms.

| Field | Value |
| --- | --- |
| Source name | `hpo` |
| Use group | `attribution` |
| Terms | HPO project custom license: attribution, version and integrity |
| Terms page | <https://human-phenotype-ontology.github.io/license.html> |
| Key | none |

## Commands

| Command | Description |
| --- | --- |
| [`biomcp search phenotype`](/reference/commands/search-phenotype/) | Search literal human HP term candidates through Monarch with bounded native pages. |
| [`biomcp get phenotype`](/reference/commands/get-phenotype/) | Read an exact HP term from HPO or Monarch, or page native source sections. |

Select this source with `--source hpo` on the commands that list it.

## Turn it off

Leave this source out of one command with `--disable-sources hpo`, or add `"hpo"` to `disable_sources` in `config.json`. The group `attribution` switches off every source in that use group. See [keys and configuration](/keys-and-configuration/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
