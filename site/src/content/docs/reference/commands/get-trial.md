---
title: biomcp get trial
description: "Reference for the BioMCP 1.0.0 command biomcp get trial: Retrieve
  exact trial cards in input order; request bounded intact source sections with
  independent continuations."
goal: Look up the arguments, sections and examples of biomcp get trial.
reader: A BioMCP user or agent checking one command
sidebar:
  label: get trial
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 command `biomcp get trial`.

Retrieve exact trial cards in input order; request bounded intact source sections with independent continuations.

## Usage

```text
biomcp get trial [OPTIONS] <input...>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--limit` | integer | no | min 1, max 50 |  | Whole-record count per source collection page; text fragments follow the text budget. |
| `--offset` | integer | no | min 0 |  | Character offset for ClinicalTrials.gov eligibility or NCI summary; whole-record offset for other collections. |
| `--pool` | string | no |  |  | Checked stored source record, supplied by a continuation. |
| `--section` | array | no |  | `eligibility`, `outcomes`, `arms`, `locations`, `contacts`, `references`, `documents`, `all`, `diseases`, `summary` | Optional intact source sections. Default cards only; diseases and summary require NCI. NCI locations, contacts, references and documents are unsupported. |
| `--source` | string | no |  | `ctgov`, `nci` | Explicit provider; defaults to ClinicalTrials.gov and never falls back. |
| `input` (position 1) | array | yes | min 1, max 20 |  | One to twenty exact NCT identifiers, in caller order. |

## Sections

Optional intact source sections. Default cards only; diseases and summary require NCI. NCI locations, contacts, references and documents are unsupported.

| Section | Description |
| --- | --- |
| `eligibility` | Intact criteria and structured age/sex fields. |
| `outcomes` | Primary, secondary and other outcome records. |
| `arms` | Arms and interventions with original relationships. |
| `locations` | Whole source sites with their attached contacts. |
| `contacts` | Central contacts and overall officials in their original roles. |
| `references` | Registry publication identities and native reference types. |
| `documents` | Posted document metadata and source URLs. |
| `all` | Every implemented trial section, each with independent continuations. |
| `diseases` | Native NCI disease concepts, including source enrichment metadata. |
| `summary` | Complete NCI summary, paged by original character offsets. |

## Sources

`--source` accepts these sources: [ctgov](/reference/sources/ctgov/), [nci](/reference/sources/nci/).

## Examples

```sh
biomcp get trial NCT04303780
biomcp get trial NCT04303780 documents
```

## Exit codes

See [exit codes](/reference/exit-codes/). Options that apply to every command are in [global options](/reference/global-options/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
