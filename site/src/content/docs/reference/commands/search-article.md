---
title: biomcp search article
description: "Reference for the BioMCP 1.0.0 command biomcp search article:
  Checked search selectors and pool cursor."
goal: Look up the arguments, sections and examples of biomcp search article.
reader: A BioMCP user or agent checking one command
sidebar:
  label: search article
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 command `biomcp search article`.

Checked search selectors and pool cursor.

## Usage

```text
biomcp search article [OPTIONS] [text]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--author` | string | no |  |  | Author words sent to PubMed's author field. |
| `--disease` | string | no |  |  | Disease words. |
| `--drug` | string | no |  |  | Drug words. |
| `--exclude-retracted` | boolean | no |  |  | Include retracted papers by default, or exclude them using a native filter. |
| `--gene` | string | no |  |  | Gene input; PubMed resolves it through HGNC. |
| `--include-retracted` | boolean | no |  |  | Include retracted papers by default, or exclude them using a native filter. |
| `--journal` | string | no |  |  | Journal words sent to PubMed's journal field. |
| `--limit` | integer | no | min 1, max 50 |  | Maximum page length. |
| `--no-preprints` | boolean | no |  |  | Exclude preprints using the source's native filter. |
| `--offset` | integer | no | min 0 |  | Number of pool items to skip. |
| `--open-access` | boolean | no |  |  | Require the source's native open-access filter; Semantic Scholar means a public PDF. |
| `--pool` | string | no |  |  | Stable snapshot from an earlier response. |
| `--since` | string | no |  |  | Inclusive lower publication-date bound at the stated precision; PubMed matches either electronic or print date. |
| `--sort` | string | no |  | `rank`, `date`, `citations` | rank (default): native provider order; several sources take turns; native search cost. date: native search cost; PubMed means date added and Europe PMC means newest first; other sources refuse it. citations: most cited first across the whole pool; adds iCite calls, about 1 s per 1,000 PMIDs. All sources support citations. |
| `--source` | array | no | min 1 | `pubmed`, `europepmc`, `litvar`, `semantic-scholar`, `litsense`, `pubtator` | Sources to search in caller order, comma separated. Default: pubmed. Several sources take turns, remove duplicates and show every native rank. pubmed: keywords with PubMed MeSH mapping; dropped words appear as notices. europepmc: words in open full text and preprints; incidental mentions are possible. litvar: exact variant publications; requires --variant and a protein change needs --gene; unresolved inputs return candidates. semantic-scholar: broad discovery; a shared keyless pool may return 429; an optional free key raises the limit. litsense: sentence resemblance, not a match; unrelated hits and timeouts are possible. pubtator: entity and word search including full text; needs the right entity ID; --gene uses HGNC Entrez identity and cannot run without that join. |
| `--type` | string | no |  | `trial`, `randomized`, `review`, `systematic_review`, `meta_analysis`, `guideline`, `case_report` | Publication-type filter. |
| `--until` | string | no |  |  | Inclusive upper publication-date bound at the stated precision; PubMed matches either electronic or print date. |
| `--variant` | string | no |  |  | Variant words. |
| `text` (position 1) | string | no |  |  | Words sent to each named source in its default fields. |

## Sources

`--source` accepts these sources: [pubmed](/sources/pubmed/), [europepmc](/sources/europepmc/), [litvar](/sources/litvar/), [semantic-scholar](/sources/semantic-scholar/), [litsense](/sources/litsense/), [pubtator](/sources/pubtator/).

## Examples

```sh
biomcp search article 'AKT1 E17K'
biomcp search article 'KRAS G12C sotorasib' --since 2024 --sort date --limit 5
```

## Exit codes

See [exit codes](/reference/error-codes/). Options that apply to every command are in [global options](/reference/global-options/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
