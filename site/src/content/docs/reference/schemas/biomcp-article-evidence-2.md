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

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp article citation-evidence`](/reference/commands/article-citation-evidence/) replies with `schema` set to `biomcp.article_evidence/2`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp article citation-evidence 33123537 17611497
biomcp article citation-evidence 33123537 17611497 --fulltext
```

`biomcp schema biomcp.article_evidence/2` prints the full JSON Schema.

## Example reply

`biomcp article citation-evidence 33123537 17611497` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

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
            {
              "position": 1,
              "character_offset": 0,
              "text": "Mutations in the PH structural domain of AKT1(E17K) inc…"
            },
            {
              "position": 2,
              "character_offset": 0,
              "text": "In contrast, AKT1(E17K) localizes the protein to the pl…"
            },
            … 4 more
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
    {
      "source": "semantic-scholar",
      "use": "noncommercial",
      "terms": "Semantic Scholar API License Agreement",
      "terms_url": "https://api.semanticscholar.org/license/",
      "state": "ok"
    }
  ]
}
```

## Fields

`result` holds [ArticleEvidenceResult](#articleevidenceresult). `query` holds [ArticleEvidence](#articleevidence). `query` repeats the request as BioMCP read it.

### ArticleEvidenceResult

Independent graph and optional open-text evidence parts.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `fulltext` | [Section](/reference/schemas/types/#section) of [CitationParagraphs](#citationparagraphs) or null | no | The citing paper's own explicit reference paragraphs. |
| `semantic_scholar` | [Section](/reference/schemas/types/#section) of [CitationContexts](#citationcontexts) | yes | Source-reported citation contexts. |

### ArticleEvidence

Read every source context and optional explicit full-text citation paragraph.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `cited` | [ArticleInput](/reference/schemas/types/#articleinput) | yes | Cited PMID, PMC identity or DOI. |
| `citing` | [ArticleInput](/reference/schemas/types/#articleinput) | yes | Citing PMID, PMC identity or DOI. |
| `fulltext` | boolean | no | Include the citing paper's own explicit reference paragraphs. |
| `offset` | integer (uint32) | no | Context fragment position returned by the advancing step. |
| `paragraph_offset` | integer (uint32) | no | Full-text fragment position returned by the advancing step. |
| `pool` | [Text](/reference/schemas/types/#text) | no | Stored checked source scan and original hit positions. |

### CitationContexts

Source edge metadata and a bounded advancing context page.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `complete` | boolean | yes | Completion means a terminal source page was reached. |
| `contexts` | [Capped](/reference/schemas/types/#capped) of [EvidenceContext](#evidencecontext) | yes | Every context remains accessible through the advancing step. |
| `influential` | boolean or null | no | Missing is distinct from false. |
| `intents` | list of [CitationIntent](/reference/schemas/types/#citationintent) or null | no | Source-reported citation intents. |
| `notice` | [EvidenceNotice](/reference/schemas/types/#evidencenotice) or null | no | Successful empty-source explanation. |
| `source_cursor` | integer or null | no | Native next source offset when a scan is incomplete. |

### CitationParagraphs

Exact JATS reference paragraphs independent of the graph result.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `license` | [License](/reference/schemas/types/#license) | yes | Original publisher license category. |
| `paragraphs` | [Capped](/reference/schemas/types/#capped) of [EvidenceParagraph](#evidenceparagraph) | yes | Original paragraph fragments and complete fragment count. |

### EvidenceContext

One advancing piece of an original source citation context.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `character_offset` | integer (uint32) | yes | Unicode character position within that context. |
| `position` | integer (uint32) | yes | One-based original source context position. |
| `text` | string | yes | Whole context, or an exact advancing fragment when the context exceeds a page. |

### EvidenceParagraph

One source paragraph fragment with its own exact outline location.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `character_offset` | integer (uint32) | yes | Unicode character position within the original paragraph. |
| `markdown` | string (markdown) | yes | Preserved source Markdown. |
| `part` | integer (uint32) | yes | Exact semantic outline part. |
| `position` | integer (uint32) | yes | One-based original body paragraph position. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp article citation-evidence`](/reference/commands/article-citation-evidence/) gives the command's arguments.
