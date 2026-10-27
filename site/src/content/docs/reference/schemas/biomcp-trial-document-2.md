---
title: biomcp.trial_document/2
description: "The BioMCP 1.0.0 reply schema biomcp.trial_document/2: its
  envelope fields, its result type TrialDocumentResult and the commands that
  return it."
goal: Look up the fields of a biomcp.trial_document/2 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.trial_document/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.trial_document/2`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.trial_document/2`.

These commands return it: [`biomcp trial document`](/reference/commands/trial-document/).

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | TrialDocumentGet | yes | Parsed query. |
| `result` | TrialDocumentResult or null | no | Answer when available. |
| `schema` | "biomcp.trial_document/2" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: TrialDocumentResult

Original trial PDF download, never document text.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `file` | Section | yes | Saved receipt, confirmed absence, or a source failure. |

## Query: TrialDocumentGet

Download one exact ClinicalTrials.gov manifest PDF to an explicit caller path; never read its text. CLI and stdio MCP support saves; HTTP refuses with the source URL.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `file` | Text | yes | Exact filename from the registry manifest; never a URL or inferred path. |
| `input` | NctId | yes | Exact registry identity. |
| `replace` | boolean | no | Replace an existing file only when explicitly requested. |
| `save` | Text | yes | Caller-owned file or existing folder. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.trial_document/2` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
