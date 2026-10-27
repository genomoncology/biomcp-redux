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

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get drug label`](/reference/commands/get-drug/) replies with `schema` set to `biomcp.drug_label/3`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp get drug 13a65923-d072-4dac-8620-8eabbf8c3641 label \
  --label-section indications_and_usage --source openfda-label
```

`biomcp schema biomcp.drug_label/3` prints the full JSON Schema.

## Example reply

`biomcp get drug 13a65923-d072-4dac-8620-8eabbf8c3641 label --label-section indications_and_usage --source openfda-label` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

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
      "data": {…}
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

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [DrugLabel](#druglabel). `query` holds [DrugGet](/reference/schemas/types/#drugget), a shared type. `query` repeats the request as BioMCP read it.

### DrugLabel

One explicit native label heading and text continuation.

| Field | Type | Meaning |
| --- | --- | --- |
| `identity` | [DrugHeader](/reference/schemas/types/#drugheader) | Selected native identity, checked independently of section facts. |
| `label` | [Section](/reference/schemas/types/#section) of [DrugLabelText](#druglabeltext) | Existing typed section state, preserving empty, unavailable and skipped outcomes. |

### DrugLabelFragment

One slice of a native label array element, partitioning the page's unchanged text.

| Field | Type | Meaning |
| --- | --- | --- |
| `characters` | integer (uint32) | Consecutive Unicode scalars consumed from the page text for this fragment. |
| `source_offset` | integer (uint32) | Unicode scalar offset inside that original array element. |
| `source_part` | integer (uint32) | Zero-based native array element; empty elements have a zero-character fragment. |

### DrugLabelReference

One independently identified label set and revision.

| Field | Type | Meaning |
| --- | --- | --- |
| `attribution` | [Attribution](/reference/schemas/types/#attribution) | Label provenance. |
| `id` | [DrugId](/reference/schemas/types/#drugid) | Published SPL set or native label record ID. |
| `revised` | [PartialDate](/reference/schemas/types/#partialdate) or null | Source-stated revision date. Optional. |
| `url` | [Text](/reference/schemas/types/#text) or null | Native label URL. Optional. |
| `version` | [Text](/reference/schemas/types/#text) or null | Native label revision. Optional. |

### DrugLabelText

A mechanical continuation of one native label heading.

| Field | Type | Meaning |
| --- | --- | --- |
| `fragments` | list of [DrugLabelFragment](#druglabelfragment) | Native array boundaries; consume the flat text in this order without separators. |
| `heading` | [Text](/reference/schemas/types/#text) | Exact native label heading. |
| `label` | [DrugLabelReference](#druglabelreference) | Exact source label set and revision. |
| `offset` | integer (uint32) | Zero-based bounded text fragment offset; section_limit is a maximum fragment count. |
| `retrieved` | [PartialDate](/reference/schemas/types/#partialdate) or null | Original source reply retrieval date; absent for legacy captures without a timestamp. Optional. |
| `text` | string | Original text slice, preserving whitespace and wording. |
| `total` | integer or null | Total bounded text fragments, including fragments for empty native array elements. Optional. |

## Next

[`biomcp get drug`](/reference/commands/get-drug/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
