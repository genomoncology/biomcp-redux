---
title: biomcp article citation-evidence
description: "Reference for the BioMCP 1.0.0 command biomcp article
  citation-evidence: Read every source context and optional explicit full-text
  citation paragraph."
goal: Look up the arguments, sections and examples of biomcp article
  citation-evidence.
reader: A BioMCP user or agent checking one command
sidebar:
  label: article citation-evidence
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 command `biomcp article citation-evidence`.

Read every source context and optional explicit full-text citation paragraph.

## Usage

```text
biomcp article citation-evidence [OPTIONS] <citing> <cited>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--fulltext` | boolean | no |  |  | Include the citing paper's own explicit reference paragraphs. |
| `--offset` | integer | no | min 0 |  | Context fragment position returned by the advancing step. |
| `--paragraph-offset` | integer | no | min 0 |  | Full-text fragment position returned by the advancing step. |
| `--pool` | string | no |  |  | Stored checked source scan and original hit positions. |
| `citing` (position 1) | string | yes |  |  | Citing PMID, PMC identity or DOI. |
| `cited` (position 2) | string | yes |  |  | Cited PMID, PMC identity or DOI. |

## Examples

```sh
biomcp article citation-evidence 33123537 17611497
biomcp article citation-evidence 33123537 17611497 --fulltext
```

## Reply schema

The help examples of this command return [`biomcp.article_evidence/2`](/reference/schemas/biomcp-article-evidence-2/). Print the full JSON schema with `biomcp schema biomcp.article_evidence/2`.

## Exit codes

See [exit codes](/reference/error-codes/). Options that apply to every command are in [global options](/reference/global-options/).

## Next

The [article entity](/user-guide/article/) page lists every article command and links to a worked guide.

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
