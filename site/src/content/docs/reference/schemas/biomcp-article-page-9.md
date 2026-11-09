---
title: Reply of biomcp search article (biomcp.article-page/9)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp search article: the
  fields of schema biomcp.article-page/9."
goal: "Parse the biomcp.article-page/9 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.article-page/9
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search article`](/reference/commands/search-article/) and [`biomcp search`](/reference/commands/search/) reply with `schema` set to `biomcp.article-page/9`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.article-page/9` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_ArticlePage](#reply-articlepage). `query` holds [Reply_ArticleSearch](#reply-articlesearch). `query` repeats the request as BioMCP read it.

### Reply_ArticlePage

A slice of the stored article source order.

| Field | Type | Meaning |
| --- | --- | --- |
| `candidates` | list of [Reply_LitVarCandidate](#reply-litvarcandidate) | Nearby source candidates when an exact variant was not uniquely selected. Optional. |
| `items` | list of [Reply_ArticleItem](/reference/schemas/types/#reply-articleitem) | Original ordered items. Optional. |

### Reply_ArticleSearch

Checked search selectors and pool cursor.

| Field | Type | Meaning |
| --- | --- | --- |
| `author` | [Reply_Text](/reference/schemas/types/#reply-text) | PubMed/Europe PMC author words. PubMed and Europe PMC only: author words sent to the source's author field. Optional. |
| `disease` | [Reply_Text](/reference/schemas/types/#reply-text) | Disease words. Available for command sources except LitVar. Optional. |
| `drug` | [Reply_Text](/reference/schemas/types/#reply-text) | Drug words. Available for command sources except LitVar. Optional. |
| `exact_term` | boolean | Search the typed drug term without resolving its other names. Optional. |
| `gene` | [Reply_GeneInput](/reference/schemas/types/#reply-geneinput) | Gene symbol or ID. Gene input; PubMed resolves it through HGNC. Optional. |
| `journal` | [Reply_Text](/reference/schemas/types/#reply-text) | PubMed/Europe PMC journal words. PubMed and Europe PMC only: journal words sent to the source's journal field. Optional. |
| `limit` | integer (uint32) | Maximum page length. Optional. |
| `no_preprints` | boolean | Exclude preprints. PubMed and Europe PMC only: exclude preprints using the source's native filter. Optional. |
| `offset` | integer (uint32) | Number of pool items to skip. Optional. |
| `open_access` | boolean | Native open access. PubMed, Europe PMC and Semantic Scholar only: require native open access; Semantic Scholar means a public PDF. Optional. |
| `pool` | [Reply_Text](/reference/schemas/types/#reply-text) | Stable result snapshot. Stable snapshot from an earlier response. Optional. |
| `retracted` | [Reply_RetractionFilter](#reply-retractionfilter) | Retraction filter. Include retracted papers by default. Exclusion is supported only by PubMed and Europe PMC. Optional. |
| `since` | [Reply_PartialDate](/reference/schemas/types/#reply-partialdate) | First publication date. PubMed, Europe PMC and Semantic Scholar only: inclusive lower publication-date bound; PubMed matches electronic or print date. Optional. |
| `sort` | [Reply_ArticleSort](#reply-articlesort) | rank, date or citations. rank (default): native provider order; several sources take turns; native search cost. date: native search cost; PubMed means date added and Europe PMC means newest first; other sources refuse it. citations: most cited first across the whole pool; adds iCite calls, about 1 s per 1,000 PMIDs. All sources support citations. Optional. |
| `source` | list of [Reply_ArticleSource](#reply-articlesource) | Sources; default pubmed. Sources to search in caller order, comma separated. Default: pubmed. Several sources take turns, remove duplicates and show every native rank. pubmed: keywords with PubMed MeSH mapping; dropped words appear as notices. europepmc: words in open full text and preprints; incidental mentions are possible. litvar: exact variant publications; requires --variant and a protein change needs --gene; unresolved inputs return candidates. semantic-scholar: broad discovery; a shared keyless pool may return 429; an optional free key raises the limit. litsense: sentence resemblance, not a match; unrelated hits and timeouts are possible. pubtator: entity and word search including full text; needs the right entity ID; --gene uses HGNC Entrez identity and cannot run without that join. Optional. |
| `text` | [Reply_Text](/reference/schemas/types/#reply-text) | Source search words. Words sent to each named source in its default fields; unsupported by LitVar, which requires a variant selector. Optional. |
| `type` | [Reply_ArticleType](#reply-articletype) | Publication type. PubMed and Europe PMC only: native publication-type filter. Optional. |
| `until` | [Reply_PartialDate](/reference/schemas/types/#reply-partialdate) | Last publication date. PubMed, Europe PMC and Semantic Scholar only: inclusive upper publication-date bound; PubMed matches electronic or print date. Optional. |
| `variant` | [Reply_Text](/reference/schemas/types/#reply-text) | Variant words. Optional. |

### Other types in this reply

5 more types appear only in this reply: [Reply_ArticleSort](#reply-articlesort), [Reply_ArticleSource](#reply-articlesource), [Reply_ArticleType](#reply-articletype), [Reply_LitVarCandidate](#reply-litvarcandidate), [Reply_RetractionFilter](#reply-retractionfilter). Open the list below to read their fields.

<details>
<summary>Fields of the 5 other types</summary>

### Reply_ArticleSort

Article ordering supported by this command.

Values: `rank`, `date`, `citations`.

### Reply_ArticleSource

Source selectors supported by article search.

Values: `pubmed`, `europepmc`, `litvar`, `semantic-scholar`, `litsense`, `pubtator`.

### Reply_ArticleType

Publication-type selectors sent to the named sources.

Values: `trial`, `randomized`, `review`, `systematic-review`, `meta-analysis`, `guideline`, `case-report`.

### Reply_LitVarCandidate

Public LitVar2 autocomplete candidate, without a clinical ranking claim.

| Field | Type | Meaning |
| --- | --- | --- |
| `count` | integer or null | Number of associated publications reported by LitVar2, without inferred totals. Optional. |
| `gene` | list of [Reply_Text](/reference/schemas/types/#reply-text) | Source gene symbols. Optional. |
| `name` | [Reply_Text](/reference/schemas/types/#reply-text) or null | Source variant name, when independently admitted. Optional. |
| `rsid` | [Reply_Text](/reference/schemas/types/#reply-text) | Source rsID, when the source associates one with this variant. Optional. |

### Reply_RetractionFilter

Native retraction inclusion policy.

Values: `include`, `exclude`.

</details>

## Next

[`biomcp search article`](/reference/commands/search-article/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
