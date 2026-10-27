---
title: Reply of biomcp search article (biomcp.article_page/3)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp search article: a
  real example and the fields of schema biomcp.article_page/3."
goal: "Parse the biomcp.article_page/3 reply: see a real example and what each
  field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.article_page/3
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search article`](/reference/commands/search-article/) replies with `schema` set to `biomcp.article_page/3`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp search article --drug daraxonrasib --limit 3
biomcp search article 'KRAS G12C sotorasib' --since 2024 --sort date --limit 3
```

`biomcp schema biomcp.article_page/3` prints the full JSON Schema.

## Example reply

`biomcp search article --drug daraxonrasib --limit 3` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.article_page/3",
  "status": "complete",
  "query": {
    "source": ["pubmed"],
    "sort": "rank",
    "drug": "daraxonrasib",
    "limit": 3,
    "offset": 0
  },
  "result": {
    "items": [
      {…},
      … 2 more
    ]
  },
  "page": {
    "offset": 0,
    "returned": 3,
    "total": 99,
    "pool": {
      "id": "d8dc7a27426e7d2d20cb523b5406ac8d",
      "size": 99,
      "created": 1793140291,
      "expires": 1793226691
    },
    "truncated": false
  },
  "sources": [
    {
      "source": "pubmed",
      "use": "restricted",
      "terms": "NCBI policies; publisher rights in abstracts",
      "terms_url": "https://www.ncbi.nlm.nih.gov/home/about/policies/",
      "state": "ok",
      "term": "daraxonrasib",
      "count": 99,
      "fetched": 99
    }
  ],
…
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [ArticlePage](#articlepage). `query` holds [ArticleSearch](#articlesearch). `query` repeats the request as BioMCP read it.

### ArticlePage

A slice of the stored article source order.

| Field | Type | Meaning |
| --- | --- | --- |
| `candidates` | list of [LitVarCandidate](#litvarcandidate) | Nearby source candidates when an exact variant was not uniquely selected. Optional. |
| `items` | list of [ArticleItem](/reference/schemas/types/#articleitem) | Original ordered items. |

### ArticleSearch

Checked search selectors and pool cursor.

| Field | Type | Meaning |
| --- | --- | --- |
| `author` | [Text](/reference/schemas/types/#text) | Author words sent to PubMed's author field. Optional. |
| `disease` | [Text](/reference/schemas/types/#text) | Disease words. Optional. |
| `drug` | [Text](/reference/schemas/types/#text) | Drug words. Optional. |
| `gene` | [GeneInput](/reference/schemas/types/#geneinput) | Gene input; PubMed resolves it through HGNC. Optional. |
| `journal` | [Text](/reference/schemas/types/#text) | Journal words sent to PubMed's journal field. Optional. |
| `limit` | integer (uint32) | Maximum page length. Optional. |
| `no_preprints` | boolean | Exclude preprints using the source's native filter. Optional. |
| `offset` | integer (uint32) | Number of pool items to skip. Optional. |
| `open_access` | boolean | Require the source's native open-access filter; Semantic Scholar means a public PDF. Optional. |
| `pool` | [Text](/reference/schemas/types/#text) | Stable snapshot from an earlier response. Optional. |
| `retracted` | [RetractionFilter](#retractionfilter) | Include retracted papers by default, or exclude them using a native filter. Optional. |
| `since` | [PartialDate](/reference/schemas/types/#partialdate) | Inclusive lower publication-date bound at the stated precision; PubMed matches either electronic or print date. Optional. |
| `sort` | [ArticleSort](#articlesort) | rank (default): native provider order; several sources take turns; native search cost. date: native search cost; PubMed means date added and Europe PMC means newest first; other sources refuse it. citations: most cited first across the whole pool; adds iCite calls, about 1 s per 1,000 PMIDs. All sources support citations. Optional. |
| `source` | list of [ArticleSource](#articlesource) | Sources to search in caller order, comma separated. Default: pubmed. Several sources take turns, remove duplicates and show every native rank. pubmed: keywords with PubMed MeSH mapping; dropped words appear as notices. europepmc: words in open full text and preprints; incidental mentions are possible. litvar: exact variant publications; requires --variant and a protein change needs --gene; unresolved inputs return candidates. semantic-scholar: broad discovery; a shared keyless pool may return 429; an optional free key raises the limit. litsense: sentence resemblance, not a match; unrelated hits and timeouts are possible. pubtator: entity and word search including full text; needs the right entity ID; --gene uses HGNC Entrez identity and cannot run without that join. Optional. |
| `text` | [Text](/reference/schemas/types/#text) | Words sent to each named source in its default fields. Optional. |
| `type` | [ArticleType](#articletype) | Publication-type filter. Optional. |
| `until` | [PartialDate](/reference/schemas/types/#partialdate) | Inclusive upper publication-date bound at the stated precision; PubMed matches either electronic or print date. Optional. |
| `variant` | [Text](/reference/schemas/types/#text) | Variant words. Optional. |

### ArticleSort

Article ordering supported by this command.

Values: `rank`, `date`, `citations`.

### ArticleSource

Source selectors supported by article search.

Values: `pubmed`, `europepmc`, `litvar`, `semantic-scholar`, `litsense`, `pubtator`.

### ArticleType

Publication-type selectors sent to the named sources.

Values: `trial`, `randomized`, `review`, `systematic_review`, `meta_analysis`, `guideline`, `case_report`.

### LitVarCandidate

Public LitVar2 autocomplete candidate, without a clinical ranking claim.

| Field | Type | Meaning |
| --- | --- | --- |
| `count` | integer (uint64) | Number of associated publications reported by LitVar2. |
| `gene` | list of [Text](/reference/schemas/types/#text) | Source gene symbols. |
| `name` | [Text](/reference/schemas/types/#text) | Source variant name. |
| `rsid` | [Text](/reference/schemas/types/#text) | Source rsID, available for exact follow-up. |

### RetractionFilter

Native retraction inclusion policy.

Values: `include`, `exclude`.

## Next

[`biomcp search article`](/reference/commands/search-article/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
