---
title: Reply of biomcp author papers (biomcp.orcid_works/2)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp author papers: a
  real example and the fields of schema biomcp.orcid_works/2."
goal: "Parse the biomcp.orcid_works/2 reply: see a real example and what each
  field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.orcid_works/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp author papers`](/reference/commands/author-papers/) replies with `schema` set to `biomcp.orcid_works/2`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp author papers orcid:0000-0002-1825-0097
```

`biomcp schema biomcp.orcid_works/2` prints the full JSON Schema.

## Example reply

`biomcp author papers orcid:0000-0002-1825-0097` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.orcid_works/2",
  "status": "complete",
  "query": {
    "id": "orcid:0000-0002-1825-0097",
    "limit": 5,
    "offset": 0
  },
  "result": {
    "collection_scope": "whole_public_works_reply",
    "author": "0000-0002-1825-0097",
    "groups": [
      {…},
      … 4 more
    ],
    "observed_groups": 6
  },
  "page": {
    "offset": 0,
    "returned": 5,
    "total": 6,
    "pool": {
      "id": "821902d2265a5ef82801f3b2d2d96856",
      "size": 6,
      "created": 1793140306,
      "expires": 1793226706
    }
  },
  "sources": [
    {
      "source": "orcid",
      "use": "noncommercial",
      "terms": "ORCID Public API Terms of Service",
      "terms_url": "https://info.orcid.org/public-client-terms-of-service/",
      "state": "ok"
    }
  ],
  "next": [
    {"kind": "pivot", "args": ["get", "article", "10.5555/12345680"]},
    … 4 more
…
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [OrcidWorks](#orcidworks). `query` holds [AuthorPapersQuery](/reference/schemas/types/#authorpapersquery), a shared type. `query` repeats the request as BioMCP read it.

### OrcidWorks

Locally paged groups from one complete public ORCID works reply.

| Field | Type | Meaning |
| --- | --- | --- |
| `author` | [OrcidId](#orcidid) | Requested exact ORCID record. |
| `collection_scope` | [OrcidWorksScope](#orcidworksscope) | The whole source reply supplies every local page. |
| `groups` | list of [OrcidWorkGroup](#orcidworkgroup) | Whole groups in native order, with every public assertion. |
| `observed_groups` | integer (uint64) | Observed groups in the complete received reply, never a publication total. |

### OrcidAssertionSource

Source and assertion origin remain distinct provider claims.

| Field | Type | Meaning |
| --- | --- | --- |
| `assertion_origin_client_id` | [OrcidClientId](#orcidclientid) or null | Native origin client ID. Optional. |
| `assertion_origin_name` | [Text](/reference/schemas/types/#text) or null | Native origin display name. Optional. |
| `assertion_origin_orcid` | [OrcidId](#orcidid) or null | Native assertion origin ORCID iD. Optional. |
| `source_client_id` | [OrcidClientId](#orcidclientid) or null | Native ORCID client ID; no person identity. Optional. |
| `source_name` | [Text](/reference/schemas/types/#text) or null | Native source display name. Optional. |
| `source_orcid` | [OrcidId](#orcidid) or null | Native source ORCID iD. Optional. |

### OrcidClientId

Type: string.

### OrcidExternalId

Native ORCID external identifier before any article pivot.

| Field | Type | Meaning |
| --- | --- | --- |
| `kind` | [Text](/reference/schemas/types/#text) | Open native external-id-type. |
| `relationship` | [Text](/reference/schemas/types/#text) | Open native relationship, including self and part-of. |
| `url` | [Text](/reference/schemas/types/#text) or null | Native identifier URL. Optional. |
| `value` | [Text](/reference/schemas/types/#text) | Unchanged native identifier value. |

### OrcidId

Type: string.

### OrcidPublicVisibility

Only public source visibility is admitted; legacy uppercase is an alias.

Values: `public`, `PUBLIC`.

### OrcidWorkAssertion

One public summary, retained in native group order without representative selection.

| Field | Type | Meaning |
| --- | --- | --- |
| `display_index` | [Text](/reference/schemas/types/#text) or null | Native display index retained without sorting. Optional. |
| `external_ids` | list of [OrcidExternalId](#orcidexternalid) | Every native assertion identifier in order. |
| `journal_title` | [Text](/reference/schemas/types/#text) or null | Native journal title. Optional. |
| `publication_date` | [PartialDate](/reference/schemas/types/#partialdate) or null | Native partial publication date. Optional. |
| `put_code` | integer (uint64) | Source assertion identifier scoped to this ORCID record. |
| `source` | [OrcidAssertionSource](#orcidassertionsource) | Native source and origin provenance. |
| `subtitle` | [Text](/reference/schemas/types/#text) or null | Native subtitle. Optional. |
| `title` | [Text](/reference/schemas/types/#text) or null | Native title; some summaries have none. Optional. |
| `translated_title` | [Text](/reference/schemas/types/#text) or null | Native translated title. Optional. |
| `url` | [Text](/reference/schemas/types/#text) or null | Native work URL. Optional. |
| `visibility` | [OrcidPublicVisibility](#orcidpublicvisibility) | Only public assertions belong to this product. |
| `work_type` | [Text](/reference/schemas/types/#text) | Open native work type. |

### OrcidWorkGroup

One native work group is one group, regardless of assertion multiplicity.

| Field | Type | Meaning |
| --- | --- | --- |
| `assertions` | list of [OrcidWorkAssertion](#orcidworkassertion) | Every public summary in native source order. |
| `external_ids` | list of [OrcidExternalId](#orcidexternalid) | Native group identifiers in source order. |

### OrcidWorksScope

Scope of one received ORCID collection.

Values: `whole_public_works_reply`.

## Next

[`biomcp author papers`](/reference/commands/author-papers/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
