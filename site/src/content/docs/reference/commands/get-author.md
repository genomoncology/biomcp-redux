---
title: biomcp get author
description: "Reference for the BioMCP 1.0.0 command biomcp get author: Exact
  qualified records in caller order, with no name reconciliation."
goal: Look up the arguments, sections and examples of biomcp get author.
reader: A BioMCP user or agent checking one command
sidebar:
  label: get author
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 command `biomcp get author`.

Exact qualified records in caller order, with no name reconciliation. Author IDs identify provider records, not resolved people. Counts cover that provider record. Missing metadata is unavailable. Topic-wide author ranking and cross-provider merging are unavailable. Semantic Scholar and ORCID are noncommercial; commercial users disable both with --disable-sources noncommercial or config disable_sources. Optional keys: biomcp key set semantic-scholar or S2_API_KEY; biomcp key set orcid or ORCID_ACCESS_TOKEN. IDs use semantic-scholar:ID (alias semanticscholar:ID) or orcid:ID. Search supports --source semantic-scholar (alias semanticscholar), --query/-q and --limit/-l; papers supports --limit/-l and Semantic Scholar details/--full.

## Usage

```text
biomcp get author <ids...>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `ids` (position 1) | array | yes | min 1, max 20 |  | One or more exact qualified IDs. |

## Examples

```sh
biomcp get author semanticscholar:456 semantic-scholar:123 semantic-scholar:999
biomcp get author semantic-scholar:123 semantic-scholar:123
```

## Exit codes

See [exit codes](/reference/exit-codes/). Options that apply to every command are in [global options](/reference/global-options/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
