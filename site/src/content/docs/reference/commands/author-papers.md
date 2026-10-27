---
title: biomcp author papers
description: "Reference for the BioMCP 1.0.0 command biomcp author papers: One
  exact provider record's publication window."
goal: Look up the arguments, sections and examples of biomcp author papers.
reader: A BioMCP user or agent checking one command
sidebar:
  label: author papers
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 command `biomcp author papers`.

One exact provider record's publication window. ORCID sends its whole public works list in one reply and pages groups locally. Author IDs identify provider records, not resolved people. Counts cover that provider record. Missing metadata is unavailable. Topic-wide author ranking and cross-provider merging are unavailable. Semantic Scholar and ORCID are noncommercial; commercial users disable both with --disable-sources noncommercial or config disable_sources. Optional keys: biomcp key set semantic-scholar or S2_API_KEY; biomcp key set orcid or ORCID_ACCESS_TOKEN. IDs use semantic-scholar:ID (alias semanticscholar:ID) or orcid:ID. Search supports --source semantic-scholar (alias semanticscholar), --query/-q and --limit/-l; papers supports --limit/-l and Semantic Scholar details/--full.

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
| `--pool` | string | no |  |  | Retained collection identity. |
| `id` (position 1) | string | yes |  |  | Exact qualified provider record. |
| `section` (position 2) | string | no |  | `details` | Detailed disclosure when requested. |

## Sections

Detailed disclosure when requested.

| Section | Description |
| --- | --- |
| `details` | Native detailed metadata for the same paper window. |

## Examples

```sh
biomcp author papers semantic-scholar:123
biomcp author papers semantic-scholar:123 details
```

## Reply schema

The help examples of this command return [`biomcp.author_papers/1`](/reference/schemas/biomcp-author-papers-1/), [`biomcp.author_paper_details/1`](/reference/schemas/biomcp-author-paper-details-1/). Print the full JSON schema with `biomcp schema biomcp.author_papers/1`.

## Exit codes

See [exit codes](/reference/error-codes/). Options that apply to every command are in [global options](/reference/global-options/).

## Next

The [author entity](/user-guide/author/) page lists every author command and links to a worked guide.

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
