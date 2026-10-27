---
title: biomcp get study
description: "Reference for the BioMCP 1.0.0 command biomcp get study: Exact
  case-sensitive study card or bounded metadata section."
goal: Look up the arguments, sections and examples of biomcp get study.
reader: A BioMCP user or agent checking one command
sidebar:
  label: get study
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 command `biomcp get study`.

Exact case-sensitive study card or bounded metadata section. No frequency, denominator, survival or comparison is calculated. Source counts keep their native names.

## Usage

```text
biomcp get study [OPTIONS] <id> [section]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--section-limit` | integer | no | min 1, max 50 |  | Maximum records in each selected section. |
| `--section-offset` | integer | no | min 0 |  | Offset within one named section; all requires zero. |
| `id` (position 1) | string | yes |  |  | Exact source study ID; no default or repair. |
| `section` (position 2) | value | no |  |  | Omit for the card; all returns bounded first pages only. |

## Sections

Omit for the card; all returns bounded first pages only.

| Section | Description |
| --- | --- |
| `profiles` | Five exact native molecular profile references. |
| `sample-lists` | Five sample-list metadata records, without member sample IDs. |
| `files` | Five repository file references; no automatic download. |
| `all` | Each named section's bounded first page; never a gene aggregate request. |

## Examples

```sh
biomcp get study synthetic_0
biomcp get study synthetic_1
```

## Exit codes

See [exit codes](/reference/error-codes/). Options that apply to every command are in [global options](/reference/global-options/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
