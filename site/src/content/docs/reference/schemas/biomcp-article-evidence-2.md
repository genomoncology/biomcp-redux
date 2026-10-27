---
title: biomcp.article_evidence/2
description: "The BioMCP 1.0.0 reply schema biomcp.article_evidence/2: its
  envelope fields, its result type ArticleEvidenceResult and the commands that
  return it."
goal: Look up the fields of a biomcp.article_evidence/2 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.article_evidence/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.article_evidence/2`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.article_evidence/2`.

These commands return it: [`biomcp article citation-evidence`](/reference/commands/article-citation-evidence/).

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | ArticleEvidence | yes | Parsed query. |
| `result` | ArticleEvidenceResult or null | no | Answer when available. |
| `schema` | "biomcp.article_evidence/2" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: ArticleEvidenceResult

Independent graph and optional open-text evidence parts.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `fulltext` | Section2 or null | no | The citing paper's own explicit reference paragraphs. |
| `semantic_scholar` | Section | yes | Source-reported citation contexts. |

## Query: ArticleEvidence

Read every source context and optional explicit full-text citation paragraph.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `cited` | ArticleInput | yes | Cited PMID, PMC identity or DOI. |
| `citing` | ArticleInput | yes | Citing PMID, PMC identity or DOI. |
| `fulltext` | boolean | no | Include the citing paper's own explicit reference paragraphs. |
| `offset` | integer (uint32) | no | Context fragment position returned by the advancing step. |
| `paragraph_offset` | integer (uint32) | no | Full-text fragment position returned by the advancing step. |
| `pool` | Text | no | Stored checked source scan and original hit positions. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.article_evidence/2` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
