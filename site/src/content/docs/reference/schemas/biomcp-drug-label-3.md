---
title: Reply of biomcp get drug label section (biomcp.drug_label/3)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get drug label: a
  real example and the fields of schema biomcp.drug_label/3."
goal: "Parse the biomcp.drug_label/3 reply: see a real example and what each
  field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.drug_label/3
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get drug label`](/reference/commands/get-drug/) replies with `schema` set to `biomcp.drug_label/3`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp get drug 13a65923-d072-4dac-8620-8eabbf8c3641 label \
  --label-section indications_and_usage --source openfda-label
```

`biomcp schema biomcp.drug_label/3` prints the full JSON Schema.

## Example reply

`biomcp get drug 13a65923-d072-4dac-8620-8eabbf8c3641 label --label-section indications_and_usage --source openfda-label` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

```json
{
  "schema": "biomcp.drug_label/3",
  "status": "complete",
  "query": {
    "input": "13a65923-d072-4dac-8620-8eabbf8c3641",
    "source": "openfda-label",
    "section": ["label"],
    "label_section": "indications_and_usage"
  },
  "result": {
    "identity": {
      "id": "13a65923-d072-4dac-8620-8eabbf8c3641",
      "source": "openfda-label",
      "name": "RASONQUE",
      "identity_level": "spl_set",
      "version": "1",
      "url": "https://dailymed.nlm.nih.gov/dailymed/drugInfo.cfm?setid=13a6592…"
    },
    "label": {
      "state": "data",
      "source": "openfda-label",
      "data": {
        "label": {
          "id": "13a65923-d072-4dac-8620-8eabbf8c3641",
          "version": "1",
          "revised": "2026-08-27",
          "url": "https://dailymed.nlm.nih.gov/dailymed/drugInfo.cfm?setid=13…",
          "attribution": {
            "source": "openfda-label",
            "record": "107d4cd9-2e5e-419d-9449-7ef84f5fbcc9",
            "version": "1",
            "url": "https://dailymed.nlm.nih.gov/dailymed/drugInfo.cfm?setid=1…"
          }
        },
        "heading": "indications_and_usage",
        "text": "1 INDICATIONS AND USAGE RASONQUE is indicated for the treatm…",
        "retrieved": "2026-10-27",
        "fragments": [
          {"source_part": 0, "source_offset": 0, "characters": 494}
        ],
        "offset": 0,
        "total": 1
      }
    }
  },
  "page": {"offset": 0, "returned": 1, "total": 1},
  "sources": [
    {
      "source": "openfda-label",
      "use": "open",
      "terms": "openFDA terms; original label rights retained",
      "terms_url": "https://open.fda.gov/terms/",
      "state": "ok",
      "count": 1,
      "fetched": 1
    }
  ]
}
```

## Fields

`result` holds [DrugLabel](#druglabel). `query` holds [DrugGet](/reference/schemas/types/#drugget), a shared type. `query` repeats the request as BioMCP read it.

### DrugLabel

One explicit native label heading and text continuation.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `identity` | [DrugHeader](/reference/schemas/types/#drugheader) | yes | Selected native identity, checked independently of section facts. |
| `label` | [Section](/reference/schemas/types/#section) of [DrugLabelText](#druglabeltext) | yes | Existing typed section state, preserving empty, unavailable and skipped outcomes. |

### DrugLabelFragment

One slice of a native label array element, partitioning the page's unchanged text.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `characters` | integer (uint32) | yes | Consecutive Unicode scalars consumed from the page text for this fragment. |
| `source_offset` | integer (uint32) | yes | Unicode scalar offset inside that original array element. |
| `source_part` | integer (uint32) | yes | Zero-based native array element; empty elements have a zero-character fragment. |

### DrugLabelReference

One independently identified label set and revision.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `attribution` | [Attribution](/reference/schemas/types/#attribution) | yes | Label provenance. |
| `id` | [DrugId](/reference/schemas/types/#drugid) | yes | Published SPL set or native label record ID. |
| `revised` | [PartialDate](/reference/schemas/types/#partialdate) or null | no | Source-stated revision date. |
| `url` | [Text](/reference/schemas/types/#text) or null | no | Native label URL. |
| `version` | [Text](/reference/schemas/types/#text) or null | no | Native label revision. |

### DrugLabelText

A mechanical continuation of one native label heading.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `fragments` | list of [DrugLabelFragment](#druglabelfragment) | yes | Native array boundaries; consume the flat text in this order without separators. |
| `heading` | [Text](/reference/schemas/types/#text) | yes | Exact native label heading. |
| `label` | [DrugLabelReference](#druglabelreference) | yes | Exact source label set and revision. |
| `offset` | integer (uint32) | yes | Zero-based bounded text fragment offset; section_limit is a maximum fragment count. |
| `retrieved` | [PartialDate](/reference/schemas/types/#partialdate) or null | no | Original source reply retrieval date; absent for legacy captures without a timestamp. |
| `text` | string | yes | Original text slice, preserving whitespace and wording. |
| `total` | integer or null | no | Total bounded text fragments, including fragments for empty native array elements. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp get drug`](/reference/commands/get-drug/) gives the command's arguments.
