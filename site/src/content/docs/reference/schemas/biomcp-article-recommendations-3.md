---
title: biomcp.article_recommendations/3
description: "The BioMCP 1.0.0 reply schema biomcp.article_recommendations/3:
  its envelope fields, its result type ArticleRecommendations and the commands
  that return it."
goal: Look up the fields of a biomcp.article_recommendations/3 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.article_recommendations/3
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.article_recommendations/3`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.article_recommendations/3`.

These commands return it: [`biomcp article recommendations`](/reference/commands/article-recommendations/).

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | ArticleRecommend | yes | Parsed query. |
| `result` | ArticleRecommendations or null | no | Answer when available. |
| `schema` | "biomcp.article_recommendations/3" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: ArticleRecommendations

Source-native recommendation page.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `items` | array of ArticleItem | yes | Papers in provider order, without a local topic filter. |
| `notice` | EvidenceNotice or null | no | Successful zero-result explanation. |
| `scope` | string | yes | Provider's default pool, stated without a clinical relevance claim. |

## Query: ArticleRecommend

Recommendations in the provider's recent-paper pool order.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `limit` | integer (uint32) | no | Maximum recommendations requested, in native order. |
| `negative` | array of ArticleInput | no | Up to ten negative paper identities. |
| `offset` | integer (uint32) | no | Number of stored recommendations to skip. |
| `pool` | Text | no | Fixed recommendation pool. |
| `seeds` | array of ArticleInput | yes | One to ten positive paper identities. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.article_recommendations/3` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
