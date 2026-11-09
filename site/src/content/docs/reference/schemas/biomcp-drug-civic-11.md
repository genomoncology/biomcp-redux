---
title: Reply of biomcp get drug (biomcp.drug-civic/11)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get drug: the fields
  of schema biomcp.drug-civic/11."
goal: "Parse the biomcp.drug-civic/11 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.drug-civic/11
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get drug`](/reference/commands/get-drug/) and [`biomcp get`](/reference/commands/get/) reply with `schema` set to `biomcp.drug-civic/11`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.drug-civic/11` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_DrugCivic](#reply_drugcivic). `query` holds [Reply_DrugGet](/reference/schemas/types/#reply_drugget), a shared type. `query` repeats the request as BioMCP read it.

### Reply_DrugCivic

Complete CIViC therapy/profile context with independent native claim continuations.

| Field | Type | Meaning |
| --- | --- | --- |
| `civic` | [Reply_Section_DrugCivicStreams](#reply_section_drugcivicstreams) | Existing typed section state, preserving empty, unavailable and skipped outcomes. |
| `identity` | [Reply_DrugHeader](/reference/schemas/types/#reply_drugheader) | Selected native identity, checked independently of section facts. |

### Other types in this reply

9 more types appear only in this reply: [Reply_CivicClaimPage_DrugGet](#reply_civicclaimpage_drugget), [Reply_DrugCivicClaim](#reply_drugcivicclaim), [Reply_DrugCivicMember](#reply_drugcivicmember), [Reply_DrugCivicPage](#reply_drugcivicpage), [Reply_DrugCivicProfile](#reply_drugcivicprofile), [Reply_DrugCivicStreams](#reply_drugcivicstreams), [Reply_DrugOncokbLookup](#reply_drugoncokblookup), [Reply_DrugOncokbMissing](#reply_drugoncokbmissing), [Reply_Section_DrugCivicStreams](#reply_section_drugcivicstreams). Open the list below to read their fields.

<details>
<summary>Fields of the 9 other types</summary>

### Reply_CivicClaimPage_DrugGet

Native stream bounds; evidence and assertions have separate cursors.

| Field | Type | Meaning |
| --- | --- | --- |
| `next` | [Reply_DrugGet](/reference/schemas/types/#reply_drugget) or null | Continue the same source candidate, profile and claim stream. Optional. |
| `returned` | integer (uint32) | Number of native rows displayed in this stream. |
| `total` | integer (uint64) | Native total for this stream. |

### Reply_DrugCivicClaim

One native claim with its complete source therapy set and molecular profile.

| Field | Type | Meaning |
| --- | --- | --- |
| `claim` | [Reply_CivicStatement](/reference/schemas/types/#reply_civicstatement) | Existing source claim mapping, including unmapped native facts. |
| `oncokb_lookup` | [Reply_DrugOncokbLookup](#reply_drugoncokblookup) or null | Scoped lookup of a source-listed genomic description, with no evidence transfer. Optional. |
| `oncokb_missing` | list of [Reply_DrugOncokbMissing](#reply_drugoncokbmissing) | Missing source context prevents a keyed variant annotation next command. Optional. |
| `profile` | [Reply_DrugCivicProfile](#reply_drugcivicprofile) or null | Source molecular profile context; its member expression is never collapsed. Optional. |
| `therapies` | list of [Reply_DrugHeader](/reference/schemas/types/#reply_drugheader) | Complete native therapy identities, without cross-source equivalence. Optional. |
| `therapy_interaction_type` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Native combination interaction label when supplied. Optional. |

### Reply_DrugCivicMember

A drug claim member with its source-owned build, without altering variant responses.

| Field | Type | Meaning |
| --- | --- | --- |
| `feature` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Native feature label, including fusion features. Optional. |
| `feature_record` | [Reply_Curie](/reference/schemas/types/#reply_curie) or null | Native feature identifier, distinct from the variant identifier. Optional. |
| `hgvs_descriptions` | list of [Reply_Text](/reference/schemas/types/#reply_text) | Every native notation, without selecting a transcript or repairing a spelling. Optional. |
| `label` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Whole source variant label. Optional. |
| `record` | [Reply_SourceRecord](/reference/schemas/types/#reply_sourcerecord) | Native variant identity and link. |
| `reference_build` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Native coordinates.referenceBuild spelling, including unsupported builds. Optional. |
| `subject` | [Reply_VariantRef](/reference/schemas/types/#reply_variantref) or null | Source-qualified subject when supported by the common model. Optional. |

### Reply_DrugCivicPage

One native claim stream and its own continuation.

| Field | Type | Meaning |
| --- | --- | --- |
| `page` | [Reply_CivicClaimPage_DrugGet](#reply_civicclaimpage_drugget) | Stream totals and continuation, independent of every other stream. |
| `records` | list of [Reply_DrugCivicClaim](#reply_drugcivicclaim) | Native claim order; each row carries the profile to which that claim belongs. Optional. |

### Reply_DrugCivicProfile

Profile context embedded in a therapy claim, with no invented profile totals.

| Field | Type | Meaning |
| --- | --- | --- |
| `compound` | boolean or null | Source complex-profile flag. Optional. |
| `label` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Complete source expression. Optional. |
| `members` | list of [Reply_DrugCivicMember](#reply_drugcivicmember) | Every source member, including members without a common-model subject. Optional. |
| `raw_name` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Exact source expression with native variant references. Optional. |
| `record` | [Reply_SourceRecord](/reference/schemas/types/#reply_sourcerecord) | Native profile identity and link. |
| `subject` | [Reply_VariantRef](/reference/schemas/types/#reply_variantref) or null | Checked source profile subject, independent of an allele join or a clinical claim. Optional. |

### Reply_DrugCivicStreams

Independent native therapy claim collections, without profile count inference.

| Field | Type | Meaning |
| --- | --- | --- |
| `assertions` | [Reply_DrugCivicPage](#reply_drugcivicpage) or null | Assertions filtered independently by the selected native therapy and optional profile. Optional. |
| `evidence` | [Reply_DrugCivicPage](#reply_drugcivicpage) or null | Evidence items filtered by the selected native therapy and optional profile. Optional. |

### Reply_DrugOncokbLookup

Link one source member to a checked variant lookup in its claim's tumor context.

| Field | Type | Meaning |
| --- | --- | --- |
| `label` | [Reply_Text](/reference/schemas/types/#reply_text) | Scope visible alongside the generated command. |
| `member` | [Reply_Text](/reference/schemas/types/#reply_text) | Native CIViC member ID to which the genomic description belongs. |
| `query` | [Reply_VariantGet](/reference/schemas/types/#reply_variantget) | Checked owning-entity request; input preserves the selected source HGVS. |

### Reply_DrugOncokbMissing

Context absent or insufficient for an exact source-owned annotation command.

Values: `gene`, `build`, `variant`, `tumor-type`.

### Reply_Section_DrugCivicStreams

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_DrugCivicStreams](#reply_drugcivicstreams)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply_sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

</details>

## Next

[`biomcp get drug`](/reference/commands/get-drug/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
