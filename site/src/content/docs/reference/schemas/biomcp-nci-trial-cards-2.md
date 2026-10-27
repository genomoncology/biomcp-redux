---
title: biomcp.nci_trial_cards/2
description: "The BioMCP 1.0.0 reply schema biomcp.nci_trial_cards/2: its
  envelope fields, its result type NciTrialCards and the commands that return
  it."
goal: Look up the fields of a biomcp.nci_trial_cards/2 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.nci_trial_cards/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.nci_trial_cards/2`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.nci_trial_cards/2`.

No help example returns this schema; the reply carries it in its `schema` field.

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | TrialGet | yes | Parsed query. |
| `result` | NciTrialCards or null | no | Answer when available. |
| `schema` | "biomcp.nci_trial_cards/2" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: NciTrialCards

Independent exact NCI answers in caller order.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `attribution` | string | yes | Publisher attribution. |
| `items` | array of NciTrialItem | yes | One independent answer per requested identity. |

## Query: TrialGet

Retrieve exact trial cards in input order; request bounded intact source sections with independent continuations.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `input` | array of NctId | yes | One to twenty exact NCT identifiers, in caller order. |
| `limit` | integer (uint32) | no | Whole-record count per source collection page; text fragments follow the text budget. |
| `offset` | integer (uint32) | no | Character offset for ClinicalTrials.gov eligibility or NCI summary; whole-record offset for other collections. |
| `pool` | Text | no | Checked stored source record, supplied by a continuation. |
| `section` | array of TrialSection | no | Optional intact source sections. Default cards only; diseases and summary require NCI. NCI locations, contacts, references and documents are unsupported. |
| `source` | TrialSource | no | Explicit provider; defaults to ClinicalTrials.gov and never falls back. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.nci_trial_cards/2` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
