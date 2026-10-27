---
title: Reply of biomcp get study files section (biomcp.study_files/1)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get study files: a
  real example and the fields of schema biomcp.study_files/1."
goal: "Parse the biomcp.study_files/1 reply: see a real example and what each
  field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.study_files/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get study files`](/reference/commands/get-study/) replies with `schema` set to `biomcp.study_files/1`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp get study synthetic_0 files
```

`biomcp schema biomcp.study_files/1` prints the full JSON Schema.

## Example reply

`biomcp get study synthetic_0 files` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

```json
{
  "schema": "biomcp.study_files/1",
  "status": "failed",
  "query": {
    "id": "synthetic_0",
    "section": "files",
    "section_limit": 5,
    "section_offset": 0
  },
  "sources": [
    {
      "source": "cbioportal",
      "use": "restricted",
      "terms": "cBioPortal study-specific terms unverified; commercial restri…",
      "terms_url": "https://docs.cbioportal.org/user-guide/faq/",
      "state": "failed",
      "reason": "proxy"
    }
  ],
  "next": [
    {
      "kind": "configure",
      "args": ["help", "network", "proxy"]
    }
  ]
}
```

The source did not answer that run, so `status` is `failed` and `result` is missing. A reply that succeeds fills `result` with the fields below.

## Fields

`result` holds [StudyFiles](/reference/schemas/types/#studyfiles), a shared type. `query` holds [StudyGet](/reference/schemas/types/#studyget), a shared type. `query` repeats the request as BioMCP read it.

Every type this reply uses is shared with other replies, so its fields are on the shared types page.

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp get study`](/reference/commands/get-study/) gives the command's arguments.
