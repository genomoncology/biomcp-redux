---
title: Reply of biomcp article citations (biomcp.article-edges/10)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp article citations:
  the fields of schema biomcp.article-edges/10."
goal: "Parse the biomcp.article-edges/10 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.article-edges/10
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp article citations`](/reference/commands/article-citations/), [`biomcp article references`](/reference/commands/article-references/) and [`biomcp article`](/reference/commands/article/) reply with `schema` set to `biomcp.article-edges/10`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.article-edges/10` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_ArticleEdgePage](#reply_articleedgepage). `query` holds [Reply_ArticleEdges](#reply_articleedges). `query` repeats the request as BioMCP read it.

### Reply_ArticleEdgePage

A page of citing or referenced papers in citation order.

| Field | Type | Meaning |
| --- | --- | --- |
| `items` | list of [Reply_ArticleItem](/reference/schemas/types/#reply_articleitem) | Papers in the fixed edge pool. Optional. |

### Reply_ArticleEdges

Page the citing or referenced PubMed papers of one checked PMID.

| Field | Type | Meaning |
| --- | --- | --- |
| `limit` | integer (uint32) | Maximum papers on one page. Optional. |
| `offset` | integer (uint32) | Number of pool items to skip. Optional. |
| `pmid` | [Reply_ArticleInput](/reference/schemas/types/#reply_articleinput) | PMID, PMC or DOI; iCite accepts only a PMID. |
| `pool` | [Reply_Text](/reference/schemas/types/#reply_text) | Fixed snapshot from an earlier page. Optional. |
| `source` | [Reply_EdgeSource](#reply_edgesource) | Graph provider; iCite remains the default. Optional. |

### Other types in this reply

One more type appear only in this reply: [Reply_EdgeSource](#reply_edgesource). Open the list below to read their fields.

<details>
<summary>Fields of the 1 other type</summary>

### Reply_EdgeSource

Graph edge provider, independent of article search providers.

Values: `icite`, `semantic-scholar`.

</details>

## Next

[`biomcp article citations`](/reference/commands/article-citations/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
