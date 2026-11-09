---
title: Reply of biomcp get study sample-lists section (biomcp.study_sample_lists/1)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get study
  sample-lists: a real example and the fields of schema
  biomcp.study_sample_lists/1."
goal: "Parse the biomcp.study_sample_lists/1 reply: see a real example and what
  each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.study_sample_lists/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get study sample-lists`](/reference/commands/get-study/) replies with `schema` set to `biomcp.study_sample_lists/1`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp get study skcm_tcga sample-lists
```

`biomcp schema biomcp.study_sample_lists/1` prints the full JSON Schema.

## Example reply

`biomcp get study skcm_tcga sample-lists` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.study_sample_lists/1",
  "status": "complete",
  "query": {
    "id": "skcm_tcga",
    "section": "sample-lists",
    "section_limit": 5,
    "section_offset": 0
  },
  "result": {
    "study_id": "skcm_tcga",
    "items": [
      {…},
      … 4 more
    ],
    "page": {"offset": 0, "returned": 5, "total": 9},
    "acquired": 1793140313
  },
  "page": {"offset": 0, "returned": 5, "total": 9},
  "sources": [
    {
      "source": "cbioportal",
      "use": "restricted",
      "terms": "cBioPortal study-specific terms unverified; commercial restri…",
      "terms_url": "https://docs.cbioportal.org/user-guide/faq/",
      "state": "ok"
    }
  ],
  "next": [
    {
      "kind": "page",
      "args": [
        "get", "study", "skcm_tcga", "sample-lists", "--section-offset", "5"
      ]
    }
  ]
}
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [StudySampleLists](/reference/schemas/types/#studysamplelists), a shared type. `query` holds [StudyGet](/reference/schemas/types/#studyget), a shared type. `query` repeats the request as BioMCP read it.

Every type this reply uses is shared with other replies, so its fields are on the shared types page.

## Next

[`biomcp get study`](/reference/commands/get-study/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
