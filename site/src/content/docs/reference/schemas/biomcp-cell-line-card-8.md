---
title: Reply of biomcp get cell-line (biomcp.cell-line-card/8)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get cell-line: the
  fields of schema biomcp.cell-line-card/8."
goal: "Parse the biomcp.cell-line-card/8 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.cell-line-card/8
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get cell-line`](/reference/commands/get-cell-line/) and [`biomcp get`](/reference/commands/get/) reply with `schema` set to `biomcp.cell-line-card/8`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.cell-line-card/8` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_CellLineCard](#reply-celllinecard). `query` holds [Reply_CellLineGet](/reference/schemas/types/#reply-celllineget), a shared type. `query` repeats the request as BioMCP read it.

### Reply_CellLineCard

One requested cell identity with explicit ambiguity and requested section states.

| Field | Type | Meaning |
| --- | --- | --- |
| `age` | [Reply_Text](/reference/schemas/types/#reply-text) or null | Native age expression when supplied. Optional. |
| `candidates` | list of [Reply_CellLineHeader](/reference/schemas/types/#reply-celllineheader) | Candidate identities without first-hit selection. Optional. |
| `dates` | [Reply_CellLineEntryDates](#reply-celllineentrydates) or null | Selected entry dates and version, separate from release metadata. Optional. |
| `fetched` | integer or null | Original retrieval seconds, distinct from entry and release dates. Optional. |
| `header` | [Reply_CellLineHeader](/reference/schemas/types/#reply-celllineheader) or null | Selected verified identity only. Optional. |
| `input` | [Reply_CellLineInput](/reference/schemas/types/#reply-celllineinput) | Exact original caller identity. |
| `more` | list of [Reply_CellLineSection](/reference/schemas/types/#reply-celllinesection) | Supported disclosures not requested. Optional. |
| `release` | [Reply_CellLineRelease](/reference/schemas/types/#reply-celllinerelease) or null | Independently fetched publisher release. Optional. |
| `sections` | [Reply_CellLineSections](#reply-celllinesections) | Only explicitly requested sections carry a state. |
| `sex` | [Reply_CellLineSex](#reply-celllinesex) or null | Native sex label when supplied. Optional. |
| `status` | [Reply_Status](/reference/schemas/types/#reply-status) | Resolved, absent, ambiguous or failed input outcome. |
| `url` | [Reply_Text](/reference/schemas/types/#reply-text) or null | Publisher URL for the selected identity. Optional. |
| `warnings` | [Reply_Capped_CellLineWarning](/reference/schemas/types/#reply-capped-celllinewarning) or null | Selected card warnings, up to three whole rows; not a fetch state. Optional. |

### Other types in this reply

8 more types appear only in this reply: [Reply_CellLineEntryDates](#reply-celllineentrydates), [Reply_CellLineSections](#reply-celllinesections), [Reply_CellLineSex](#reply-celllinesex), [Reply_Section_CellLineChembl](#reply-section-celllinechembl), [Reply_Section_CellLineIdentityPage](#reply-section-celllineidentitypage), [Reply_Section_CellLineNames](#reply-section-celllinenames), [Reply_Section_CellLineVariants](#reply-section-celllinevariants), [Reply_Section_CellLineXrefs](#reply-section-celllinexrefs). Open the list below to read their fields.

<details>
<summary>Fields of the 8 other types</summary>

### Reply_CellLineEntryDates

Entry dates and entry version are independent of dataset release and retrieval time.

| Field | Type | Meaning |
| --- | --- | --- |
| `created` | [Reply_PartialDate](/reference/schemas/types/#reply-partialdate) or null | Source entry creation date. Optional. |
| `updated` | [Reply_PartialDate](/reference/schemas/types/#reply-partialdate) or null | Source entry update date. Optional. |
| `version` | integer or null | Source entry version, never a dataset release. Optional. |

### Reply_CellLineSections

Requested sections carry explicit states; absent members were not requested.

| Field | Type | Meaning |
| --- | --- | --- |
| `chembl` | [Reply_Section_CellLineChembl](#reply-section-celllinechembl) or null | Requested chembl section state. Optional. |
| `identity` | [Reply_Section_CellLineIdentityPage](#reply-section-celllineidentitypage) or null | Requested identity section state. Optional. |
| `names` | [Reply_Section_CellLineNames](#reply-section-celllinenames) or null | Requested names section state. Optional. |
| `variants` | [Reply_Section_CellLineVariants](#reply-section-celllinevariants) or null | Requested variants section state. Optional. |
| `xrefs` | [Reply_Section_CellLineXrefs](#reply-section-celllinexrefs) or null | Requested xrefs section state. Optional. |

### Reply_CellLineSex

Native publisher vocabulary; new values are retained unchanged. Known values: Female; Male; Mixed sex; Sex ambiguous; Sex unspecified;

Type: string.

### Reply_Section_CellLineChembl

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_CellLineChembl](/reference/schemas/types/#reply-celllinechembl)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply-sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_Section_CellLineIdentityPage

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_CellLineIdentityPage](/reference/schemas/types/#reply-celllineidentitypage)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply-sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_Section_CellLineNames

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_CellLineNames](/reference/schemas/types/#reply-celllinenames)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply-sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_Section_CellLineVariants

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_CellLineVariants](/reference/schemas/types/#reply-celllinevariants)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply-sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_Section_CellLineXrefs

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_CellLineXrefs](/reference/schemas/types/#reply-celllinexrefs)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply-sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

</details>

## Next

[`biomcp get cell-line`](/reference/commands/get-cell-line/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
