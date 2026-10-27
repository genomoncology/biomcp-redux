---
title: Reply of biomcp article citation-evidence (biomcp.article_evidence/2)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp article
  citation-evidence: a real example and the fields of schema
  biomcp.article_evidence/2."
goal: "Parse the biomcp.article_evidence/2 reply: see a real example and what
  each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.article_evidence/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp article citation-evidence`](/reference/commands/article-citation-evidence/) replies with `schema` set to `biomcp.article_evidence/2`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp article citation-evidence 33123537 17611497
biomcp article citation-evidence 33123537 17611497 --fulltext
```

`biomcp schema biomcp.article_evidence/2` prints the full JSON Schema.

## Example reply

`biomcp article citation-evidence 33123537 17611497` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.article_evidence/2",
  "status": "complete",
  "query": {
    "citing": "33123537",
    "cited": "17611497",
    "fulltext": false,
    "offset": 0,
    "paragraph_offset": 0
  },
  "result": {
    "semantic_scholar": {
      "state": "data",
      "source": "semantic-scholar",
      "data": {
        "contexts": {
          "items": [
            {…},
            … 5 more
          ],
          "total": 6
        },
        "complete": true,
        "intents": ["background"],
        "influential": true
      }
    }
  },
  "sources": [
    {
      "source": "pubmed",
      "use": "restricted",
      "terms": "NCBI policies; publisher rights in abstracts",
      "terms_url": "https://www.ncbi.nlm.nih.gov/home/about/policies/",
      "state": "ok"
    },
    … 1 more
  ]
}
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [ArticleEvidenceResult](#articleevidenceresult). `query` holds [ArticleEvidence](#articleevidence). `query` repeats the request as BioMCP read it.

### ArticleEvidenceResult

Independent graph and optional open-text evidence parts.

| Field | Type | Meaning |
| --- | --- | --- |
| `fulltext` | [Section](/reference/schemas/types/#section) of [CitationParagraphs](#citationparagraphs) or null | The citing paper's own explicit reference paragraphs. Optional. |
| `semantic_scholar` | [Section](/reference/schemas/types/#section) of [CitationContexts](#citationcontexts) | Source-reported citation contexts. |

### ArticleEvidence

Read every source context and optional explicit full-text citation paragraph.

| Field | Type | Meaning |
| --- | --- | --- |
| `cited` | [ArticleInput](/reference/schemas/types/#articleinput) | Cited PMID, PMC identity or DOI. |
| `citing` | [ArticleInput](/reference/schemas/types/#articleinput) | Citing PMID, PMC identity or DOI. |
| `fulltext` | boolean | Include the citing paper's own explicit reference paragraphs. Optional. |
| `offset` | integer (uint32) | Context fragment position returned by the advancing step. Optional. |
| `paragraph_offset` | integer (uint32) | Full-text fragment position returned by the advancing step. Optional. |
| `pool` | [Text](/reference/schemas/types/#text) | Stored checked source scan and original hit positions. Optional. |

### CitationContexts

Source edge metadata and a bounded advancing context page.

| Field | Type | Meaning |
| --- | --- | --- |
| `complete` | boolean | Completion means a terminal source page was reached. |
| `contexts` | [Capped](/reference/schemas/types/#capped) of [EvidenceContext](#evidencecontext) | Every context remains accessible through the advancing step. |
| `influential` | boolean or null | Missing is distinct from false. Optional. |
| `intents` | list of [CitationIntent](/reference/schemas/types/#citationintent) or null | Source-reported citation intents. Optional. |
| `notice` | [EvidenceNotice](/reference/schemas/types/#evidencenotice) or null | Successful empty-source explanation. Optional. |
| `source_cursor` | integer or null | Native next source offset when a scan is incomplete. Optional. |

### CitationParagraphs

Exact JATS reference paragraphs independent of the graph result.

| Field | Type | Meaning |
| --- | --- | --- |
| `license` | [License](/reference/schemas/types/#license) | Original publisher license category. |
| `paragraphs` | [Capped](/reference/schemas/types/#capped) of [EvidenceParagraph](#evidenceparagraph) | Original paragraph fragments and complete fragment count. |

### EvidenceContext

One advancing piece of an original source citation context.

| Field | Type | Meaning |
| --- | --- | --- |
| `character_offset` | integer (uint32) | Unicode character position within that context. |
| `position` | integer (uint32) | One-based original source context position. |
| `text` | string | Whole context, or an exact advancing fragment when the context exceeds a page. |

### EvidenceParagraph

One source paragraph fragment with its own exact outline location.

| Field | Type | Meaning |
| --- | --- | --- |
| `character_offset` | integer (uint32) | Unicode character position within the original paragraph. |
| `markdown` | string (markdown) | Preserved source Markdown. |
| `part` | integer (uint32) | Exact semantic outline part. |
| `position` | integer (uint32) | One-based original body paragraph position. |

## Next

[`biomcp article citation-evidence`](/reference/commands/article-citation-evidence/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
