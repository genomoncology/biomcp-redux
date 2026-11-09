---
title: Reply of biomcp get protein (biomcp.protein-card/10)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get protein: the
  fields of schema biomcp.protein-card/10."
goal: "Parse the biomcp.protein-card/10 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.protein-card/10
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get protein`](/reference/commands/get-protein/) and [`biomcp get`](/reference/commands/get/) reply with `schema` set to `biomcp.protein-card/10`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.protein-card/10` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_ProteinCard](#reply-proteincard). `query` holds [Reply_ProteinGet](/reference/schemas/types/#reply-proteinget), a shared type. `query` repeats the request as BioMCP read it.

### Reply_ProteinCard

Answers to requested inputs; ambiguous gene inputs retain all candidates.

| Field | Type | Meaning |
| --- | --- | --- |
| `items` | list of [Reply_ProteinCardItem](#reply-proteincarditem) | One result per input in caller order. Optional. |

### Other types in this reply

6 more types appear only in this reply: [Reply_ProteinCardItem](#reply-proteincarditem), [Reply_ProteinSections](#reply-proteinsections), [Reply_Section_ProteinComplexes](#reply-section-proteincomplexes), [Reply_Section_ProteinDomains](#reply-section-proteindomains), [Reply_Section_ProteinFunctionResult](#reply-section-proteinfunctionresult), [Reply_Section_ProteinSequenceResult](#reply-section-proteinsequenceresult). Open the list below to read their fields.

<details>
<summary>Fields of the 6 other types</summary>

### Reply_ProteinCardItem

One requested identity and its source outcome, without first-hit selection.

| Field | Type | Meaning |
| --- | --- | --- |
| `audit` | [Reply_ProteinAudit](/reference/schemas/types/#reply-proteinaudit) or null | Native audit for the selected entry only. Optional. |
| `candidates` | list of [Reply_ProteinHeader](/reference/schemas/types/#reply-proteinheader) | Competing exact source identities when a gene does not resolve uniquely. Optional. |
| `header` | [Reply_ProteinHeader](/reference/schemas/types/#reply-proteinheader) or null | Exact resolved native header; absent for ambiguity or failure. Optional. |
| `input` | [Reply_ProteinInput](/reference/schemas/types/#reply-proteininput) | Original caller identity. |
| `more` | list of [Reply_ProteinSection](/reference/schemas/types/#reply-proteinsection) | Supported sections not requested. Optional. |
| `sections` | [Reply_ProteinSections](#reply-proteinsections) or null | Requested bounded sections only. Optional. |
| `status` | [Reply_Status](/reference/schemas/types/#reply-status) | Outcome of this input. |

### Reply_ProteinSections

Independent source section states; all discloses bounded first pages only.

| Field | Type | Meaning |
| --- | --- | --- |
| `complexes` | [Reply_Section_ProteinComplexes](#reply-section-proteincomplexes) or null | Requested exact participant-admitted header page. Optional. |
| `domains` | [Reply_Section_ProteinDomains](#reply-section-proteindomains) or null | Requested InterPro match page. Optional. |
| `features` | [Reply_Section_ProteinSectionPage](/reference/schemas/types/#reply-section-proteinsectionpage) of [Reply_ProteinSectionPage](/reference/schemas/types/#reply-proteinsectionpage) of [Reply_ProteinFeature](/reference/schemas/types/#reply-proteinfeature) or null | Requested UniProt feature page. Optional. |
| `function` | [Reply_Section_ProteinFunctionResult](#reply-section-proteinfunctionresult) or null | Requested whole native function text and evidence. Optional. |
| `isoforms` | [Reply_Section_ProteinSectionPage](/reference/schemas/types/#reply-section-proteinsectionpage) of [Reply_ProteinSectionPage](/reference/schemas/types/#reply-proteinsectionpage) of [Reply_ProteinIsoform](/reference/schemas/types/#reply-proteinisoform) or null | Requested native alternative-product page. Optional. |
| `sequence` | [Reply_Section_ProteinSequenceResult](#reply-section-proteinsequenceresult) or null | Requested direct sequence; absent/unavailable states remain explicit. Optional. |
| `structures` | [Reply_Section_ProteinSectionPage](/reference/schemas/types/#reply-section-proteinsectionpage) of [Reply_ProteinSectionPage](/reference/schemas/types/#reply-proteinsectionpage) of [Reply_ProteinStructureRef](/reference/schemas/types/#reply-proteinstructureref) or null | Requested structure-link page. Optional. |

### Reply_Section_ProteinComplexes

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_ProteinComplexes](/reference/schemas/types/#reply-proteincomplexes)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply-sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_Section_ProteinDomains

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_ProteinDomains](/reference/schemas/types/#reply-proteindomains)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply-sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_Section_ProteinFunctionResult

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_ProteinFunctionResult](/reference/schemas/types/#reply-proteinfunctionresult)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply-sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_Section_ProteinSequenceResult

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_ProteinSequenceResult](/reference/schemas/types/#reply-proteinsequenceresult)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply-sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

</details>

## Next

[`biomcp get protein`](/reference/commands/get-protein/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
