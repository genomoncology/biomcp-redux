---
title: biomcp search diagnostic
description: "The BioMCP 1.0.0 command biomcp search diagnostic: Finds
  diagnostic tests in one source, such as GTR, the FDA companion diagnostic list
  or WHO's prequalified tests."
goal: Look up the arguments, sections and examples of biomcp search diagnostic.
reader: A BioMCP user or agent checking one command
sidebar:
  label: search diagnostic
kind: reference
verified: 2026-11-09
---

`biomcp search diagnostic` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It finds diagnostic tests in one source, such as GTR, the FDA companion diagnostic list or WHO's prequalified tests. This page lists its arguments, examples and reply.

The program's help describes it:

> Search diagnostic laboratory tests and devices. Sources: fda-510k, fda-cdx, fda-device-recall, fda-pma, gtr, who-ivd.

## Usage

```text
biomcp search diagnostic [OPTIONS]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--biomarker` | string | no |  |  | Exact table-0 cell. Exact table-0 biomarker cell; unsupported in the group table. Value form: string. Example: --biomarker KRAS. Command sources: fda-510k, fda-cdx, fda-device-recall, fda-pma, gtr, who-ivd; source restrictions in the field description apply. |
| `--bundle-revision` | string | no |  |  | Stored [bundle](/local-data/) revision. Immutable stored bundle revision retained by a continuation. Value form: string. Example: --bundle-revision sha256:3eb93d3ba1a08b508955b0699d32042584fd8e0c805c72f7eeb470ef04ba959a. Command sources: fda-510k, fda-cdx, fda-device-recall, fda-pma, gtr, who-ivd; source restrictions in the field description apply. |
| `--condition-id` | string | no |  |  | Exact MedGen:CUI. Exact MedGen:CUI identifier only. Value form: string. Example: --condition-id MedGen:C2676676. Command sources: fda-510k, fda-cdx, fda-device-recall, fda-pma, gtr, who-ivd; source restrictions in the field description apply. |
| `--decision-from` | string | no |  |  | First date, YYYY-MM-DD. Inclusive source decision date, YYYY-MM-DD; PMA and 510(k) only. Value form: string. Example: --decision-from 2017-11-30. Command sources: fda-510k, fda-cdx, fda-device-recall, fda-pma, gtr, who-ivd; source restrictions in the field description apply. |
| `--decision-to` | string | no |  |  | Last date, YYYY-MM-DD. Inclusive source decision date, YYYY-MM-DD; PMA and 510(k) only. Value form: string. Example: --decision-to 2017-11-30. Command sources: fda-510k, fda-cdx, fda-device-recall, fda-pma, gtr, who-ivd; source restrictions in the field description apply. |
| `--device-id` | string | no |  |  | PMA or K identifier. Exact PMA application, application/supplement or source K identity. Value form: string. Example: --device-id P200010/S026. Command sources: fda-510k, fda-cdx, fda-device-recall, fda-pma, gtr, who-ivd; source restrictions in the field description apply. |
| `--drug-application` | string | no |  |  | NDA/BLA application. Cell-owned typed NDA/BLA application. Value form: string. Example: --drug-application BLA125514. Command sources: fda-510k, fda-cdx, fda-device-recall, fda-pma, gtr, who-ivd; source restrictions in the field description apply. |
| `--gene` | string | no |  |  | Exact gene symbol. Exact source gene_symbol field, without alias expansion. Value form: string. Example: --gene BRCA1. Command sources: fda-510k, fda-cdx, fda-device-recall, fda-pma, gtr, who-ivd; source restrictions in the field description apply. |
| `--gene-id` | string | no |  |  | Exact NCBI GeneID. Exact source GeneID, exclusive with gene symbol. Value form: string. Example: --gene-id 672. Command sources: fda-510k, fda-cdx, fda-device-recall, fda-pma, gtr, who-ivd; source restrictions in the field description apply. |
| `--initiated-from` | string | no |  |  | First date, YYYY-MM-DD. Inclusive initiation date, YYYY-MM-DD; FDA device recalls only. Value form: string. Example: --initiated-from 2024-01-01. Command sources: fda-510k, fda-cdx, fda-device-recall, fda-pma, gtr, who-ivd; source restrictions in the field description apply. |
| `--initiated-to` | string | no |  |  | Last date, YYYY-MM-DD. Inclusive initiation date, YYYY-MM-DD; FDA device recalls only. Value form: string. Example: --initiated-to 2024-12-31. Command sources: fda-510k, fda-cdx, fda-device-recall, fda-pma, gtr, who-ivd; source restrictions in the field description apply. |
| `--limit` | integer | no | min 1, max 50 |  | Five results by default; at most fifty source rows or product headers. Value form: integer. Example: --limit 1. Command sources: fda-510k, fda-cdx, fda-device-recall, fda-pma, gtr, who-ivd; source restrictions in the field description apply. |
| `--manufacturer` | string | no |  |  | Manufacturer phrase. WHO manufacturer words, source FDA applicant (PMA/510(k)) or recalling_firm phrase. Value form: string. Example: --manufacturer 'Synthetic maker'. Command sources: fda-510k, fda-cdx, fda-device-recall, fda-pma, gtr, who-ivd; source restrictions in the field description apply. |
| `--marker` | string | no |  |  | WHO marker phrase. Exact whole-word phrase in WHO's Pathogen/Disease/Marker cell. Value form: string. Example: --marker hIv. Command sources: fda-510k, fda-cdx, fda-device-recall, fda-pma, gtr, who-ivd; source restrictions in the field description apply. |
| `--name` | string | no |  |  | Source device name. Source trade_name (PMA) or device_name (510(k)) phrase. Value form: string. Example: --name 'FoundationOne CDx'. Command sources: fda-510k, fda-cdx, fda-device-recall, fda-pma, gtr, who-ivd; source restrictions in the field description apply. |
| `--offset` | integer | no | min 0 |  | Zero-based source result offset. Value form: integer. Example: --offset 5. Command sources: fda-510k, fda-cdx, fda-device-recall, fda-pma, gtr, who-ivd; source restrictions in the field description apply. |
| `--product-code` | string | no |  |  | FDA product code. Exact source FDA product code; PMA, 510(k) and device recalls only. Value form: string. Example: --product-code ABC. Command sources: fda-510k, fda-cdx, fda-device-recall, fda-pma, gtr, who-ivd; source restrictions in the field description apply. |
| `--source` | string | yes |  | `gtr`, `fda-cdx`, `fda-pma`, `fda-510k`, `who-ivd`, `fda-device-recall` | Source diagnostic source; help search diagnostic Source diagnostic source. Explicit source GTR, WHO IVD, FDA PMA, 510(k), companion or medical device recall discovery. Listed choices accept any letter case; replies use their listed spelling. Value form: string. Example: --source fda-device-recall. Command sources: fda-510k, fda-cdx, fda-device-recall, fda-pma, gtr, who-ivd; source restrictions in the field description apply. |
| `--type` | string | no |  | `Clinical`, `Research`, `Clinical microbial` | Source test_type; help search diagnostic Source test_type. Exact source test_type value. Listed choices accept any letter case; replies use their listed spelling. Value form: string. Example: --type Research. Command sources: fda-510k, fda-cdx, fda-device-recall, fda-pma, gtr, who-ivd; source restrictions in the field description apply. |

## Sources

`--source` picks the source to ask: [gtr](/sources/gtr/), [fda-cdx](/sources/fda-cdx/), [fda-pma](/sources/fda-pma/), [fda-510k](/sources/fda-510k/), [who-ivd](/sources/who-ivd/), [fda-device-recall](/sources/fda-device-recall/).

## Examples

The program's help gives these examples.

```sh
biomcp search diagnostic --source fda-device-recall \
  --manufacturer 'Synthetic maker'
```

```sh
biomcp search diagnostic --source fda-device-recall --product-code ABC
```

```sh
biomcp search diagnostic --source gtr --gene BRCA1
```

```sh
biomcp search diagnostic --source gtr --condition-id MedGen:C2676676
```

```sh
biomcp search diagnostic --source fda-pma --name 'FoundationOne CDx'
```

```sh
biomcp search diagnostic --source fda-pma --manufacturer 'Foundation Medicine' \
  --decision-from 2017-11-30 --decision-to 2017-11-30
```

```sh
biomcp search diagnostic --source fda-cdx --device-id P200010/S026
```

```sh
biomcp search diagnostic --biomarker KRAS \
  --bundle-revision sha256:3eb93d3ba1a08b508955b0699d32042584fd8e0c805c72f7eeb470ef04ba959a \
  --offset 5 --source fda-cdx
```

```sh
biomcp search diagnostic --source gtr --gene-id 672 --offset 5
```

```sh
biomcp search diagnostic --source gtr --gene BRCA1 --type Research
```

```sh
biomcp search diagnostic --source fda-cdx --drug-application BLA125514
```

```sh
biomcp search diagnostic --source fda-device-recall \
  --initiated-from 2024-01-01 --initiated-to 2024-12-31 --limit 1
```

```sh
biomcp search diagnostic --source who-ivd --marker hIv
```

## Reply

The reply is one JSON object. Its schema is one of [`biomcp.diagnostic-companion/4`](/reference/schemas/biomcp-diagnostic-companion-4/), [`biomcp.diagnostic-page/5`](/reference/schemas/biomcp-diagnostic-page-5/), whose page lists its fields.

## Next

The [diagnostic guide](/user-guide/diagnostic/) shows every diagnostic command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
