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
verified: 2026-11-09
---

`biomcp get disease` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It reads one disease concept or SEER cancer site, and optionally a section such as its genes or incidence. A [section word](/concepts/progressive-disclosure/) after the identifier adds one more part of the record to the reply. This page lists its arguments, sections, examples and reply.

The program's help describes it:

> Get disease details and source statistics. Sources: monarch, mydisease, ols, seer.

## Usage

```text
biomcp get disease [OPTIONS] <id> [section]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--age-range` | string | no |  |  | Source age-range code. Value form: string. Example: --age-range 1. Command sources: monarch, mydisease, ols, seer; source restrictions in the field description apply. |
| `--dataset` | string | no |  |  | Requested source dataset selector, not proof of returned membership. Value form: string. Example: --dataset 4. Command sources: monarch, mydisease, ols, seer; source restrictions in the field description apply. |
| `--interval` | string | no |  |  | Survival-only interval code. Value form: string. Example: --interval 5. Command sources: monarch, mydisease, ols, seer; source restrictions in the field description apply. |
| `--ontology` | string | no |  | `mondo`, `doid`, `ncit` | One OLS ontology; help get disease One OLS ontology. Listed choices accept any letter case; replies use their listed spelling. Value form: string. Example: --ontology mondo. Command sources: monarch, mydisease, ols, seer; source restrictions in the field description apply. |
| `--period` | string | no |  |  | Incidence-only source period code. Value form: string. Example: --period 5. Command sources: monarch, mydisease, ols, seer; source restrictions in the field description apply. |
| `--race` | string | no |  |  | Source race code. Value form: string. Example: --race 1. Command sources: monarch, mydisease, ols, seer; source restrictions in the field description apply. |
| `--section-limit` | integer | no | min 1, max 50 |  | Source rows per page: default 2 for Monarch genes, 1 for Monarch phenotypes, otherwise 5. Value form: integer. Example: --section-limit 1. Command sources: monarch, mydisease, ols, seer; source restrictions in the field description apply. |
| `--section-offset` | integer | no | min 0 |  | Source row offset within the selected section. Value form: integer. Example: --section-offset 1. Command sources: monarch, mydisease, ols, seer; source restrictions in the field description apply. |
| `--sex` | string | no |  |  | Source sex code. Value form: string. Example: --sex 3. Command sources: monarch, mydisease, ols, seer; source restrictions in the field description apply. |
| `--source` | string | no |  | `ols`, `mydisease`, `monarch`, `seer` | Default MyDisease; no automatic fallback; help get disease Default MyDisease; no automatic fallback. Listed choices accept any letter case; replies use their listed spelling. Value form: string. Example: --source seer. Command sources: monarch, mydisease, ols, seer; source restrictions in the field description apply. |
| `id` (position 1) | string | yes |  |  | Exact ontology CURIE or a SEER site code or name. |
| `section` (position 2) | string | no |  | `synonyms`, `xrefs`, `hierarchy`, `genes`, `phenotypes`, `incidence`, `survival` | One named section; absent means identity card only; help get disease Listed choices accept any letter case; replies use their listed spelling. |

## Sections

One named section; absent means identity card only.

| Section | Description |
| --- | --- |
| `synonyms` | Publisher scoped synonyms. |
| `xrefs` | Source cross references. |
| `hierarchy` | Source parent relations. |
| `genes` | Source gene association rows. |
| `phenotypes` | Source phenotype association rows. |
| `incidence` | Source stage incidence rows. |
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

```sh
biomcp get disease MONDO:0007254 synonyms --source mydisease --section-limit 2 \
  --section-offset 1
```

```sh
biomcp get disease MONDO:0007254 --source ols --ontology mondo
```

## Reply

The reply is one JSON object. Its schema is one of [`biomcp.disease-card/6`](/reference/schemas/biomcp-disease-card-6/), [`biomcp.disease-genes/9`](/reference/schemas/biomcp-disease-genes-9/), [`biomcp.disease-hierarchy/5`](/reference/schemas/biomcp-disease-hierarchy-5/), [`biomcp.disease-incidence/7`](/reference/schemas/biomcp-disease-incidence-7/), [`biomcp.disease-page/5`](/reference/schemas/biomcp-disease-page-5/), [`biomcp.disease-phenotypes/9`](/reference/schemas/biomcp-disease-phenotypes-9/), [`biomcp.disease-sites/6`](/reference/schemas/biomcp-disease-sites-6/), [`biomcp.disease-survival/7`](/reference/schemas/biomcp-disease-survival-7/), [`biomcp.disease-synonyms/5`](/reference/schemas/biomcp-disease-synonyms-5/), [`biomcp.disease-xrefs/5`](/reference/schemas/biomcp-disease-xrefs-5/), whose page lists its fields.

## Next

The [disease guide](/user-guide/disease/) shows every disease command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
