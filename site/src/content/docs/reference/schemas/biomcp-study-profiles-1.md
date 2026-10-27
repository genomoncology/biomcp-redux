---
title: Reply of biomcp get study profiles section (biomcp.study_profiles/1)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get study profiles:
  a real example and the fields of schema biomcp.study_profiles/1."
goal: "Parse the biomcp.study_profiles/1 reply: see a real example and what each
  field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.study_profiles/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get study profiles`](/reference/commands/get-study/) replies with `schema` set to `biomcp.study_profiles/1`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp get study skcm_tcga profiles
```

`biomcp schema biomcp.study_profiles/1` prints the full JSON Schema.

## Example reply

`biomcp get study skcm_tcga profiles` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

```json
{
  "schema": "biomcp.study_profiles/1",
  "status": "complete",
  "query": {
    "id": "skcm_tcga",
    "section": "profiles",
    "section_limit": 5,
    "section_offset": 0
  },
  "result": {
    "study_id": "skcm_tcga",
    "items": [
      {
        "molecularProfileId": "skcm_tcga_gistic",
        "studyId": "skcm_tcga",
        "molecularAlterationType": "COPY_NUMBER_ALTERATION",
        "datatype": "DISCRETE",
        "name": "Putative copy-number alterations from GISTIC",
        "description": "Putative copy-number calls on 367 cases determined us…",
        "patientLevel": false,
        "showProfileInAnalysisTab": true
      },
      {
        "molecularProfileId": "skcm_tcga_linear_CNA",
        "studyId": "skcm_tcga",
        "molecularAlterationType": "COPY_NUMBER_ALTERATION",
        "datatype": "CONTINUOUS",
        "name": "Capped relative linear copy-number values",
        "description": "Capped relative linear copy-number values for each ge…",
        "patientLevel": false,
        "showProfileInAnalysisTab": false
      },
      … 3 more
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
        "get", "study", "skcm_tcga", "profiles", "--section-offset", "5"
      ]
    }
  ]
}
```

## Fields

`result` holds [StudyProfiles](/reference/schemas/types/#studyprofiles), a shared type. `query` holds [StudyGet](/reference/schemas/types/#studyget), a shared type. `query` repeats the request as BioMCP read it.

Every type this reply uses is shared with other replies, so its fields are on the shared types page.

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp get study`](/reference/commands/get-study/) gives the command's arguments.
