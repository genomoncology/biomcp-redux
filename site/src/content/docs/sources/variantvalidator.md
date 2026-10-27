---
title: VariantValidator (variantvalidator)
description: "How BioMCP 1.0.0 uses VariantValidator: commands, key, use group
  and publisher terms."
goal: Find what BioMCP reads from VariantValidator, which commands reach it, and
  its terms.
reader: A BioMCP user checking one data source
sidebar:
  label: variantvalidator
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 names this source `variantvalidator`. BioMCP sends reference-qualified HGVS to VariantValidator with a genome build.

| Field | Value |
| --- | --- |
| Source name | `variantvalidator` |
| Use group | `restricted` |
| Terms | VariantValidator service output terms require review |
| Terms page | <https://variantvalidator.org/help/about/> |
| Key | none |

## Commands

| Command | Description |
| --- | --- |
| [`biomcp variant normalize`](/reference/commands/variant-normalize/) | Send unchanged reference-qualified notation to explicitly named services. |

Select this source with `--source variantvalidator` on the commands that list it.

## Turn it off

Leave this source out of one command with `--disable-sources variantvalidator`, or add `"variantvalidator"` to `disable_sources` in `config.json`. The group `restricted` switches off every source in that use group. See [keys and configuration](/getting-started/api-keys/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
