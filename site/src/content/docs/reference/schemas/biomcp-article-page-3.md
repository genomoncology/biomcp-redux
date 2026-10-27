---
title: biomcp.article_page/3
description: "The BioMCP 1.0.0 reply schema biomcp.article_page/3: its envelope
  fields, its result type ArticlePage and the commands that return it."
goal: Look up the fields of a biomcp.article_page/3 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.article_page/3
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.article_page/3`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.article_page/3`.

These commands return it: [`biomcp search article`](/reference/commands/search-article/).

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | ArticleSearch | yes | Parsed query. |
| `result` | ArticlePage or null | no | Answer when available. |
| `schema` | "biomcp.article_page/3" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: ArticlePage

A slice of the stored article source order.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `candidates` | array of LitVarCandidate | no | Nearby source candidates when an exact variant was not uniquely selected. |
| `items` | array of ArticleItem | yes | Original ordered items. |

## Query: ArticleSearch

Checked search selectors and pool cursor.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `author` | Text | no | Author words sent to PubMed's author field. |
| `disease` | Text | no | Disease words. |
| `drug` | Text | no | Drug words. |
| `gene` | GeneInput | no | Gene input; PubMed resolves it through HGNC. |
| `journal` | Text | no | Journal words sent to PubMed's journal field. |
| `limit` | integer (uint32) | no | Maximum page length. |
| `no_preprints` | boolean | no | Exclude preprints using the source's native filter. |
| `offset` | integer (uint32) | no | Number of pool items to skip. |
| `open_access` | boolean | no | Require the source's native open-access filter; Semantic Scholar means a public PDF. |
| `pool` | Text | no | Stable snapshot from an earlier response. |
| `retracted` | RetractionFilter | no | Include retracted papers by default, or exclude them using a native filter. |
| `since` | PartialDate | no | Inclusive lower publication-date bound at the stated precision; PubMed matches either electronic or print date. |
| `sort` | ArticleSort | no | rank (default): native provider order; several sources take turns; native search cost. date: native search cost; PubMed means date added and Europe PMC means newest first; other sources refuse it. citations: most cited first across the whole pool; adds iCite calls, about 1 s per 1,000 PMIDs. All sources support citations. |
| `source` | array of ArticleSource | no | Sources to search in caller order, comma separated. Default: pubmed. Several sources take turns, remove duplicates and show every native rank. pubmed: keywords with PubMed MeSH mapping; dropped words appear as notices. europepmc: words in open full text and preprints; incidental mentions are possible. litvar: exact variant publications; requires --variant and a protein change needs --gene; unresolved inputs return candidates. semantic-scholar: broad discovery; a shared keyless pool may return 429; an optional free key raises the limit. litsense: sentence resemblance, not a match; unrelated hits and timeouts are possible. pubtator: entity and word search including full text; needs the right entity ID; --gene uses HGNC Entrez identity and cannot run without that join. |
| `text` | Text | no | Words sent to each named source in its default fields. |
| `type` | ArticleType | no | Publication-type filter. |
| `until` | PartialDate | no | Inclusive upper publication-date bound at the stated precision; PubMed matches either electronic or print date. |
| `variant` | Text | no | Variant words. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.article_page/3` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
