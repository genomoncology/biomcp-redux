---
title: biomcp search drug
description: "The BioMCP 1.0.0 command biomcp search drug: Finds drugs by name
  in one source."
goal: Look up the arguments, sections and examples of biomcp search drug.
reader: A BioMCP user or agent checking one command
sidebar:
  label: search drug
kind: reference
verified: 2026-11-10
---

`biomcp search drug` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It finds drugs by name in one source. This page lists its arguments, examples and reply.

The program's help describes it:

> Search drugs and medicines by name or source label indication. Sources: chembl, civic, ddinter, drugsfda, ema, fda-drug-recall, mychem, openfda-label, opentargets, who-pq.

## Usage

```text
biomcp search drug [OPTIONS] [query]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--classification` | string | no |  | `Class I`, `Class II`, `Class III` | Source FDA classification; drug recalls only; help search drug Source FDA classification; drug recalls only. Listed choices accept any letter case; replies use their listed spelling. Value form: string. Example: --classification 'Class II'. Command sources: chembl, civic, ddinter, drugsfda, ema, fda-drug-recall, mychem, openfda-label, opentargets, who-pq; source restrictions in the field description apply. |
| `--expand-drug-names` | boolean | no |  |  | Expand openFDA label aliases. Uses bounded source-stated names. Exact search is the default. Value form: boolean. Example: --expand-drug-names. Command sources: chembl, civic, ddinter, drugsfda, ema, fda-drug-recall, mychem, openfda-label, opentargets, who-pq; source restrictions in the field description apply. |
| `--firm` | string | no |  |  | Recall firm phrase. Matches the published firm literally. Value form: string. Example: --firm Apotex. Command sources: chembl, civic, ddinter, drugsfda, ema, fda-drug-recall, mychem, openfda-label, opentargets, who-pq; source restrictions in the field description apply. |
| `--indication` | string | no |  |  | Exact label indication; openFDA labels only. Search the source Indications and Usage field without disease synonym expansion. An optional name query further narrows the same source search. Value form: string. Example: --indication melanoma. Command sources: chembl, civic, ddinter, drugsfda, ema, fda-drug-recall, mychem, openfda-label, opentargets, who-pq; source restrictions in the field description apply. |
| `--initiated-from` | string | no |  |  | Inclusive initiation day, YYYY-MM-DD; drug recalls only. Value form: string. Example: --initiated-from 2024-01-01. Command sources: chembl, civic, ddinter, drugsfda, ema, fda-drug-recall, mychem, openfda-label, opentargets, who-pq; source restrictions in the field description apply. |
| `--initiated-to` | string | no |  |  | Inclusive initiation day, YYYY-MM-DD; drug recalls only. Value form: string. Example: --initiated-to 2024-12-31. Command sources: chembl, civic, ddinter, drugsfda, ema, fda-drug-recall, mychem, openfda-label, opentargets, who-pq; source restrictions in the field description apply. |
| `--limit` | integer | no | min 1, max 50 |  | Source candidate rows per page. Value form: integer. Example: --limit 1. Command sources: chembl, civic, ddinter, drugsfda, ema, fda-drug-recall, mychem, openfda-label, opentargets, who-pq; source restrictions in the field description apply. |
| `--name-selector` | string | no |  | `preferred`, `synonym` | ChEMBL name selector; help search drug ChEMBL name selector. Omit to search the preferred name. Listed choices accept any letter case; replies use their listed spelling. Value form: string. Example: --name-selector preferred. Command sources: chembl, civic, ddinter, drugsfda, ema, fda-drug-recall, mychem, openfda-label, opentargets, who-pq; source restrictions in the field description apply. |
| `--name-snapshot` | string | no |  |  | Label alias page token. Follow the returned command with unchanged filters to retain aliases and matching tier. Value form: string. Example: --name-snapshot &lt;name-snapshot&gt; (syntax; replace the placeholder with your retained value). Command sources: chembl, civic, ddinter, drugsfda, ema, fda-drug-recall, mychem, openfda-label, opentargets, who-pq; source restrictions in the field description apply. |
| `--offset` | integer | no | min 0 |  | Source rows to skip. Value form: integer. Example: --offset 1. Command sources: chembl, civic, ddinter, drugsfda, ema, fda-drug-recall, mychem, openfda-label, opentargets, who-pq; source restrictions in the field description apply. |
| `--prequalified-from` | string | no |  |  | WHO PQ start date, inclusive. Accepts YYYY, YYYY-MM or YYYY-MM-DD. Value form: string. Example: --prequalified-from 2020. Command sources: chembl, civic, ddinter, drugsfda, ema, fda-drug-recall, mychem, openfda-label, opentargets, who-pq; source restrictions in the field description apply. |
| `--prequalified-to` | string | no |  |  | WHO PQ end date, inclusive. Accepts YYYY, YYYY-MM or YYYY-MM-DD. Value form: string. Example: --prequalified-to 2020. Command sources: chembl, civic, ddinter, drugsfda, ema, fda-drug-recall, mychem, openfda-label, opentargets, who-pq; source restrictions in the field description apply. |
| `--reason` | string | no |  |  | Recall reason phrase. Value form: string. Example: --reason 'Lack of Sterility Assurance'. Command sources: chembl, civic, ddinter, drugsfda, ema, fda-drug-recall, mychem, openfda-label, opentargets, who-pq; source restrictions in the field description apply. |
| `--source` | string | no |  | `who-pq`, `fda-drug-recall`, `mychem`, `chembl`, `opentargets`, `civic`, `openfda-label`, `drugsfda`, `ema`, `ddinter` | Selected source; help search drug Selected source. After an empty name lookup, existing sources may supply aliases. Listed choices accept any letter case; replies use their listed spelling. Value form: string. Example: --source ema. Command sources: chembl, civic, ddinter, drugsfda, ema, fda-drug-recall, mychem, openfda-label, opentargets, who-pq; source restrictions in the field description apply. |
| `--status` | string | no |  |  | Exact recall status. Value form: string. Example: --status Ongoing. Command sources: chembl, civic, ddinter, drugsfda, ema, fda-drug-recall, mychem, openfda-label, opentargets, who-pq; source restrictions in the field description apply. |
| `query` (position 1) | string | no |  |  | Drug name phrase. FDA drug recalls, WHO date ranges and label indication searches allow no name. Drugs@FDA searches brands and ingredients, exact names before word matches. openFDA labels search exact then word brand/generic names, then label text (including indications) when no names match; source matches need not be exact names. Harmonized recall names match but never become display names. |

## Sources

`--source` picks the source to ask: [who-pq](/sources/who-prequalification/), [fda-drug-recall](/sources/fda-drug-recall/), [mychem](/sources/mychem/), [chembl](/sources/chembl/), [opentargets](/sources/opentargets/), [civic](/sources/civic/), [openfda-label](/sources/openfda/), [drugsfda](/sources/drugsfda/), [ema](/sources/ema/), [ddinter](/sources/ddinter/).

## Examples

The program's help gives these examples.

```sh
biomcp search drug Tibsovo --source ema --limit 1
```

```sh
biomcp search drug Tibsovo --source ema --limit 1 --offset 1 --fresh
```

```sh
biomcp search drug --indication melanoma --source openfda-label --limit 1
```

```sh
biomcp search drug --source fda-drug-recall --firm Apotex --limit 1
```

```sh
biomcp search drug --source fda-drug-recall --status Ongoing \
  --classification 'Class II' --limit 1
```

```sh
biomcp search drug --source fda-drug-recall \
  --reason 'Lack of Sterility Assurance' --limit 1
```

```sh
biomcp search drug --source fda-drug-recall --initiated-from 2024-01-01 \
  --initiated-to 2024-12-31 --limit 1
```

```sh
biomcp search drug --source who-pq --prequalified-from 2020 \
  --prequalified-to 2020 --limit 50 --offset 0
```

## Reply

The reply is one JSON object. Its schema is one of [`biomcp.drug-fda-page/7`](/reference/schemas/biomcp-drug-fda-page-7/), [`biomcp.drug-page/7`](/reference/schemas/biomcp-drug-page-7/), [`biomcp.drug-recall-page/4`](/reference/schemas/biomcp-drug-recall-page-4/), [`biomcp.drug-who-page/5`](/reference/schemas/biomcp-drug-who-page-5/), whose page lists its fields.

## Next

The [drug guide](/user-guide/drug/) shows every drug command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
