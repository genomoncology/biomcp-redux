---
title: Reply of biomcp get study (biomcp.study_card/2)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get study: a real
  example and the fields of schema biomcp.study_card/2."
goal: "Parse the biomcp.study_card/2 reply: see a real example and what each
  field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.study_card/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get study`](/reference/commands/get-study/) replies with `schema` set to `biomcp.study_card/2`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp get study synthetic_0
```

`biomcp schema biomcp.study_card/2` prints the full JSON Schema.

## Example reply

`biomcp get study synthetic_0` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.study_card/2",
  "status": "failed",
  "query": {
    "id": "synthetic_0",
    "section_limit": 5,
    "section_offset": 0
  },
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

`result` holds [StudyCard](#studycard). `query` holds [StudyGet](/reference/schemas/types/#studyget), a shared type. `query` repeats the request as BioMCP read it.

### StudyCard

Exact card with independent live section states and catalog provenance.

| Field | Type | Meaning |
| --- | --- | --- |
| `files` | [Section](/reference/schemas/types/#section) of [StudyFiles](/reference/schemas/types/#studyfiles) or null | Requested file-reference page, with no downloaded data. Optional. |
| `profiles` | [Section](/reference/schemas/types/#section) of [StudyProfiles](/reference/schemas/types/#studyprofiles) or null | Requested molecular-profile page, absent when not requested. Optional. |
| `sample_lists` | [Section](/reference/schemas/types/#section) of [StudySampleLists](/reference/schemas/types/#studysamplelists) or null | Requested sample-list metadata page, without member IDs. Optional. |
| `snapshot` | [StudySnapshot](/reference/schemas/types/#studysnapshot) | Catalog revision and acquisition, apart from each live section's time. |
| `source_url` | [Text](/reference/schemas/types/#text) | Exact publisher study page for this checked source identity. |
| `study` | [Study](#study) | Native card metadata, preserving all source count distinctions. |

### Other types in this reply

4 more types appear only in this reply: [Study](#study), [StudyImportTime](#studyimporttime), [StudyResourceCount](#studyresourcecount), [StudyResourceId](#studyresourceid). Open the list below to read their fields.

<details>
<summary>Fields of the 4 other types</summary>

### Study

Source-specific study card metadata; no inferred denominator or gene aggregate.

| Field | Type | Meaning |
| --- | --- | --- |
| `cnaSampleCount` | integer (uint64) | Native cnaSampleCount. |
| `description` | string | Native description. |
| `groups` | string | Native access groups, possibly empty; not a reuse license. |
| `header` | [StudyHeader](/reference/schemas/types/#studyheader) | Compact study identity and publication references. |
| `importDate` | [StudyImportTime](#studyimporttime) | Native timezone-unstated import time. |
| `patientCount` | integer or null | Only an explicitly stated card patientCount; captured cards omit it, distinct from zero. Optional. |
| `publicStudy` | boolean | Native public-study access metadata. |
| `readPermission` | boolean | Native access metadata; not a reuse license. |
| `resourceCounts` | list of [StudyResourceCount](#studyresourcecount) | Independent resource-scoped counts, never summed into study totals. |
| `sequencedSampleCount` | integer (uint64) | Native sequencedSampleCount, not gene coverage. |
| `structuralVariantCount` | integer (uint64) | Native structuralVariantCount; its name does not imply sample units. |
| `treatmentCount` | integer (uint64) | Native treatmentCount without a unit conversion. |

### StudyImportTime

Type: string.

### StudyResourceCount

One native resource scope; counts remain attached to its study and resource.

| Field | Type | Meaning |
| --- | --- | --- |
| `cancerStudyIdentifier` | [StudyId](/reference/schemas/types/#studyid) | Exact resource study scope. |
| `description` | string | Native resource description. |
| `displayName` | string | Native resource display name. |
| `openByDefault` | boolean | Native access/display setting, not a reuse license. |
| `patientCount` | integer (uint64) | Native patient count within this resource only. |
| `priority` | [Text](/reference/schemas/types/#text) | Native priority string, without numeric reinterpretation. |
| `resourceId` | [StudyResourceId](#studyresourceid) | Native resource ID. |
| `resourceType` | [Text](/reference/schemas/types/#text) | Extensible native resource type, not a derived count unit. |
| `sampleCount` | integer (uint64) | Native count within this resource only. |

### StudyResourceId

Type: string.

</details>

## Next

[`biomcp get study`](/reference/commands/get-study/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
