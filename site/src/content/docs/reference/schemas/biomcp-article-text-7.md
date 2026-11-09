---
title: Reply of biomcp article text (biomcp.article-text/7)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp article text: the
  fields of schema biomcp.article-text/7."
goal: "Parse the biomcp.article-text/7 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.article-text/7
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp article text`](/reference/commands/article-text/) and [`biomcp article`](/reference/commands/article/) reply with `schema` set to `biomcp.article-text/7`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.article-text/7` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_ArticleTextResult](#reply_articletextresult). `query` holds [Reply_ArticleText](#reply_articletext). `query` repeats the request as BioMCP read it.

### Reply_ArticleTextResult

Body text response; the envelope supplies advancing reader steps.

| Field | Type | Meaning |
| --- | --- | --- |
| `content` | [Reply_Section_TextContent](#reply_section_textcontent) | Explicit section outcome including not applicable and not open. |
| `pmcid` | [Reply_Pmcid](/reference/schemas/types/#reply_pmcid) or null | Requested PMC record when known. Optional. |
| `text_withheld` | [Reply_WithheldArticleText](/reference/schemas/types/#reply_withheldarticletext) or null | Explain a withheld body and provide the public reading destination. Optional. |

### Reply_ArticleText

Return the abstract card by default, or explicitly read body text.

| Field | Type | Meaning |
| --- | --- | --- |
| `find` | [Reply_Text](/reference/schemas/types/#reply_text) | Case-insensitive literal text to find. Optional. |
| `input` | [Reply_ArticleInput](/reference/schemas/types/#reply_articleinput) | PubMed or PMC identity. DOI selectors are not accepted by the reader. |
| `offset` | integer (uint32) | Number of matching rows already returned. Optional. |
| `part` | integer (uint32) | One-based outline part; omit reader selectors for the abstract card. Optional. |
| `replace` | boolean | Replace an existing destination only when explicitly requested with save. Optional. |
| `save` | [Reply_Text](/reference/schemas/types/#reply_text) | Save the whole original document to an explicit file or existing folder. Example: --save article-output. Optional. |
| `variant` | [Reply_Text](/reference/schemas/types/#reply_text) | Explicit checked protein substitution. Example: --variant p.V600E. Optional. |

### Other types in this reply

3 more types appear only in this reply: [Reply_Section_TextContent](#reply_section_textcontent), [Reply_TextContent](#reply_textcontent), [Reply_TextHit](#reply_texthit). Open the list below to read their fields.

<details>
<summary>Fields of the 3 other types</summary>

### Reply_Section_TextContent

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_TextContent](#reply_textcontent)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply_sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_TextContent

One bounded part or matching rows with the bucket license.

| Field | Type | Meaning |
| --- | --- | --- |
| `forms` | list of string or null | Literal protein change spellings searched by an explicit variant selector. Optional. |
| `heading_path` | list of [Reply_Text](/reference/schemas/types/#reply_text) or null | Section ancestry for a selected part. Optional. |
| `hits` | list of [Reply_TextHit](#reply_texthit) or null | Matching source rows for a find selection. Optional. |
| `license` | [Reply_License](/reference/schemas/types/#reply_license) | Bucket license. |
| `manuscript` | boolean | Author manuscript flag. Optional. |
| `markdown` | [Reply_Markdown](/reference/schemas/types/#reply_markdown) or null | Bounded body text for a direct part selection. Optional. |
| `part` | integer or null | Part number for a direct part selection. Optional. |
| `processing_version` | integer (uint32) | Selected processing version. Optional. |
| `saved` | [Reply_ArticleSaved](/reference/schemas/types/#reply_articlesaved) or null | Explicit save receipt, absent for reader views. Optional. |
| `scope` | [Reply_Text](/reference/schemas/types/#reply_text) | Scope of returned source text. |
| `url` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Source URL for an explicit save, including refused destinations. Optional. |

### Reply_TextHit

One matching paragraph or table row, split when necessary to fit the text budget.

| Field | Type | Meaning |
| --- | --- | --- |
| `heading_path` | list of [Reply_Text](/reference/schemas/types/#reply_text) | Section ancestry in source order. Optional. |
| `markdown` | [Reply_Markdown](/reference/schemas/types/#reply_markdown) | Whole bounded source row. |
| `part` | integer (uint32) | Outline part containing this text. |

</details>

## Next

[`biomcp article text`](/reference/commands/article-text/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
