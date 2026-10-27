---
title: biomcp search trial
description: "Reference for the BioMCP 1.0.0 command biomcp search trial: Search
  one provider with bounded pages and source-native counts; default
  ClinicalTrials.gov, explicit keyed NCI, never a fallback."
goal: Look up the arguments, sections and examples of biomcp search trial.
reader: A BioMCP user or agent checking one command
sidebar:
  label: search trial
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 command `biomcp search trial`.

Search one provider with bounded pages and source-native counts; default ClinicalTrials.gov, explicit keyed NCI, never a fallback.

## Usage

```text
biomcp search trial [OPTIONS] [query]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--age` | integer | no | min 0 |  | Age in years; unstated source bounds count as no limit. |
| `--condition` | string | no |  |  | Registry condition text, or an NCIt code for NCI; use --keyword for NCI free text. |
| `--condition-id` | string | no |  |  | Exact MONDO or DOID CURIE resolved by MyDisease; NCI also accepts explicit NCIT:C codes. |
| `--count-only` | boolean | no |  |  | Source total only; no returned study rows. |
| `--criteria` | string | no |  |  | Search literal registry eligibility text for discovery only; no eligibility judgment. |
| `--distance` | string | no |  |  | Source distance with explicit mi or km units. |
| `--facility` | string | no |  |  | Exact registry facility field; cannot combine with radius. |
| `--gene` | string | no |  |  | Resolve one checked gene for registry text discovery only; no eligibility judgment. |
| `--has-results` | boolean | no |  |  | Require posted source results. |
| `--id` | string | no |  |  | Exact NCT identity in a conjunction with other filters. |
| `--intervention` | string | no |  |  | Literal source intervention text. |
| `--keyword` | string | no |  |  | NCI free text; explicit keyword mode never substitutes for a disease identity. |
| `--lat` | string | no |  |  | Radius center latitude in degrees. |
| `--limit` | integer | no | min 1, max 100 |  | Headers per page; NCI applies its own lower publisher limit. |
| `--lon` | string | no |  |  | Radius center longitude in degrees. |
| `--next-page` | string | no |  |  | Opaque registry continuation; keep filters unchanged. |
| `--phase` | string | no |  | `early-1`, `1`, `2`, `3`, `4`, `not-applicable`, `1/2`, `2/3` | Registry phase: 1, 2, 3, 4, early-1, not-applicable, 1/2 or 2/3. |
| `--sex` | string | no |  | `ALL`, `FEMALE`, `MALE` | Registry sex facet. |
| `--source` | string | no |  | `ctgov`, `nci` | Explicit provider; defaults to ClinicalTrials.gov and never falls back. |
| `--sponsor` | string | no |  |  | Exact lead sponsor field. |
| `--sponsor-type` | string | no |  | `nih`, `fed`, `industry`, `other` | Registry sponsor funding facet. |
| `--status` | string | no |  | `ACTIVE_NOT_RECRUITING`, `COMPLETED`, `ENROLLING_BY_INVITATION`, `NOT_YET_RECRUITING`, `RECRUITING`, `SUSPENDED`, `TERMINATED`, `WITHDRAWN`, `AVAILABLE`, `NO_LONGER_AVAILABLE`, `APPROVED_FOR_MARKETING`, `WITHHELD`, `UNKNOWN` | Exact registry overall status. |
| `--study-type` | string | no |  | `INTERVENTIONAL`, `OBSERVATIONAL`, `EXPANDED_ACCESS` | Exact registry study type. |
| `--updated-since` | string | no |  |  | First included registry update date. |
| `--updated-until` | string | no |  |  | Last included registry update date. |
| `--variant` | string | no |  |  | Literal variant text for registry discovery only; no eligibility judgment. |
| `query` (position 1) | string | no |  |  | General registry discovery text. |

## Sources

`--source` accepts these sources: [ctgov](/sources/clinicaltrials-gov/), [nci](/sources/nci/).

## Examples

```sh
biomcp search trial --condition melanoma
biomcp search trial --condition melanoma --intervention pembrolizumab
```

## Exit codes

See [exit codes](/reference/error-codes/). Options that apply to every command are in [global options](/reference/global-options/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
