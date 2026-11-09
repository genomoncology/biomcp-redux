---
title: biomcp get trial
description: "The BioMCP 1.0.0 command biomcp get trial: Reads clinical trials
  by NCT ID, and optionally sections such as eligibility or locations."
goal: Look up the arguments, sections and examples of biomcp get trial.
reader: A BioMCP user or agent checking one command
sidebar:
  label: get trial
kind: reference
verified: 2026-11-09
---

`biomcp get trial` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It reads clinical trials by NCT ID, and optionally sections such as eligibility or locations. A [section word](/concepts/progressive-disclosure/) after the identifier adds one more part of the record to the reply. This page lists its arguments, sections, examples and reply.

The program's help describes it:

> Get clinical trial details. Sources: ctgov, nci.

## Usage

```text
biomcp get trial [OPTIONS] <input...>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--limit` | integer | no | min 1, max 50 |  | Whole-record count per source collection page; text fragments follow the text budget. Value form: integer. Example: --limit 1. Command sources: ctgov, nci; source restrictions in the field description apply. |
| `--offset` | integer | no | min 0 |  | Character offset for ClinicalTrials.gov eligibility or NCI summary; whole-record offset for other collections. Value form: integer. Example: --offset 0. Command sources: ctgov, nci; source restrictions in the field description apply. |
| `--pool` | string | no |  |  | Checked stored source record, supplied by a continuation. Value form: string. Example: --pool &lt;pool&gt; (syntax; replace the placeholder with your retained value). Command sources: ctgov, nci; source restrictions in the field description apply. |
| `--section` | array | no |  | `eligibility`, `outcomes`, `results`, `arms`, `locations`, `contacts`, `references`, `documents`, `all`, `diseases`, `summary` | Optional intact source sections. Default cards only; diseases; help get trial Listed choices accept any letter case; replies use their listed spelling. Value form: repeatable values (repeat the flag). Example: --section outcomes. Command sources: ctgov, nci; source restrictions in the field description apply. |
| `--source` | string | no |  | `ctgov`, `nci` | Explicit source; defaults to ClinicalTrials.gov and never; help get trial Explicit source; defaults to ClinicalTrials.gov and never falls back. Listed choices accept any letter case; replies use their listed spelling. Value form: string. Example: --source nci. Command sources: ctgov, nci; source restrictions in the field description apply. |
| `input` (position 1) | array | yes | min 1, max 20 |  | One to twenty exact NCT identifiers, in caller order. |

## Sections

Optional intact source sections. Default cards only; diseases and summary require NCI. NCI locations, contacts, references and documents are unsupported.

| Section | Description |
| --- | --- |
| `eligibility` | Intact criteria and structured age/sex fields. |
| `outcomes` | Primary, secondary and other outcome records. |
| `results` | Posted outcome values, analyses and adverse-event tables (ClinicalTrials.gov). |
| `arms` | Arms and interventions with original relationships. |
| `locations` | Whole source sites with their attached contacts. |
| `contacts` | Central contacts and overall officials in their original roles. |
| `references` | Registry publication identities and source reference types. |
| `documents` | Posted document metadata and source URLs. |
| `all` | Every implemented trial section, each with independent continuations. |
| `diseases` | Source NCI disease concepts, including source enrichment metadata. |
| `summary` | Complete NCI summary, paged by original character offsets. |

## Sources

`--source` picks the source to ask: [ctgov](/sources/clinicaltrials-gov/), [nci](/sources/nci/).

## Examples

The program's help gives these examples.

```sh
biomcp get trial NCT04303780
```

```sh
biomcp get trial NCT04303780 documents
```

```sh
biomcp get trial NCT05136196 --source nci
```

```sh
biomcp get trial NCT03734029 locations --limit 1
```

```sh
biomcp get trial NCT02563002 --section outcomes
```

## Reply

The reply is one JSON object. Its schema is one of [`biomcp.nci-trial-cards/8`](/reference/schemas/biomcp-nci-trial-cards-8/), [`biomcp.trial-cards/9`](/reference/schemas/biomcp-trial-cards-9/), whose page lists its fields.

## Next

The [trial guide](/user-guide/trial/) shows every trial command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
