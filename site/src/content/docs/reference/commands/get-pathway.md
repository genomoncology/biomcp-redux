---
title: biomcp get pathway
description: "Reference for the BioMCP 1.0.0 command biomcp get pathway: Get an
  exact human Reactome R-HSA or WikiPathways WP card."
goal: Look up the arguments, sections and examples of biomcp get pathway.
reader: A BioMCP user or agent checking one command
sidebar:
  label: get pathway
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 command `biomcp get pathway`.

Get an exact human Reactome R-HSA or WikiPathways WP card. Read whole summary, genes, Reactome events or bounded all sections. Membership names a source and release; it implies no therapy, response or actionability. No enrichment is calculated.

## Usage

```text
biomcp get pathway [OPTIONS] <id> [section]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--fresh` | boolean | no |  |  | Request a new source observation. |
| `--pool` | string | no |  |  | Retained snapshot, when continuing a source relation. |
| `--section-limit` | integer | no | min 1, max 50 |  | Bounded initial section size; each all section has its own first page. |
| `--section-offset` | integer | no | min 0 |  | Native relation offset for a selected genes or events section. |
| `id` (position 1) | string | yes |  |  | Exact human Reactome or WikiPathways identity without version suffixes. |
| `section` (position 2) | string | no |  | `summary`, `genes`, `events`, `all` | Omit for the card; WikiPathways has summary and genes sections. |

## Sections

Omit for the card; WikiPathways has summary and genes sections.

| Section | Description |
| --- | --- |
| `summary` | Complete native source description. |
| `genes` | Namespaced native participants with exact resolved gene joins. |
| `events` | Native Reactome event objects and unresolved database references. |
| `all` | Each source-supported section's bounded first page. |

## Examples

```sh
biomcp get pathway R-HSA-9652817
biomcp get pathway R-HSA-177929
```

## Reply schema

The help examples of this command return [`biomcp.pathway_card/5`](/reference/schemas/biomcp-pathway-card-5/). Print the full JSON schema with `biomcp schema biomcp.pathway_card/5`.

## Exit codes

See [exit codes](/reference/error-codes/). Options that apply to every command are in [global options](/reference/global-options/).

## Next

The [pathway entity](/user-guide/pathway/) page lists every pathway command and links to a worked guide.

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
