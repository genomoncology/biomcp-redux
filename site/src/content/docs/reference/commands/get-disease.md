---
title: biomcp get disease
description: "Reference for the BioMCP 1.0.0 command biomcp get disease: Exact
  concept or native SEER site with independently paged section."
goal: Look up the arguments, sections and examples of biomcp get disease.
reader: A BioMCP user or agent checking one command
sidebar:
  label: get disease
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 command `biomcp get disease`.

Exact concept or native SEER site with independently paged section.

## Usage

```text
biomcp get disease [OPTIONS] <id> [section]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--age-range` | value | no |  |  | Native age-range code. |
| `--dataset` | value | no |  |  | Requested native dataset selector, not proof of returned membership. |
| `--interval` | value | no |  |  | Survival-only interval code. |
| `--ontology` | string | no |  | `mondo`, `doid`, `ncit` | One OLS ontology. |
| `--period` | value | no |  |  | Incidence-only native period code. |
| `--race` | value | no |  |  | Native race code. |
| `--section-limit` | integer | no | min 1, max 50 |  | Native rows per page: default 2 for Monarch genes, 1 for Monarch phenotypes, otherwise 5. |
| `--section-offset` | integer | no | min 0 |  | Native row offset within the selected section. |
| `--sex` | value | no |  |  | Native sex code. |
| `--source` | string | no |  | `ols`, `mydisease`, `monarch`, `seer` | Default MyDisease; no automatic fallback. |
| `id` (position 1) | string | yes |  |  | Exact ontology CURIE or an explicit SEER catalog site ID. |
| `section` (position 2) | value | no |  |  | One named section; absent means identity card only. |

## Sections

One named section; absent means identity card only.

| Section | Description |
| --- | --- |
| `synonyms` | Publisher scoped synonyms. |
| `xrefs` | Native cross references. |
| `hierarchy` | Native parent relations. |
| `genes` | Native gene association rows. |
| `phenotypes` | Native phenotype association rows. |
| `incidence` | Native stage incidence rows. |
| `survival` | Annual survival rows. |

## Sources

`--source` accepts these sources: [ols](/reference/sources/ols/), [mydisease](/reference/sources/mydisease/), [monarch](/reference/sources/monarch/), [seer](/reference/sources/seer/).

## Examples

```sh
biomcp get disease 47 incidence --source seer --dataset 4 --period 5 --section-limit 1
biomcp get disease 55 survival --source seer --sex 3 --race 1 --age-range 1 --dataset 4 --interval 5 --section-limit 5
```

## Exit codes

See [exit codes](/reference/exit-codes/). Options that apply to every command are in [global options](/reference/global-options/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
