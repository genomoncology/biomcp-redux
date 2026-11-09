---
title: biomcp search article
description: "The BioMCP 1.0.0 command biomcp search article: Finds articles by
  text, gene, disease, drug, variant, author, journal or date."
goal: Look up the arguments, sections and examples of biomcp search article.
reader: A BioMCP user or agent checking one command
sidebar:
  label: search article
kind: reference
verified: 2026-11-09
---

`biomcp search article` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It finds articles by text, gene, disease, drug, variant, author, journal or date. This page lists its arguments, examples and reply.

The program's help describes it:

> Search research articles and biomedical papers. Sources: europepmc, litsense, litvar, pubmed, pubtator, semantic-scholar.

## Usage

```text
biomcp search article [OPTIONS] [text]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--author` | string | no |  |  | PubMed/Europe PMC author words. PubMed and Europe PMC only: author words sent to the source's author field. Value form: string. Example: --author Carpten. Command sources: europepmc, litsense, litvar, pubmed, pubtator, semantic-scholar; source restrictions in the field description apply. |
| `--disease` | string | no |  |  | Disease words. Available for command sources except LitVar. Value form: string. Example: --disease meningioma. Command sources: europepmc, litsense, litvar, pubmed, pubtator, semantic-scholar; source restrictions in the field description apply. |
| `--drug` | string | no |  |  | Drug words. Available for command sources except LitVar. Value form: string. Example: --drug 'drug words'. Command sources: europepmc, litsense, litvar, pubmed, pubtator, semantic-scholar; source restrictions in the field description apply. |
| `--exact-term` | boolean | no |  |  | Search the typed drug term without resolving its other names. Value form: boolean. Example: --exact-term. Command sources: europepmc, litsense, litvar, pubmed, pubtator, semantic-scholar; source restrictions in the field description apply. |
| `--exclude-retracted` | boolean | no |  |  | Retraction filter; help search article Retraction filter. Include retracted papers by default. Exclusion is supported only by PubMed and Europe PMC. Value form: boolean. Example: --exclude-retracted. Command sources: europepmc, litsense, litvar, pubmed, pubtator, semantic-scholar; source restrictions in the field description apply. |
| `--gene` | string | no |  |  | Gene symbol or ID. Gene input; PubMed resolves it through HGNC. Value form: string. Example: --gene NF2. Command sources: europepmc, litsense, litvar, pubmed, pubtator, semantic-scholar; source restrictions in the field description apply. |
| `--include-retracted` | boolean | no |  |  | Retraction filter; help search article Retraction filter. Include retracted papers by default. Exclusion is supported only by PubMed and Europe PMC. Value form: boolean. Example: --include-retracted. Command sources: europepmc, litsense, litvar, pubmed, pubtator, semantic-scholar; source restrictions in the field description apply. |
| `--journal` | string | no |  |  | PubMed/Europe PMC journal words. PubMed and Europe PMC only: journal words sent to the source's journal field. Value form: string. Example: --journal 'journal words'. Command sources: europepmc, litsense, litvar, pubmed, pubtator, semantic-scholar; source restrictions in the field description apply. |
| `--limit` | integer | no | min 1, max 50 |  | Maximum page length. Value form: integer. Example: --limit 5. Command sources: europepmc, litsense, litvar, pubmed, pubtator, semantic-scholar; source restrictions in the field description apply. |
| `--no-preprints` | boolean | no |  |  | Exclude preprints. PubMed and Europe PMC only: exclude preprints using the source's source filter. Value form: boolean. Example: --no-preprints. Command sources: europepmc, litsense, litvar, pubmed, pubtator, semantic-scholar; source restrictions in the field description apply. |
| `--offset` | integer | no | min 0 |  | Number of pool items to skip. Value form: integer. Example: --offset 5. Command sources: europepmc, litsense, litvar, pubmed, pubtator, semantic-scholar; source restrictions in the field description apply. |
| `--open-access` | boolean | no |  |  | Source open access. PubMed, Europe PMC and Semantic Scholar only: require source open access; Semantic Scholar means a public PDF. Value form: boolean. Example: --open-access. Command sources: europepmc, litsense, litvar, pubmed, pubtator, semantic-scholar; source restrictions in the field description apply. |
| `--pool` | string | no |  |  | Stable result snapshot. Stable snapshot from an earlier response. Value form: string. Example: --pool &lt;pool&gt; (syntax; replace the placeholder with your retained value). Command sources: europepmc, litsense, litvar, pubmed, pubtator, semantic-scholar; source restrictions in the field description apply. |
| `--query` | string | no |  |  | Alias for positional article text. Value form: string. Example: --query &lt;query&gt; (syntax; replace the placeholder with your retained value). Command sources: europepmc, litsense, litvar, pubmed, pubtator, semantic-scholar; source restrictions in the field description apply. |
| `--since` | string | no |  |  | First publication date. PubMed, Europe PMC and Semantic Scholar only: inclusive lower publication-date bound; PubMed matches electronic or print date. Value form: string. Example: --since 2024. Command sources: europepmc, litsense, litvar, pubmed, pubtator, semantic-scholar; source restrictions in the field description apply. |
| `--sort` | string | no |  | `rank`, `date`, `citations` | rank, date or citations; help search article rank, date or citations. rank (default): source order; several sources take turns; source search cost. date: source search cost; PubMed means date added and Europe PMC means newest first; other sources refuse it. citations: most cited first across the whole pool; adds iCite calls, about 1 s per 1,000 PMIDs. All sources support citations. Listed choices accept any letter case; replies use their listed spelling. Value form: string. Example: --sort date. Command sources: europepmc, litsense, litvar, pubmed, pubtator, semantic-scholar; source restrictions in the field description apply. |
| `--source` | array | no | min 1 | `pubmed`, `europepmc`, `litvar`, `semantic-scholar`, `litsense`, `pubtator` | Sources; default pubmed; help search article Sources; default pubmed. Sources to search in caller order, comma separated. Default: pubmed. Several sources take turns, remove duplicates and show every source rank. pubmed: keywords with PubMed MeSH mapping; dropped words appear as notices. europepmc: words in open full text and preprints; incidental mentions are possible. litvar: exact variant publications; requires --variant and a protein change needs --gene; unresolved inputs return candidates. semantic-scholar: broad discovery; a shared keyless pool may return 429; an optional free key raises the limit. litsense: sentence resemblance, not a match; unrelated hits and timeouts are possible. pubtator: entity and word search including full text; needs the right entity ID; --gene uses HGNC Entrez identity and cannot run without that join. Listed choices accept any letter case; replies use their listed spelling. Value form: repeatable values (repeat the flag). Example: --source europepmc. Command sources: europepmc, litsense, litvar, pubmed, pubtator, semantic-scholar; source restrictions in the field description apply. |
| `--type` | string | no |  | `trial`, `randomized`, `review`, `systematic-review`, `meta-analysis`, `guideline`, `case-report` | Publication type; help search article Publication type. PubMed and Europe PMC only: source publication-type filter. Listed choices accept any letter case; replies use their listed spelling. Value form: string. Example: --type review. Command sources: europepmc, litsense, litvar, pubmed, pubtator, semantic-scholar; source restrictions in the field description apply. |
| `--until` | string | no |  |  | Last publication date. PubMed, Europe PMC and Semantic Scholar only: inclusive upper publication-date bound; PubMed matches electronic or print date. Value form: string. Example: --until 2020. Command sources: europepmc, litsense, litvar, pubmed, pubtator, semantic-scholar; source restrictions in the field description apply. |
| `--variant` | string | no |  |  | Variant words. Value form: string. Example: --variant rs121434592. Command sources: europepmc, litsense, litvar, pubmed, pubtator, semantic-scholar; source restrictions in the field description apply. |
| `text` (position 1) | string | no |  |  | Source search words. Words sent to each named source in its default fields; unsupported by LitVar, which requires a variant selector. |

## Sources

`--source` picks the source to ask: [pubmed](/sources/pubmed/), [europepmc](/sources/europepmc/), [litvar](/sources/litvar/), [semantic-scholar](/sources/semantic-scholar/), [litsense](/sources/litsense/), [pubtator](/sources/pubtator/).

## Examples

The program's help gives these examples.

```sh
biomcp search article 'AKT1 E17K'
```

```sh
biomcp search article 'KRAS G12C sotorasib' --since 2024 --sort date --limit 5
```

```sh
biomcp search article 'AKT1 E17K' --source europepmc --open-access \
  --no-preprints --exclude-retracted
```

```sh
biomcp search article 'AKT1 E17K' --include-retracted
```

```sh
biomcp search article 'KRAS G12C sotorasib' --since 2024 --sort date --limit 5 \
  --offset 5
```

```sh
biomcp search article AKT1 --author Carpten --source pubmed,europepmc
```

```sh
biomcp search article --variant rs121434592 --source litvar --sort citations
```

```sh
biomcp search article words --gene NF2 --variant K322 --disease meningioma \
  --drug 'drug words' --author 'author words' --journal 'journal words' \
  --since 2000 --until 2020 --type review --limit 7
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.article-page/9`](/reference/schemas/biomcp-article-page-9/), whose page lists its fields.

## Next

The [article guide](/user-guide/article/) shows every article command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
