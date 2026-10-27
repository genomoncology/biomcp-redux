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

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search article`](/reference/commands/search-article/) replies with `schema` set to `biomcp.article_page/3`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp search article --drug daraxonrasib --limit 3
biomcp search article 'KRAS G12C sotorasib' --since 2024 --sort date --limit 3
```

`biomcp schema biomcp.article_page/3` prints the full JSON Schema.

## Example reply

`biomcp search article --drug daraxonrasib --limit 3` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

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
      {
        "key": {"namespace": "pubmed", "id": "40056080"},
        "pmid": "40056080",
        "header": {…}
      },
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
  "next": [
    {
      "kind": "page",
      "args": [
        "search", "article", "--drug", "daraxonrasib", "--limit", "3",
        "--offset", "3",
        … 2 more
      ]
    },
    … 2 more
  ]
}
```

## Fields

`result` holds [ArticlePage](#articlepage). `query` holds [ArticleSearch](#articlesearch). `query` repeats the request as BioMCP read it.

### ArticlePage

A slice of the stored article source order.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `candidates` | list of [LitVarCandidate](#litvarcandidate) | no | Nearby source candidates when an exact variant was not uniquely selected. |
| `items` | list of [ArticleItem](/reference/schemas/types/#articleitem) | yes | Original ordered items. |

### ArticleSearch

Checked search selectors and pool cursor.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `author` | [Text](/reference/schemas/types/#text) | no | Author words sent to PubMed's author field. |
| `disease` | [Text](/reference/schemas/types/#text) | no | Disease words. |
| `drug` | [Text](/reference/schemas/types/#text) | no | Drug words. |
| `gene` | [GeneInput](/reference/schemas/types/#geneinput) | no | Gene input; PubMed resolves it through HGNC. |
| `journal` | [Text](/reference/schemas/types/#text) | no | Journal words sent to PubMed's journal field. |
| `limit` | integer (uint32) | no | Maximum page length. |
| `no_preprints` | boolean | no | Exclude preprints using the source's native filter. |
| `offset` | integer (uint32) | no | Number of pool items to skip. |
| `open_access` | boolean | no | Require the source's native open-access filter; Semantic Scholar means a public PDF. |
| `pool` | [Text](/reference/schemas/types/#text) | no | Stable snapshot from an earlier response. |
| `retracted` | [RetractionFilter](#retractionfilter) | no | Include retracted papers by default, or exclude them using a native filter. |
| `since` | [PartialDate](/reference/schemas/types/#partialdate) | no | Inclusive lower publication-date bound at the stated precision; PubMed matches either electronic or print date. |
| `sort` | [ArticleSort](#articlesort) | no | rank (default): native provider order; several sources take turns; native search cost. date: native search cost; PubMed means date added and Europe PMC means newest first; other sources refuse it. citations: most cited first across the whole pool; adds iCite calls, about 1 s per 1,000 PMIDs. All sources support citations. |
| `source` | list of [ArticleSource](#articlesource) | no | Sources to search in caller order, comma separated. Default: pubmed. Several sources take turns, remove duplicates and show every native rank. pubmed: keywords with PubMed MeSH mapping; dropped words appear as notices. europepmc: words in open full text and preprints; incidental mentions are possible. litvar: exact variant publications; requires --variant and a protein change needs --gene; unresolved inputs return candidates. semantic-scholar: broad discovery; a shared keyless pool may return 429; an optional free key raises the limit. litsense: sentence resemblance, not a match; unrelated hits and timeouts are possible. pubtator: entity and word search including full text; needs the right entity ID; --gene uses HGNC Entrez identity and cannot run without that join. |
| `text` | [Text](/reference/schemas/types/#text) | no | Words sent to each named source in its default fields. |
| `type` | [ArticleType](#articletype) | no | Publication-type filter. |
| `until` | [PartialDate](/reference/schemas/types/#partialdate) | no | Inclusive upper publication-date bound at the stated precision; PubMed matches either electronic or print date. |
| `variant` | [Text](/reference/schemas/types/#text) | no | Variant words. |

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

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `count` | integer (uint64) | yes | Number of associated publications reported by LitVar2. |
| `gene` | list of [Text](/reference/schemas/types/#text) | yes | Source gene symbols. |
| `name` | [Text](/reference/schemas/types/#text) | yes | Source variant name. |
| `rsid` | [Text](/reference/schemas/types/#text) | yes | Source rsID, available for exact follow-up. |

### RetractionFilter

Native retraction inclusion policy.

Values: `include`, `exclude`.

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp search article`](/reference/commands/search-article/) gives the command's arguments.
