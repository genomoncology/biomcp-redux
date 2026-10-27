---
title: biomcp.cell_line_card/3
description: "The BioMCP 1.0.0 reply schema biomcp.cell_line_card/3: its
  envelope fields, its result type CellLineCard and the commands that return
  it."
goal: Look up the fields of a biomcp.cell_line_card/3 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.cell_line_card/3
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.cell_line_card/3`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.cell_line_card/3`.

These commands return it: [`biomcp get cell-line`](/reference/commands/get-cell-line/).

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | CellLineGet | yes | Parsed query. |
| `result` | CellLineCard or null | no | Answer when available. |
| `schema` | "biomcp.cell_line_card/3" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: CellLineCard

One requested cell identity with explicit ambiguity and requested section states.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `age` | Text or null | no | Native age expression when supplied. |
| `candidates` | array of CellLineHeader | yes | Candidate identities without first-hit selection. |
| `dates` | CellLineEntryDates or null | no | Selected entry dates and version, separate from release metadata. |
| `fetched` | integer or null | no | Original retrieval seconds, distinct from entry and release dates. |
| `header` | CellLineHeader or null | no | Selected verified identity only. |
| `input` | CellLineInput | yes | Exact original caller identity. |
| `more` | array of CellLineSection | yes | Supported disclosures not requested. |
| `release` | CellLineRelease or null | no | Independently fetched publisher release. |
| `sections` | CellLineSections | yes | Only explicitly requested sections carry a state. |
| `sex` | Text or null | no | Native sex label when supplied. |
| `status` | Status | yes | Resolved, absent, ambiguous or failed input outcome. |
| `url` | Text or null | no | Publisher URL for the selected identity. |
| `warnings` | Capped2 or null | no | Selected card warnings, up to three whole rows; not a fetch state. |

## Query: CellLineGet

One exact identity and an optional bounded named section.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `id` | CellLineInput | yes | One exact primary accession or qualified source ID. |
| `section` | CellLineSection | no | Omit for the default card; all returns bounded first pages without an assay count. |
| `section_limit` | integer (uint32) | no | Explicit row limit for each requested collection; native variation defaults to three. |
| `section_offset` | integer (uint32) | no | Offset within each independently requested collection. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.cell_line_card/3` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
