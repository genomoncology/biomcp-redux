---
title: biomcp author papers
description: "The BioMCP 1.0.0 command biomcp author papers: Lists the papers of
  one author record from Semantic Scholar or ORCID, a page at a time."
goal: Look up the arguments, sections and examples of biomcp author papers.
reader: A BioMCP user or agent checking one command
sidebar:
  label: author papers
kind: reference
verified: 2026-10-27
---

`biomcp author papers` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It lists the papers of one author record from Semantic Scholar or ORCID, a page at a time. A [section word](/concepts/progressive-disclosure/) after the identifier adds one more part of the record to the reply. This page lists its arguments, sections and examples.

The program's help describes it:

> One exact provider record's publication window. ORCID sends its whole public works list in one reply and pages groups locally. Author IDs identify provider records, not resolved people. Counts cover that provider record. Missing metadata is unavailable. Topic-wide author ranking and cross-provider merging are unavailable. Semantic Scholar and ORCID are noncommercial; commercial users disable both with --disable-sources noncommercial or config disable_sources. Optional keys: biomcp key set semantic-scholar or S2_API_KEY; biomcp key set orcid or ORCID_ACCESS_TOKEN. IDs use semantic-scholar:ID (alias semanticscholar:ID) or orcid:ID. Search supports --source semantic-scholar (alias semanticscholar), --query/-q and --limit/-l; papers supports --limit/-l and Semantic Scholar details/--full.

## Usage

```text
biomcp author papers [OPTIONS] <id> [section]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--full` | boolean | no |  |  | Legacy alias for details; normalized on construction. |
| `--limit` | integer | no | min 1, max 25 |  | Bounded row count. |
| `--offset` | integer | no | min 0 |  | Native or retained collection offset. |
| `--pool` | string | no |  |  | The ID of a stored list, from an earlier reply, so later pages read the same data. |
| `id` (position 1) | string | yes |  |  | Exact qualified provider record. |
| `section` (position 2) | string | no |  | `details` | Detailed disclosure when requested. |

## Sections

Detailed disclosure when requested.

| Section | Description |
| --- | --- |
| `details` | Native detailed metadata for the same paper window. |

## Examples

The program's help gives these examples.

```sh
biomcp author papers semantic-scholar:123
```

```sh
biomcp author papers semantic-scholar:123 details
```

## Next

The [author guide](/user-guide/author/) shows every author command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
