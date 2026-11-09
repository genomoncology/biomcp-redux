---
title: Reply of biomcp get pathway (biomcp.pathway-card/10)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get pathway: the
  fields of schema biomcp.pathway-card/10."
goal: "Parse the biomcp.pathway-card/10 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.pathway-card/10
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get pathway`](/reference/commands/get-pathway/) and [`biomcp get`](/reference/commands/get/) reply with `schema` set to `biomcp.pathway-card/10`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.pathway-card/10` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_PathwayCard](#reply-pathwaycard). `query` holds [Reply_PathwayGet](#reply-pathwayget). `query` repeats the request as BioMCP read it.

### Reply_PathwayCard

One checked source pathway with supported disclosures and whole native text.

| Field | Type | Meaning |
| --- | --- | --- |
| `description_truncated` | boolean or null | Whether the source itself shortened its description, rather than the client. Optional. |
| `events` | [Reply_Section_PathwayEvents](#reply-section-pathwayevents) or null | Requested bounded event page, including unresolved integer references. Optional. |
| `genes` | [Reply_Section_PathwayGenes](#reply-section-pathwaygenes) or null | Requested bounded membership page, including its explicit source state. Optional. |
| `pathway` | [Reply_Pathway](#reply-pathway) | Source-native identity, species, versions and requested description. |
| `release` | [Reply_Text](/reference/schemas/types/#reply-text) or null | Native Reactome database release, separate from pathway version. Optional. |
| `sections` | list of [Reply_PathwaySection](#reply-pathwaysection) | Source-supported named sections; WikiPathways excludes events. Optional. |

### Reply_PathwayGet

Get an exact human Reactome R-HSA or WikiPathways WP card. Read whole summary, genes, Reactome events or bounded all sections. Membership names a source and release; it implies no therapy, response or actionability. No enrichment is calculated.

| Field | Type | Meaning |
| --- | --- | --- |
| `fresh` | boolean | Request a new source observation. Optional. |
| `id` | string | Exact human Reactome or WikiPathways identity without version suffixes. |
| `pool` | [Reply_Text](/reference/schemas/types/#reply-text) | Retained snapshot, when continuing a source relation. Optional. |
| `section` | [Reply_PathwaySection](#reply-pathwaysection) | Omit for the card; WikiPathways has summary and genes sections. Optional. |
| `section_limit` | integer (uint32) | Bounded initial section size; each all section has its own first page. Optional. |
| `section_offset` | integer (uint32) | Native relation offset for a selected genes or events section. Optional. |

### Other types in this reply

6 more types appear only in this reply: [Reply_Pathway](#reply-pathway), [Reply_PathwayLiteratureRef](#reply-pathwayliteratureref), [Reply_PathwaySection](#reply-pathwaysection), [Reply_PathwaySummation](#reply-pathwaysummation), [Reply_Section_PathwayEvents](#reply-section-pathwayevents), [Reply_Section_PathwayGenes](#reply-section-pathwaygenes). Open the list below to read their fields.

<details>
<summary>Fields of the 6 other types</summary>

### Reply_Pathway

Neutral pathway facts, without display sections or source calls.

| Field | Type | Meaning |
| --- | --- | --- |
| `cited_in` | string or null | WikiPathways catalog citedIn cell, including a native empty cell. Optional. |
| `description` | string or null | Complete native description when present; empty is distinct from absent. Optional. |
| `id` | [Reply_PathwayId](/reference/schemas/types/#reply-pathwayid) | Checked source-owned identity. |
| `name` | [Reply_Text](/reference/schemas/types/#reply-text) or null | Whole native pathway name. Optional. |
| `references` | list of [Reply_PathwayLiteratureRef](#reply-pathwayliteratureref) or null | Native pathway-level literature references, separate from each summation. Optional. |
| `revision` | [Reply_Text](/reference/schemas/types/#reply-text) or null | WikiPathways native revision, separate from identity. Optional. |
| `species` | [Reply_PathwaySpecies](/reference/schemas/types/#reply-pathwayspecies) | Explicit source-stated human species. |
| `summations` | list of [Reply_PathwaySummation](#reply-pathwaysummation) or null | Native Reactome summation objects in source order; not concatenated. Optional. |
| `version` | [Reply_Text](/reference/schemas/types/#reply-text) or null | Whole native Reactome stIdVersion, separate from stable identity. Optional. |

### Reply_PathwayLiteratureRef

A native pathway-level literature reference; never inferred from prose.

| Field | Type | Meaning |
| --- | --- | --- |
| `db_id` | [Reply_Identifier_NonZeroU64](/reference/schemas/types/#reply-identifier-nonzerou64) | Native literature database reference. |
| `pubmed` | [Reply_Pmid](/reference/schemas/types/#reply-pmid) or null | Native PubMed identifier when supplied. Optional. |
| `title` | [Reply_Text](/reference/schemas/types/#reply-text) or null | Native title when supplied. Optional. |
| `url` | [Reply_Text](/reference/schemas/types/#reply-text) or null | Source record URL when supplied. Optional. |

### Reply_PathwaySection

Supported source-owned pathway disclosures.

| Value | Meaning |
| --- | --- |
| `summary` | Complete native source description. |
| `genes` | Namespaced native participants with exact resolved gene joins. |
| `events` | Native Reactome event objects and unresolved database references. |
| `all` | Each source-supported section's bounded first page. |

### Reply_PathwaySummation

One native summation, with its own database identity and whole text.

| Field | Type | Meaning |
| --- | --- | --- |
| `db_id` | [Reply_Identifier_NonZeroU64](/reference/schemas/types/#reply-identifier-nonzerou64) | Native summation database identity, not the pathway identity. |
| `text` | string or null | Complete native text; no sentence clipping or reference reassignment. Optional. |

### Reply_Section_PathwayEvents

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_PathwayEvents](/reference/schemas/types/#reply-pathwayevents)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply-sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_Section_PathwayGenes

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_PathwayGenes](/reference/schemas/types/#reply-pathwaygenes)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply-sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

</details>

## Next

[`biomcp get pathway`](/reference/commands/get-pathway/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
