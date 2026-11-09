---
title: Reply of biomcp article citation-evidence (biomcp.article-evidence/6)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp article
  citation-evidence: the fields of schema biomcp.article-evidence/6."
goal: "Parse the biomcp.article-evidence/6 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.article-evidence/6
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp article citation-evidence`](/reference/commands/article-citation-evidence/) and [`biomcp article`](/reference/commands/article/) reply with `schema` set to `biomcp.article-evidence/6`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.article-evidence/6` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_ArticleEvidenceResult](#reply_articleevidenceresult). `query` holds [Reply_ArticleEvidence](#reply_articleevidence). `query` repeats the request as BioMCP read it.

### Reply_ArticleEvidenceResult

Independent graph and optional open-text evidence parts.

| Field | Type | Meaning |
| --- | --- | --- |
| `fulltext` | [Reply_Section_CitationParagraphs](#reply_section_citationparagraphs) or null | The citing paper's own explicit reference paragraphs. Optional. |
| `semantic_scholar` | [Reply_Section_CitationContexts](#reply_section_citationcontexts) | Source-reported citation contexts. |

### Reply_ArticleEvidence

Read every source context and optional explicit full-text citation paragraph.

| Field | Type | Meaning |
| --- | --- | --- |
| `cited` | [Reply_ArticleInput](/reference/schemas/types/#reply_articleinput) | Cited PMID, PMC identity or DOI. |
| `citing` | [Reply_ArticleInput](/reference/schemas/types/#reply_articleinput) | Citing PMID, PMC identity or DOI. |
| `fulltext` | boolean | Include the citing paper's own explicit reference paragraphs. Optional. |
| `offset` | integer (uint32) | Context fragment position returned by the advancing step. Optional. |
| `paragraph_offset` | integer (uint32) | Full-text fragment position returned by the advancing step. Optional. |
| `pool` | [Reply_Text](/reference/schemas/types/#reply_text) | Stored checked source scan and original hit positions. Optional. |

### Other types in this reply

8 more types appear only in this reply: [Reply_Capped_EvidenceContext](#reply_capped_evidencecontext), [Reply_Capped_EvidenceParagraph](#reply_capped_evidenceparagraph), [Reply_CitationContexts](#reply_citationcontexts), [Reply_CitationParagraphs](#reply_citationparagraphs), [Reply_EvidenceContext](#reply_evidencecontext), [Reply_EvidenceParagraph](#reply_evidenceparagraph), [Reply_Section_CitationContexts](#reply_section_citationcontexts), [Reply_Section_CitationParagraphs](#reply_section_citationparagraphs). Open the list below to read their fields.

<details>
<summary>Fields of the 8 other types</summary>

### Reply_Capped_EvidenceContext

A list cut to a maximum length. `total` counts the items before the cut.

| Field | Type | Meaning |
| --- | --- | --- |
| `items` | list of [Reply_EvidenceContext](#reply_evidencecontext) | The items, in the source's order. Optional. |
| `total` | integer (uint64) | How many distinct items there were before the cut. |

### Reply_Capped_EvidenceParagraph

A list cut to a maximum length. `total` counts the items before the cut.

| Field | Type | Meaning |
| --- | --- | --- |
| `items` | list of [Reply_EvidenceParagraph](#reply_evidenceparagraph) | The items, in the source's order. Optional. |
| `total` | integer (uint64) | How many distinct items there were before the cut. |

### Reply_CitationContexts

Source edge metadata and a bounded advancing context page.

| Field | Type | Meaning |
| --- | --- | --- |
| `complete` | boolean | Completion means a terminal source page was reached. |
| `contexts` | [Reply_Capped_EvidenceContext](#reply_capped_evidencecontext) | Every context remains accessible through the advancing step. |
| `influential` | boolean or null | Missing is distinct from false. Optional. |
| `intents` | list of [Reply_CitationIntent](/reference/schemas/types/#reply_citationintent) or null | Source-reported citation intents. Optional. |
| `license` | [Reply_License](/reference/schemas/types/#reply_license) or null | Record-owned license of the citing paper when independently checked. Optional. |
| `notice` | [Reply_EvidenceNotice](/reference/schemas/types/#reply_evidencenotice) or null | Successful empty-source explanation. Optional. |
| `source_cursor` | integer or null | Native next source offset when a scan is incomplete. Optional. |
| `text_withheld` | [Reply_WithheldArticleText](/reference/schemas/types/#reply_withheldarticletext) or null | The edge remains present when its text cannot be returned. Optional. |

### Reply_CitationParagraphs

Exact JATS reference paragraphs independent of the graph result.

| Field | Type | Meaning |
| --- | --- | --- |
| `license` | [Reply_License](/reference/schemas/types/#reply_license) | Original publisher license category. |
| `paragraphs` | [Reply_Capped_EvidenceParagraph](#reply_capped_evidenceparagraph) | Original paragraph fragments and complete fragment count. |

### Reply_EvidenceContext

One advancing piece of an original source citation context.

| Field | Type | Meaning |
| --- | --- | --- |
| `character_offset` | integer (uint32) | Unicode character position within that context. |
| `position` | integer (uint32) | One-based original source context position. |
| `text` | string | Whole context, or an exact advancing fragment when the context exceeds a page. |

### Reply_EvidenceParagraph

One source paragraph fragment with its own exact outline location.

| Field | Type | Meaning |
| --- | --- | --- |
| `character_offset` | integer (uint32) | Unicode character position within the original paragraph. |
| `markdown` | string (markdown) | Preserved source Markdown. |
| `part` | integer (uint32) | Exact semantic outline part. |
| `position` | integer (uint32) | One-based original body paragraph position. |

### Reply_Section_CitationContexts

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_CitationContexts](#reply_citationcontexts)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply_sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_Section_CitationParagraphs

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_CitationParagraphs](#reply_citationparagraphs)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply_sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

</details>

## Next

[`biomcp article citation-evidence`](/reference/commands/article-citation-evidence/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
