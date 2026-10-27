---
title: biomcp get study
description: "The BioMCP 1.0.0 command biomcp get study: Reads one cBioPortal
  study, and optionally its profiles, sample lists or files."
goal: Look up the arguments, sections and examples of biomcp get study.
reader: A BioMCP user or agent checking one command
sidebar:
  label: get study
kind: reference
verified: 2026-10-27
---

`biomcp get study` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It reads one cBioPortal study, and optionally its profiles, sample lists or files. A [section word](/concepts/progressive-disclosure/) after the identifier adds one more part of the record to the reply. This page lists its arguments, sections, examples and reply.

The program's help describes it:

> Exact case-sensitive study card or bounded metadata section. No frequency, denominator, survival or comparison is calculated. Source counts keep their native names.

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

The program's help gives these examples.

```sh
biomcp get study synthetic_0
```

```sh
biomcp get study synthetic_1
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.study_card/2`](/reference/schemas/biomcp-study-card-2/), whose page shows a real reply and its fields.

## Next

The [study guide](/user-guide/study/) shows every study command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
