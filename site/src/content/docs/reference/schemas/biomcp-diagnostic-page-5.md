---
title: Reply of biomcp search diagnostic (biomcp.diagnostic-page/5)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp search diagnostic:
  the fields of schema biomcp.diagnostic-page/5."
goal: "Parse the biomcp.diagnostic-page/5 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.diagnostic-page/5
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search diagnostic`](/reference/commands/search-diagnostic/) and [`biomcp search`](/reference/commands/search/) reply with `schema` set to `biomcp.diagnostic-page/5`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.diagnostic-page/5` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_DiagnosticPage](#reply-diagnosticpage). `query` holds [Reply_DiagnosticSearch](/reference/schemas/types/#reply-diagnosticsearch), a shared type. `query` repeats the request as BioMCP read it.

### Reply_DiagnosticPage

Source-ordered native diagnostic and medical-device submission candidates.

| Field | Type | Meaning |
| --- | --- | --- |
| `count_unit` | [Reply_DiagnosticCountUnit](/reference/schemas/types/#reply-diagnosticcountunit) or null | Exact meaning of the envelope total; supplied by the owning source command. Optional. |
| `distinct_id_total` | integer or null | Distinct complete native IDs across the selected relation collection. Optional. |
| `items` | list of [Reply_DiagnosticCandidate](#reply-diagnosticcandidate) | Complete source-owned rows in native order; envelope totals count native rows. Optional. |

### Other types in this reply

2 more types appear only in this reply: [Reply_DiagnosticCandidate](#reply-diagnosticcandidate), [Reply_GtrTestRelation](#reply-gtrtestrelation). Open the list below to read their fields.

<details>
<summary>Fields of the 2 other types</summary>

### Reply_DiagnosticCandidate

A conflict is visible once without a selected name or card.

One of these forms:

- `identity_status` is `"pma-submission"`: Native PMA submission; supplements retain their own identity. Fields: `header` ([Reply_DiagnosticPmaHeader](/reference/schemas/types/#reply-diagnosticpmaheader)).
- `identity_status` is `"clearance"`: Native 510(k) record. Fields: `header` (object).
- `identity_status` is `"device-recall"`: One native product recall record; repeated event numbers remain separate records. Fields: `header` ([Reply_DeviceRecallHeader](/reference/schemas/types/#reply-devicerecallheader)).
- `identity_status` is `"who-ivd"`: One matched WHO product with cells from its first matching native row. Fields: `id` ([Reply_WhoProductId](/reference/schemas/types/#reply-whoproductid)), `manufacturer` (string), `marker` (string), `product_name` (string).
- `identity_status` is `"resolved"`: One resolved native identity. Fields: `header` ([Reply_GtrHeader](/reference/schemas/types/#reply-gtrheader)), `matched` ([Reply_GtrTestRelation](#reply-gtrtestrelation) or null).
- `identity_status` is `"missing-card"`: A native relation remains visible without a current card. Fields: `id` ([Reply_GtrTestId](/reference/schemas/types/#reply-gtrtestid)), `matched` ([Reply_GtrTestRelation](#reply-gtrtestrelation)).
- `identity_status` is `"conflict"`: Several incompatible native rows with the same exact accession/version. Fields: `id` ([Reply_GtrTestId](/reference/schemas/types/#reply-gtrtestid)), `problem` ([Reply_Problem](/reference/schemas/types/#reply-problem)).

### Reply_GtrTestRelation

Source-owned native membership or one independent method-list entry.

One of these forms:

- `kind` is `"gene"`: Native gene relation row. Fields: `relation` ([Reply_GtrGeneRelation](/reference/schemas/types/#reply-gtrgenerelation)).
- `kind` is `"condition"`: Native condition relation row. Fields: `relation` ([Reply_GtrConditionRelation](/reference/schemas/types/#reply-gtrconditionrelation)).
- `kind` is `"method"`: One independent method-list entry. Fields: `relation` ([Reply_GtrMethod](/reference/schemas/types/#reply-gtrmethod)).

</details>

## Next

[`biomcp search diagnostic`](/reference/commands/search-diagnostic/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
