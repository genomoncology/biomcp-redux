---
title: biomcp get article
description: "Reference for the BioMCP 1.0.0 command biomcp get article: Get one
  to twenty article inputs in caller order."
goal: Look up the arguments, sections and examples of biomcp get article.
reader: A BioMCP user or agent checking one command
sidebar:
  label: get article
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 command `biomcp get article`.

Get one to twenty article inputs in caller order.

## Usage

```text
biomcp get article [OPTIONS] <input...>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--section` | array | no |  | `annotations`, `fulltext`, `supplements`, `tldr`, `citations`, `indexing`, `pdf`, `all` | Requested section words; trailing words and --section share this field. |
| `input` (position 1) | array | yes | min 1, max 20 |  | PubMed, PMC or DOI identifiers in input order. |

## Sections

Requested section words; trailing words and --section share this field.

| Section | Description |
| --- | --- |
| `annotations` | Normalized text-mined entities from PubTator3. |
| `fulltext` | Licensed PMC body outline. |
| `supplements` | Supplementary files confirmed by PMC metadata. |
| `tldr` | Machine-written Semantic Scholar summary and counts. |
| `citations` | Citation counts from NIH iCite. |
| `indexing` | MeSH, keywords and author indexing from the citation. |
| `pdf` | Publisher PDF link without downloading. |
| `all` | Every landed article section. |

## Examples

```sh
biomcp get article 33123537
biomcp get article PPR1295425 fulltext
```

## Reply schema

The help examples of this command return [`biomcp.article_cards/3`](/reference/schemas/biomcp-article-cards-3/). Print the full JSON schema with `biomcp schema biomcp.article_cards/3`.

## Exit codes

See [exit codes](/reference/error-codes/). Options that apply to every command are in [global options](/reference/global-options/).

## Next

The [article entity](/user-guide/article/) page lists every article command and links to a worked guide.

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
