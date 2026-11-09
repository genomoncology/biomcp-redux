---
title: Reply of biomcp article recommendations (biomcp.article-recommendations/9)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp article
  recommendations: the fields of schema biomcp.article-recommendations/9."
goal: "Parse the biomcp.article-recommendations/9 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.article-recommendations/9
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp article recommendations`](/reference/commands/article-recommendations/) and [`biomcp article`](/reference/commands/article/) reply with `schema` set to `biomcp.article-recommendations/9`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.article-recommendations/9` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_ArticleRecommendations](#reply_articlerecommendations). `query` holds [Reply_ArticleRecommend](#reply_articlerecommend). `query` repeats the request as BioMCP read it.

### Reply_ArticleRecommendations

Source-native recommendation page.

| Field | Type | Meaning |
| --- | --- | --- |
| `items` | list of [Reply_ArticleItem](/reference/schemas/types/#reply_articleitem) | Papers in provider order, without a local topic filter. Optional. |
| `notice` | [Reply_EvidenceNotice](/reference/schemas/types/#reply_evidencenotice) or null | Successful zero-result explanation. Optional. |
| `scope` | string | Provider's default pool, stated without a clinical relevance claim. |

### Reply_ArticleRecommend

Recommendations in the provider's recent-paper pool order.

| Field | Type | Meaning |
| --- | --- | --- |
| `limit` | integer (uint32) | Maximum recommendations requested, in native order. Optional. |
| `negative` | list of [Reply_ArticleInput](/reference/schemas/types/#reply_articleinput) | Up to ten negative paper identities. Optional. |
| `offset` | integer (uint32) | Number of stored recommendations to skip. Optional. |
| `pool` | [Reply_Text](/reference/schemas/types/#reply_text) | Fixed recommendation pool. Optional. |
| `seeds` | list of [Reply_ArticleInput](/reference/schemas/types/#reply_articleinput) | One to ten positive paper identities. |

## Next

[`biomcp article recommendations`](/reference/commands/article-recommendations/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
