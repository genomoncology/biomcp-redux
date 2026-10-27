---
title: Reply of biomcp search diagnostic (biomcp.diagnostic_page/2)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp search diagnostic: a
  real example and the fields of schema biomcp.diagnostic_page/2."
goal: "Parse the biomcp.diagnostic_page/2 reply: see a real example and what
  each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.diagnostic_page/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search diagnostic`](/reference/commands/search-diagnostic/) replies with `schema` set to `biomcp.diagnostic_page/2`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp search diagnostic --source gtr --gene BRAF --limit 2
biomcp search diagnostic --source who-ivd --limit 2
```

`biomcp schema biomcp.diagnostic_page/2` prints the full JSON Schema.

## Example reply

`biomcp search diagnostic --source gtr --gene BRAF --limit 2` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

```json
{
  "schema": "biomcp.diagnostic_page/2",
  "status": "complete",
  "query": {
    "source": "gtr",
    "gene": "BRAF",
    "limit": 2,
    "offset": 0
  },
  "result": {
    "items": [
      {
        "identity_status": "resolved",
        "header": {
          "id": "GTR000273063.3",
          "name": "BRAF-Related Cardiofaciocutaneous Syndrome",
          "laboratory": {…},
          "url": "https://www.ncbi.nlm.nih.gov/gtr/tests/classic/273063/"
        },
        "matched": {
          "kind": "gene",
          "relation": {…}
        }
      },
      … 1 more
    ],
    "count_unit": "test_identities"
  },
  "page": {"offset": 0, "returned": 2, "total": 440},
  "sources": [
    {
      "source": "gtr",
      "use": "open",
      "terms": "NCBI usage policies; submitter and third-party rights retained",
      "terms_url": "https://www.ncbi.nlm.nih.gov/home/about/policies/",
      "state": "ok"
    }
  ],
  "next": [
    {
      "kind": "disambiguate",
      "args": [
        "get", "diagnostic", "GTR000273063.3", "--bundle-revision",
        "sha256:421cbbba145a93789b4dd2020dd4f64a0e7fa4b51e2a3cf2c0c184d4bf55e…",
        "--source", "gtr"
      ]
    },
    … 2 more
  ]
}
```

## Fields

`result` holds [DiagnosticPage](#diagnosticpage). `query` holds [DiagnosticSearch](/reference/schemas/types/#diagnosticsearch), a shared type. `query` repeats the request as BioMCP read it.

### DiagnosticPage

Source-ordered GTR test candidates or WHO product headers.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `count_unit` | [DiagnosticCountUnit](/reference/schemas/types/#diagnosticcountunit) or null | no | Exact meaning of the envelope total; supplied by the owning source command. |
| `distinct_id_total` | integer or null | no | Distinct complete native IDs across the selected relation collection. |
| `items` | list of [DiagnosticCandidate](#diagnosticcandidate) | yes | Complete source-owned rows in native order; envelope totals count native rows. |

### DiagnosticCandidate

A conflict is visible once without a selected name or card.

One of these forms:

- `identity_status` is `"who_ivd"`: One matched WHO product with cells from its first matching native row. Fields: `id` ([WhoProductId](/reference/schemas/types/#whoproductid)), `manufacturer` (string), `marker` (string), `product_name` (string).
- `identity_status` is `"resolved"`: One resolved native identity. Fields: `header` ([GtrHeader](/reference/schemas/types/#gtrheader)), `matched` ([GtrTestRelation](#gtrtestrelation) or null).
- `identity_status` is `"missing_card"`: A native relation remains visible without a current card. Fields: `id` ([GtrTestId](/reference/schemas/types/#gtrtestid)), `matched` ([GtrTestRelation](#gtrtestrelation)).
- `identity_status` is `"conflict"`: Several incompatible native rows with the same exact accession/version. Fields: `id` ([GtrTestId](/reference/schemas/types/#gtrtestid)), `problem` ([Problem](/reference/schemas/types/#problem)).

### GtrTestRelation

Source-owned native membership or one independent method-list entry.

One of these forms:

- `kind` is `"gene"`: Native gene relation row. Fields: `relation` ([GtrGeneRelation](/reference/schemas/types/#gtrgenerelation)).
- `kind` is `"condition"`: Native condition relation row. Fields: `relation` ([GtrConditionRelation](/reference/schemas/types/#gtrconditionrelation)).
- `kind` is `"method"`: One independent method-list entry. Fields: `relation` ([GtrMethod](/reference/schemas/types/#gtrmethod)).

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp search diagnostic`](/reference/commands/search-diagnostic/) gives the command's arguments.
