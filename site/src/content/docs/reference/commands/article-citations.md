---
title: biomcp article citations
description: "Reference for the BioMCP 1.0.0 command biomcp article citations:
  Page the citing or referenced PubMed papers of one checked PMID."
goal: Look up the arguments, sections and examples of biomcp article citations.
reader: A BioMCP user or agent checking one command
sidebar:
  label: article citations
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 command `biomcp article citations`.

Page the citing or referenced PubMed papers of one checked PMID.

## Usage

```text
biomcp article citations [OPTIONS] <pmid>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--limit` | integer | no | min 1, max 50 |  | Maximum papers on one page. |
| `--offset` | integer | no | min 0 |  | Number of pool items to skip. |
| `--pool` | string | no |  |  | Fixed snapshot from an earlier page. |
| `--source` | string | no |  | `icite`, `semantic-scholar` | Graph provider; iCite remains the default. |
| `pmid` (position 1) | string | yes |  |  | PMID, PMC or DOI; iCite accepts only a PMID. |

## Sources

`--source` accepts these sources: [icite](/sources/icite/), [semantic-scholar](/sources/semantic-scholar/).

## Examples

```sh
biomcp article citations 17611497
biomcp article citations 23000897 --source semantic-scholar
```

## Exit codes

See [exit codes](/reference/error-codes/). Options that apply to every command are in [global options](/reference/global-options/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
