---
title: biomcp get article
description: "The BioMCP 1.0.0 command biomcp get article: Reads one to twenty
  articles by PMID, PMCID or DOI, and optionally sections such as the full
  text."
goal: Look up the arguments, sections and examples of biomcp get article.
reader: A BioMCP user or agent checking one command
sidebar:
  label: get article
kind: reference
verified: 2026-11-09
---

`biomcp get article` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It reads one to twenty articles by PMID, PMCID or DOI, and optionally sections such as the full text. A [section word](/concepts/progressive-disclosure/) after the identifier adds one more part of the record to the reply. This page lists its arguments, sections, examples and reply.

The program's help describes it:

> Get research article details. Sources: europepmc, icite, pmc-oa, pubmed, pubtator, semantic-scholar.

## Usage

```text
biomcp get article [OPTIONS] <input...>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--section` | array | no |  | `annotations`, `fulltext`, `supplements`, `tldr`, `citations`, `indexing`, `pdf`, `all` | Requested section words; trailing words and --section share; help get article Listed choices accept any letter case; replies use their listed spelling. Value form: repeatable values (repeat the flag). Example: --section annotations. Command sources: europepmc, icite, pmc-oa, pubmed, pubtator, semantic-scholar; source restrictions in the field description apply. |
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

The program's help gives these examples.

```sh
biomcp get article 33123537
```

```sh
biomcp get article PPR1295425 fulltext
```

```sh
biomcp get article PPR1295425 all --disable-sources semantic-scholar
```

```sh
biomcp get article 33123537 --section annotations --section annotations
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.article-cards/9`](/reference/schemas/biomcp-article-cards-9/), whose page lists its fields.

## Next

The [article guide](/user-guide/article/) shows every article command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
