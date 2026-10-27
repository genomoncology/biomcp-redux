---
title: Reply of biomcp search study (biomcp.study_page/1)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp search study: a real
  example and the fields of schema biomcp.study_page/1."
goal: "Parse the biomcp.study_page/1 reply: see a real example and what each
  field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.study_page/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search study`](/reference/commands/search-study/) replies with `schema` set to `biomcp.study_page/1`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp search study melanoma --limit 2
biomcp search study Atlas
```

`biomcp schema biomcp.study_page/1` prints the full JSON Schema.

## Example reply

`biomcp search study melanoma --limit 2` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.study_page/1",
  "status": "failed",
  "query": {"text": "melanoma", "limit": 2, "offset": 0},
  "sources": [
    {
      "source": "cbioportal",
      "use": "restricted",
      "terms": "cBioPortal study-specific terms unverified; commercial restri…",
      "terms_url": "https://docs.cbioportal.org/user-guide/faq/",
      "state": "syncing",
      "reason": "syncing"
    }
  ],
  "next": [
    {
      "kind": "sync",
      "args": ["sync", "cbioportal-studies"]
    }
  ]
}
```

The source did not answer that run, so `status` is `failed` and `result` is missing. A reply that succeeds fills `result` with the fields below.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [StudyPage](#studypage). `query` holds [StudySearch](#studysearch). `query` repeats the request as BioMCP read it.

### StudyPage

Source-ordered checked study headers from one retained catalog.

| Field | Type | Meaning |
| --- | --- | --- |
| `items` | list of [StudyHeader](/reference/schemas/types/#studyheader) | Whole source headers, without expanded descriptions. |
| `page` | [PageInfo](/reference/schemas/types/#pageinfo) | Snapshot-scoped matching counts and offset. |
| `snapshot` | [StudySnapshot](/reference/schemas/types/#studysnapshot) | Catalog acquisition provenance. |

### StudySearch

Literal stored metadata search; text is never parsed as a query language. No frequency, denominator, survival or comparison is calculated. Source counts keep their native names.

| Field | Type | Meaning |
| --- | --- | --- |
| `cancer_type` | [CbioportalCancerTypeId](/reference/schemas/types/#cbioportalcancertypeid) | Exact cBioPortal cancer-type code; no OncoTree conversion. Optional. |
| `limit` | integer (uint32) | Maximum headers per page. Optional. |
| `offset` | integer (uint32) | Zero-based offset in the checked snapshot. Optional. |
| `pool` | [Text](/reference/schemas/types/#text) or null | Opaque continuation retaining the catalog revision and selectors. Optional. |
| `text` | [Text](/reference/schemas/types/#text) | Literal study ID, name or description text, without synonyms or typo repair. Optional. |

## Next

[`biomcp search study`](/reference/commands/search-study/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
