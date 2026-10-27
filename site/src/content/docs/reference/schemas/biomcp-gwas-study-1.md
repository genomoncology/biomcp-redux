---
title: biomcp.gwas_study/1
description: "The BioMCP 1.0.0 reply schema biomcp.gwas_study/1: its envelope
  fields, its result type GwasStudy and the commands that return it."
goal: Look up the fields of a biomcp.gwas_study/1 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.gwas_study/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.gwas_study/1`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.gwas_study/1`.

These commands return it: [`biomcp get gwas`](/reference/commands/get-gwas/).

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | GwasGet | yes | Parsed query. |
| `result` | GwasStudy or null | no | Answer when available. |
| `schema` | "biomcp.gwas_study/1" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: GwasStudy

Native study card; independent source descriptions are never reconciled.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `array_manufacturers` | array or null | no | Native array manufacturer labels. |
| `discovery_ancestries` | array of Text | yes | Native discovery ancestry summaries. |
| `genotyping_technologies` | array of Text | yes | Source technologies. |
| `gxe` | boolean or null | no | Native gene-environment interaction flag. |
| `gxg` | boolean or null | no | Native gene-gene interaction flag. |
| `id` | GwasStudyId | yes | Exact accession. |
| `imputed` | boolean or null | no | Native imputation flag. |
| `initial_sample_description` | Text or null | no | Whole discovery sample prose. |
| `license_url` | Text or null | no | Native terms link without an inferred reuse grant. |
| `ontology_traits` | array of GwasTrait | yes | Every native mapped ontology label and ID. |
| `platforms` | Text or null | no | Native platform description. |
| `pooled` | boolean or null | no | Native pooled study flag. |
| `publication` | Pmid or null | no | Source publication identity. |
| `replication_ancestries` | array of Text | yes | Native replication ancestry summaries. |
| `replication_sample_description` | Text or null | no | Whole replication sample prose. |
| `reported_traits` | array of Text | yes | Native reported disease traits. |
| `snp_count` | integer or null | no | Source SNP count; missing differs from zero. |
| `summary_statistics_available` | boolean or null | no | Source availability flag; grants no download or reuse permission. |
| `summary_statistics_url` | Text or null | no | Native summary statistics link; not a local download. |
| `user_requested` | boolean or null | no | Native user-requested study flag. |

## Query: GwasGet

Explicit resource sum; impossible section and identity pairings fail during decoding.

One of:

- object: Read an association.
- object: Read a study.

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.gwas_study/1` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
