---
title: biomcp.key/2
description: "The BioMCP 1.0.0 reply schema biomcp.key/2: its envelope fields,
  its result type KeyHelp and the commands that return it."
goal: Look up the fields of a biomcp.key/2 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.key/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.key/2`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.key/2`.

These commands return it: [`biomcp help key ncbi`](/reference/commands/help-key-ncbi/), [`biomcp help key semantic-scholar`](/reference/commands/help-key-semantic-scholar/), [`biomcp help key orcid`](/reference/commands/help-key-orcid/), [`biomcp help key oncokb`](/reference/commands/help-key-oncokb/), [`biomcp help key nci`](/reference/commands/help-key-nci/), [`biomcp help key openfda`](/reference/commands/help-key-openfda/).

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | null | yes | Parsed query. |
| `result` | KeyHelp or null | no | Answer when available. |
| `schema` | "biomcp.key/2" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: KeyHelp

Configuration names for one keyed source; never includes credential values.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `config` | Text | yes | Configuration file key path. |
| `environment` | Text | yes | Environment variable name. |
| `guidance` | Text or null | no | Optional source-specific key guidance. |
| `source` | Source | yes | Source name. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.key/2` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
