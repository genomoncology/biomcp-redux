---
title: cBioPortal (cbioportal)
description: "How BioMCP 1.0.0 uses cBioPortal: commands, key, use group and
  publisher terms."
goal: Find what BioMCP reads from cBioPortal, which commands reach it, and its terms.
reader: A BioMCP user checking one data source
sidebar:
  label: cbioportal
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 names this source `cbioportal`. BioMCP reads cBioPortal study metadata from a local bundle and counts samples with a variant in one study.

| Field | Value |
| --- | --- |
| Source name | `cbioportal` |
| Use group | `restricted` |
| Terms | cBioPortal study-specific terms unverified; commercial restrictions may apply |
| Terms page | <https://docs.cbioportal.org/user-guide/faq/> |
| Key | none |
| Local bundle | `biomcp sync cbioportal-studies` |

## Commands

| Command | Description |
| --- | --- |
| [`biomcp variant recurrence`](/reference/commands/variant-recurrence/) | Observe exact-allele samples within one caller-selected source page. |
| [`biomcp search study`](/reference/commands/search-study/) | Literal stored metadata search; text is never parsed as a query language. |
| [`biomcp get study`](/reference/commands/get-study/) | Exact case-sensitive study card or bounded metadata section. |

Select this source with `--source cbioportal` on the commands that list it.

## Turn it off

Leave this source out of one command with `--disable-sources cbioportal`, or add `"cbioportal"` to `disable_sources` in `config.json`. The group `restricted` switches off every source in that use group. See [keys and configuration](/keys-and-configuration/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
