---
title: Reply of biomcp get study (biomcp.study-card/7)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get study: the
  fields of schema biomcp.study-card/7."
goal: "Parse the biomcp.study-card/7 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.study-card/7
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get study`](/reference/commands/get-study/) and [`biomcp get`](/reference/commands/get/) reply with `schema` set to `biomcp.study-card/7`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.study-card/7` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_StudyCard](#reply-studycard). `query` holds [Reply_StudyGet](/reference/schemas/types/#reply-studyget), a shared type. `query` repeats the request as BioMCP read it.

### Reply_StudyCard

Exact card with independent live section states and catalog provenance.

| Field | Type | Meaning |
| --- | --- | --- |
| `files` | [Reply_Section_StudyFiles](#reply-section-studyfiles) or null | Requested file-reference page, with no downloaded data. Optional. |
| `mutations` | [Reply_Section_StudyMutations](#reply-section-studymutations) or null | Requested source mutation rows. Optional. |
| `profiles` | [Reply_Section_StudyProfiles](#reply-section-studyprofiles) or null | Requested molecular-profile page, absent when not requested. Optional. |
| `sample_lists` | [Reply_Section_StudySampleLists](#reply-section-studysamplelists) or null | Requested sample-list metadata page, without member IDs. Optional. |
| `samples` | [Reply_Section_StudySamples](#reply-section-studysamples) or null | Requested sample identities and types. Optional. |
| `snapshot` | [Reply_StudySnapshot](/reference/schemas/types/#reply-studysnapshot) | Catalog revision and acquisition, apart from each live section's time. |
| `source_url` | [Reply_Text](/reference/schemas/types/#reply-text) | Exact publisher study page for this checked source identity. |
| `study` | [Reply_Study](#reply-study) | Native card metadata, preserving all source count distinctions. |

### Other types in this reply

9 more types appear only in this reply: [Reply_Section_StudyFiles](#reply-section-studyfiles), [Reply_Section_StudyMutations](#reply-section-studymutations), [Reply_Section_StudyProfiles](#reply-section-studyprofiles), [Reply_Section_StudySampleLists](#reply-section-studysamplelists), [Reply_Section_StudySamples](#reply-section-studysamples), [Reply_Study](#reply-study), [Reply_StudyImportTime](#reply-studyimporttime), [Reply_StudyResourceCount](#reply-studyresourcecount), [Reply_StudyResourceId](#reply-studyresourceid). Open the list below to read their fields.

<details>
<summary>Fields of the 9 other types</summary>

### Reply_Section_StudyFiles

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_StudyFiles](/reference/schemas/types/#reply-studyfiles)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply-sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_Section_StudyMutations

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_StudyMutations](/reference/schemas/types/#reply-studymutations)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply-sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_Section_StudyProfiles

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_StudyProfiles](/reference/schemas/types/#reply-studyprofiles)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply-sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_Section_StudySampleLists

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_StudySampleLists](/reference/schemas/types/#reply-studysamplelists)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply-sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_Section_StudySamples

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_StudySamples](/reference/schemas/types/#reply-studysamples)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply-sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_Study

Source-specific study card metadata; no inferred denominator or gene aggregate.

| Field | Type | Meaning |
| --- | --- | --- |
| `cnaSampleCount` | integer or null | Native cnaSampleCount. Optional. |
| `description` | string or null | Native description. Optional. |
| `groups` | string | Native access groups, possibly empty; not a reuse license. |
| `header` | [Reply_StudyHeader](/reference/schemas/types/#reply-studyheader) | Compact study identity and publication references. |
| `importDate` | [Reply_StudyImportTime](#reply-studyimporttime) or null | Native timezone-unstated import time. Optional. |
| `patientCount` | integer or null | Only an explicitly stated card patientCount; captured cards omit it, distinct from zero. Optional. |
| `publicStudy` | boolean | Native public-study access metadata. |
| `readPermission` | boolean | Native access metadata; not a reuse license. |
| `resourceCounts` | list of [Reply_StudyResourceCount](#reply-studyresourcecount) | Independent resource-scoped counts, never summed into study totals. Optional. |
| `sequencedSampleCount` | integer or null | Native sequencedSampleCount, not gene coverage. Optional. |
| `structuralVariantCount` | integer or null | Native structuralVariantCount; its name does not imply sample units. Optional. |
| `treatmentCount` | integer or null | Native treatmentCount without a unit conversion. Optional. |

### Reply_StudyImportTime

Native study import timestamp with unstated timezone; never an acquisition time. The parser additionally validates the calendar day, including leap years.

Type: string.

### Reply_StudyResourceCount

One native resource scope; counts remain attached to its study and resource.

| Field | Type | Meaning |
| --- | --- | --- |
| `cancerStudyIdentifier` | [Reply_StudyId](/reference/schemas/types/#reply-studyid) | Exact resource study scope. |
| `description` | string or null | Native resource description. Optional. |
| `displayName` | string or null | Native resource display name. Optional. |
| `openByDefault` | boolean | Native access/display setting, not a reuse license. |
| `patientCount` | integer or null | Native patient count within this resource only. Optional. |
| `priority` | [Reply_Text](/reference/schemas/types/#reply-text) or null | Native priority string, without numeric reinterpretation. Optional. |
| `resourceId` | [Reply_StudyResourceId](#reply-studyresourceid) | Native resource ID. |
| `resourceType` | [Reply_Text](/reference/schemas/types/#reply-text) or null | Extensible native resource type, not a derived count unit. Optional. |
| `sampleCount` | integer or null | Native count within this resource only. Optional. |

### Reply_StudyResourceId

Exact cBioPortal resource identity scoped to one study.

Type: string.

</details>

## Next

[`biomcp get study`](/reference/commands/get-study/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
