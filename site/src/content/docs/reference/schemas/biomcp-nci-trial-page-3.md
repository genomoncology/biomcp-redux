---
title: biomcp.nci_trial_page/3
description: "The BioMCP 1.0.0 reply schema biomcp.nci_trial_page/3: its
  envelope fields, its result type NciTrialPage and the commands that return
  it."
goal: Look up the fields of a biomcp.nci_trial_page/3 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.nci_trial_page/3
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.nci_trial_page/3`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.nci_trial_page/3`.

No help example returns this schema; the reply carries it in its `schema` field.

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | TrialSearch | yes | Parsed query. |
| `result` | NciTrialPage or null | no | Answer when available. |
| `schema` | "biomcp.nci_trial_page/3" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: NciTrialPage

NCI search page in native order with native count.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `attribution` | string | yes | Publisher attribution. |
| `condition_resolution` | TrialConditionResolution or null | no | Exact condition identity and source-provided mapping, absent for literal text. |
| `items` | array of NciTrial | yes | Source rows, never merged with another provider. |
| `request` | array of TrialParameter | yes | Exact source parameters, excluding credentials. |

## Query: TrialSearch

Search one provider with bounded pages and source-native counts; default ClinicalTrials.gov, explicit keyed NCI, never a fallback.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `age` | integer (uint32) | no | Age in years; unstated source bounds count as no limit. |
| `condition` | Text | no | Registry condition text, or an NCIt code for NCI; use --keyword for NCI free text. |
| `condition_id` | Curie | no | Exact MONDO or DOID CURIE resolved by MyDisease; NCI also accepts explicit NCIT:C codes. |
| `count_only` | boolean | no | Source total only; no returned study rows. |
| `criteria` | Text | no | Search literal registry eligibility text for discovery only; no eligibility judgment. |
| `distance` | Text | no | Source distance with explicit mi or km units. |
| `facility` | Text | no | Exact registry facility field; cannot combine with radius. |
| `gene` | GeneInput | no | Resolve one checked gene for registry text discovery only; no eligibility judgment. |
| `has_results` | boolean | no | Require posted source results. |
| `id` | NctId | no | Exact NCT identity in a conjunction with other filters. |
| `intervention` | Text | no | Literal source intervention text. |
| `keyword` | Text | no | NCI free text; explicit keyword mode never substitutes for a disease identity. |
| `lat` | Text | no | Radius center latitude in degrees. |
| `limit` | integer (uint32) | no | Headers per page; NCI applies its own lower publisher limit. |
| `lon` | Text | no | Radius center longitude in degrees. |
| `next_page` | Text | no | Opaque registry continuation; keep filters unchanged. |
| `phase` | PhaseFilter | no | Registry phase: 1, 2, 3, 4, early-1, not-applicable, 1/2 or 2/3. |
| `query` | Text | no | General registry discovery text. |
| `sex` | TrialSex | no | Registry sex facet. |
| `source` | TrialSource | no | Explicit provider; defaults to ClinicalTrials.gov and never falls back. |
| `sponsor` | Text | no | Exact lead sponsor field. |
| `sponsor_type` | SponsorType | no | Registry sponsor funding facet. |
| `status` | TrialStatus | no | Exact registry overall status. |
| `study_type` | StudyType | no | Exact registry study type. |
| `updated_since` | PartialDate | no | First included registry update date. |
| `updated_until` | PartialDate | no | Last included registry update date. |
| `variant` | Text | no | Literal variant text for registry discovery only; no eligibility judgment. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.nci_trial_page/3` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
