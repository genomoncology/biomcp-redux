---
title: biomcp get study
description: "The BioMCP 1.0.0 command biomcp get study: Reads one cBioPortal
  study, and optionally its profiles, sample lists or files."
goal: Look up the arguments, sections and examples of biomcp get study.
reader: A BioMCP user or agent checking one command
sidebar:
  label: get study
kind: reference
verified: 2026-11-09
---

`biomcp get study` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It reads one cBioPortal study, and optionally its profiles, sample lists or files. A [section word](/concepts/progressive-disclosure/) after the identifier adds one more part of the record to the reply. This page lists its arguments, sections, examples and reply.

The program's help describes it:

> Get cancer genomics study details. Sources: cbioportal.

## Usage

```text
biomcp get study [OPTIONS] <id> [section]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--gene` | string | no |  |  | Exact cBioPortal HUGO gene symbol or Entrez ID; required for mutations. Value form: string. Example: --gene BRAF. Command sources: cbioportal; source restrictions in the field description apply. |
| `--molecular-profile` | string | no |  |  | Exact mutation profile ID; omitted selects the study's first mutation profile. Value form: string. Example: --molecular-profile skcm_tcga_pan_can_atlas_2018_mutations. Command sources: cbioportal; source restrictions in the field description apply. |
| `--section-limit` | integer | no | min 1, max 50 |  | Maximum records in each selected section. Value form: integer. Example: --section-limit 1. Command sources: cbioportal; source restrictions in the field description apply. |
| `--section-offset` | integer | no | min 0 |  | Offset within one named section; all requires zero. Value form: integer. Example: --section-offset 45. Command sources: cbioportal; source restrictions in the field description apply. |
| `id` (position 1) | string | yes |  |  | Exact source study ID; no default or repair. |
| `section` (position 2) | string | no |  | `profiles`, `sample-lists`, `mutations`, `samples`, `files`, `all` | Omit for the card; all returns bounded first pages only; help get study Listed choices accept any letter case; replies use their listed spelling. |

## Sections

Omit for the card; all returns bounded first pages only.

| Section | Description |
| --- | --- |
| `profiles` | Five exact source molecular profile references. |
| `sample-lists` | Five sample-list metadata records, without member sample IDs. |
| `mutations` | Source mutation rows for the required gene and selected mutation profile. |
| `samples` | Source sample identities and types only. |
| `files` | Five repository file references; no automatic download. |
| `all` | Bounded first pages; mutations are included only when gene is supplied. |

## Examples

The program's help gives these examples.

```sh
biomcp get study synthetic_0
```

```sh
biomcp get study synthetic_1
```

```sh
biomcp get study skcm_tcga_pan_can_atlas_2018 mutations --gene BRAF \
  --section-limit 1 --section-offset 45
```

```sh
biomcp get study skcm_tcga_pan_can_atlas_2018 mutations --gene BRAF \
  --section-limit 1 --section-offset 46 \
  --molecular-profile skcm_tcga_pan_can_atlas_2018_mutations
```

## Reply

The reply is one JSON object. Its schema is one of [`biomcp.study-card/7`](/reference/schemas/biomcp-study-card-7/), [`biomcp.study-files/3`](/reference/schemas/biomcp-study-files-3/), [`biomcp.study-mutations/1`](/reference/schemas/biomcp-study-mutations-1/), [`biomcp.study-profiles/4`](/reference/schemas/biomcp-study-profiles-4/), [`biomcp.study-sample-lists/4`](/reference/schemas/biomcp-study-sample-lists-4/), [`biomcp.study-samples/1`](/reference/schemas/biomcp-study-samples-1/), whose page lists its fields.

## Next

The [study guide](/user-guide/study/) shows every study command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
