---
title: biomcp.gene_cards/4
description: "The BioMCP 1.0.0 reply schema biomcp.gene_cards/4: its envelope
  fields, its result type GeneCards and the commands that return it."
goal: Look up the fields of a biomcp.gene_cards/4 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.gene_cards/4
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.gene_cards/4`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.gene_cards/4`.

These commands return it: [`biomcp get gene`](/reference/commands/get-gene/).

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | GeneGet | yes | Parsed query. |
| `result` | GeneCards or null | no | Answer when available. |
| `schema` | "biomcp.gene_cards/4" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: GeneCards

Gene answers in input order.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `items` | array of GeneItem | yes | Requested inputs and their answers. |

## Query: GeneGet

Get one to twenty human gene inputs in their original order.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `input` | array of GeneInput | yes | Gene symbols or checked identifiers, in input order. |
| `section` | array of GeneSectionWord | no | Requested section words, preserving caller order and repetitions. Trailing section words select sections. Use --section WORD to keep inputs that equal a section word as inputs. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.gene_cards/4` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
