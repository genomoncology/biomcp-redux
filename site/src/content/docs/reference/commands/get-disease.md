---
title: biomcp get disease
description: "The BioMCP 1.0.0 command biomcp get disease: Reads one disease
  concept or SEER cancer site, and optionally a section such as its genes or
  incidence."
goal: Look up the arguments, sections and examples of biomcp get disease.
reader: A BioMCP user or agent checking one command
sidebar:
  label: get disease
kind: reference
verified: 2026-10-28
---

`biomcp get disease` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It reads one disease concept or SEER cancer site, and optionally a section such as its genes or incidence. A [section word](/concepts/progressive-disclosure/) after the identifier adds one more part of the record to the reply. This page lists its arguments, sections, examples and reply.

The program's help describes it:

> Exact concept or native SEER site with independently paged section.

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

`--source` picks the source to ask: [ols](/sources/ols/), [mydisease](/sources/mydisease/), [monarch](/sources/monarch-initiative/), [seer](/sources/seer/).

## Examples

The program's help gives these examples.

```sh
biomcp get disease 47 incidence --source seer --dataset 4 --period 5 \
  --section-limit 1
```

```sh
biomcp get disease 55 survival --source seer --sex 3 --race 1 --age-range 1 \
  --dataset 4 --interval 5 --section-limit 5
```

## Reply

The reply is one JSON object. Its schema is one of [`biomcp.disease_incidence/3`](/reference/schemas/biomcp-disease-incidence-3/), [`biomcp.disease_survival/3`](/reference/schemas/biomcp-disease-survival-3/), whose page shows a real reply and its fields.

## Next

The [disease guide](/user-guide/disease/) shows every disease command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
