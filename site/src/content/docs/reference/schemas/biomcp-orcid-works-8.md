---
title: Reply of biomcp author papers (biomcp.orcid-works/8)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp author papers: the
  fields of schema biomcp.orcid-works/8."
goal: "Parse the biomcp.orcid-works/8 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.orcid-works/8
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp author papers`](/reference/commands/author-papers/) and [`biomcp author`](/reference/commands/author/) reply with `schema` set to `biomcp.orcid-works/8`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.orcid-works/8` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_OrcidWorks](#reply_orcidworks). `query` holds [Reply_AuthorPapersQuery](/reference/schemas/types/#reply_authorpapersquery), a shared type. `query` repeats the request as BioMCP read it.

### Reply_OrcidWorks

Locally paged groups from one complete public ORCID works reply.

| Field | Type | Meaning |
| --- | --- | --- |
| `author` | [Reply_OrcidId](#reply_orcidid) | Requested exact ORCID record. |
| `collection_scope` | [Reply_OrcidWorksScope](#reply_orcidworksscope) | The whole source reply supplies every local page. |
| `groups` | list of [Reply_OrcidWorkGroup](#reply_orcidworkgroup) | Whole groups in native order, with every public assertion. Optional. |
| `observed_groups` | integer (uint64) | Observed groups in the complete received reply, never a publication total. |

### Other types in this reply

8 more types appear only in this reply: [Reply_OrcidAssertionSource](#reply_orcidassertionsource), [Reply_OrcidClientId](#reply_orcidclientid), [Reply_OrcidExternalId](#reply_orcidexternalid), [Reply_OrcidId](#reply_orcidid), [Reply_OrcidPublicVisibility](#reply_orcidpublicvisibility), [Reply_OrcidWorkAssertion](#reply_orcidworkassertion), [Reply_OrcidWorkGroup](#reply_orcidworkgroup), [Reply_OrcidWorksScope](#reply_orcidworksscope). Open the list below to read their fields.

<details>
<summary>Fields of the 8 other types</summary>

### Reply_OrcidAssertionSource

Source and assertion origin remain distinct provider claims.

| Field | Type | Meaning |
| --- | --- | --- |
| `assertion_origin_client_id` | [Reply_OrcidClientId](#reply_orcidclientid) or null | Native origin client ID. Optional. |
| `assertion_origin_name` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Native origin display name. Optional. |
| `assertion_origin_orcid` | [Reply_OrcidId](#reply_orcidid) or null | Native assertion origin ORCID iD. Optional. |
| `source_client_id` | [Reply_OrcidClientId](#reply_orcidclientid) or null | Native ORCID client ID; no person identity. Optional. |
| `source_name` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Native source display name. Optional. |
| `source_orcid` | [Reply_OrcidId](#reply_orcidid) or null | Native source ORCID iD. Optional. |

### Reply_OrcidClientId

A checked native ORCID client application identifier. The parser additionally validates ORCID checksums for legacy numeric identifiers.

Type: string.

### Reply_OrcidExternalId

Native ORCID external identifier before any article pivot.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [Reply_Text](/reference/schemas/types/#reply_text) | Unchanged native identifier value. |
| `namespace` | [Reply_IdentifierNamespace](/reference/schemas/types/#reply_identifiernamespace) | Open native external-id-type. |
| `relationship` | [Reply_Text](/reference/schemas/types/#reply_text) | Open native relationship, including self and part-of. Optional. |
| `url` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Native identifier URL. Optional. |

### Reply_OrcidId

A checksummed ORCID iD, identifying only the requested ORCID record. The parser additionally validates the ISO 7064 checksum.

Type: string.

### Reply_OrcidPublicVisibility

Only public source visibility is admitted; legacy uppercase is an alias.

Values: `public`, `PUBLIC`.

### Reply_OrcidWorkAssertion

One public summary, retained in native group order without representative selection.

| Field | Type | Meaning |
| --- | --- | --- |
| `display_index` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Native display index retained without sorting. Optional. |
| `external_ids` | list of [Reply_OrcidExternalId](#reply_orcidexternalid) | Every native assertion identifier in order. Optional. |
| `journal_title` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Native journal title. Optional. |
| `publication_date` | [Reply_PartialDate](/reference/schemas/types/#reply_partialdate) or null | Native partial publication date. Optional. |
| `put_code` | integer (uint64) | Source assertion identifier scoped to this ORCID record. |
| `source` | [Reply_OrcidAssertionSource](#reply_orcidassertionsource) or null | Native source and origin provenance. Optional. |
| `subtitle` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Native subtitle. Optional. |
| `title` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Native title; some summaries have none. Optional. |
| `translated_title` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Native translated title. Optional. |
| `url` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Native work URL. Optional. |
| `visibility` | [Reply_OrcidPublicVisibility](#reply_orcidpublicvisibility) | Only public assertions belong to this product. |
| `work_type` | [Reply_Text](/reference/schemas/types/#reply_text) | Open native work type. Optional. |

### Reply_OrcidWorkGroup

One native work group is one group, regardless of assertion multiplicity.

| Field | Type | Meaning |
| --- | --- | --- |
| `assertions` | list of [Reply_OrcidWorkAssertion](#reply_orcidworkassertion) | Every public summary in native source order. Optional. |
| `external_ids` | list of [Reply_OrcidExternalId](#reply_orcidexternalid) | Native group identifiers in source order. Optional. |

### Reply_OrcidWorksScope

Scope of one received ORCID collection.

Values: `whole-public-works-reply`.

</details>

## Next

[`biomcp author papers`](/reference/commands/author-papers/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
