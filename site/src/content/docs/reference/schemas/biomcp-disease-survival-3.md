---
title: biomcp.disease_survival/3
description: "The BioMCP 1.0.0 reply schema biomcp.disease_survival/3: its
  envelope fields, its result type DiseaseSurvival and the commands that return
  it."
goal: Look up the fields of a biomcp.disease_survival/3 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.disease_survival/3
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.disease_survival/3`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.disease_survival/3`.

These commands return it: [`biomcp get disease`](/reference/commands/get-disease/).

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | DiseaseGet | yes | Parsed query. |
| `result` | DiseaseSurvival or null | no | Answer when available. |
| `schema` | "biomcp.disease_survival/3" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: DiseaseSurvival

Native annual survival rows with separate observed and modeled values.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `components` | array of ComponentLabel | yes | Applicable component use terms, independent of row provenance. |
| `fetched` | integer or null | no | Original response retrieval time in Unix seconds, distinct from dataset publication. |
| `items` | array of DiseaseStatistic | yes | Native source rows; totals in the envelope count rows, not distinct entities. |

## Query: DiseaseGet

Exact concept or native SEER site with independently paged section.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `age_range` | Text or null | no | Native age-range code. |
| `dataset` | Text or null | no | Requested native dataset selector, not proof of returned membership. |
| `id` | Text | yes | Exact ontology CURIE or an explicit SEER catalog site ID. |
| `interval` | Text or null | no | Survival-only interval code. |
| `ontology` | DiseaseOntology | no | One OLS ontology. |
| `period` | Text or null | no | Incidence-only native period code. |
| `race` | Text or null | no | Native race code. |
| `section` | DiseaseSection or null | no | One named section; absent means identity card only. |
| `section_limit` | integer (uint32) | no | Native rows per page: default 2 for Monarch genes, 1 for Monarch phenotypes, otherwise 5. |
| `section_offset` | integer (uint32) | no | Native row offset within the selected section. |
| `sex` | Text or null | no | Native sex code. |
| `source` | DiseaseSource | no | Default MyDisease; no automatic fallback. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.disease_survival/3` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
