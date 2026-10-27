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

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp author papers`](/reference/commands/author-papers/) replies with `schema` set to `biomcp.orcid_works/2`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp author papers orcid:0000-0002-1825-0097
```

`biomcp schema biomcp.orcid_works/2` prints the full JSON Schema.

## Example reply

`biomcp author papers orcid:0000-0002-1825-0097` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

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
      {
        "external_ids": [
          {"kind": "doi", "value": "10.5555/12345680", "relationship": "self"}
        ],
        "assertions": [
          {…},
          … 1 more
        ]
      },
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
    {
      "kind": "pivot",
      "args": ["get", "article", "10.5555/12345680"]
    },
    … 4 more
  ]
}
```

## Fields

`result` holds [OrcidWorks](#orcidworks). `query` holds [AuthorPapersQuery](/reference/schemas/types/#authorpapersquery), a shared type. `query` repeats the request as BioMCP read it.

### OrcidWorks

Locally paged groups from one complete public ORCID works reply.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `author` | [OrcidId](#orcidid) | yes | Requested exact ORCID record. |
| `collection_scope` | [OrcidWorksScope](#orcidworksscope) | yes | The whole source reply supplies every local page. |
| `groups` | list of [OrcidWorkGroup](#orcidworkgroup) | yes | Whole groups in native order, with every public assertion. |
| `observed_groups` | integer (uint64) | yes | Observed groups in the complete received reply, never a publication total. |

### OrcidAssertionSource

Source and assertion origin remain distinct provider claims.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `assertion_origin_client_id` | [OrcidClientId](#orcidclientid) or null | no | Native origin client ID. |
| `assertion_origin_name` | [Text](/reference/schemas/types/#text) or null | no | Native origin display name. |
| `assertion_origin_orcid` | [OrcidId](#orcidid) or null | no | Native assertion origin ORCID iD. |
| `source_client_id` | [OrcidClientId](#orcidclientid) or null | no | Native ORCID client ID; no person identity. |
| `source_name` | [Text](/reference/schemas/types/#text) or null | no | Native source display name. |
| `source_orcid` | [OrcidId](#orcidid) or null | no | Native source ORCID iD. |

### OrcidClientId

Type: string.

### OrcidExternalId

Native ORCID external identifier before any article pivot.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `kind` | [Text](/reference/schemas/types/#text) | yes | Open native external-id-type. |
| `relationship` | [Text](/reference/schemas/types/#text) | yes | Open native relationship, including self and part-of. |
| `url` | [Text](/reference/schemas/types/#text) or null | no | Native identifier URL. |
| `value` | [Text](/reference/schemas/types/#text) | yes | Unchanged native identifier value. |

### OrcidId

Type: string.

### OrcidPublicVisibility

Only public source visibility is admitted; legacy uppercase is an alias.

Values: `public`, `PUBLIC`.

### OrcidWorkAssertion

One public summary, retained in native group order without representative selection.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `display_index` | [Text](/reference/schemas/types/#text) or null | no | Native display index retained without sorting. |
| `external_ids` | list of [OrcidExternalId](#orcidexternalid) | yes | Every native assertion identifier in order. |
| `journal_title` | [Text](/reference/schemas/types/#text) or null | no | Native journal title. |
| `publication_date` | [PartialDate](/reference/schemas/types/#partialdate) or null | no | Native partial publication date. |
| `put_code` | integer (uint64) | yes | Source assertion identifier scoped to this ORCID record. |
| `source` | [OrcidAssertionSource](#orcidassertionsource) | yes | Native source and origin provenance. |
| `subtitle` | [Text](/reference/schemas/types/#text) or null | no | Native subtitle. |
| `title` | [Text](/reference/schemas/types/#text) or null | no | Native title; some summaries have none. |
| `translated_title` | [Text](/reference/schemas/types/#text) or null | no | Native translated title. |
| `url` | [Text](/reference/schemas/types/#text) or null | no | Native work URL. |
| `visibility` | [OrcidPublicVisibility](#orcidpublicvisibility) | yes | Only public assertions belong to this product. |
| `work_type` | [Text](/reference/schemas/types/#text) | yes | Open native work type. |

### OrcidWorkGroup

One native work group is one group, regardless of assertion multiplicity.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `assertions` | list of [OrcidWorkAssertion](#orcidworkassertion) | yes | Every public summary in native source order. |
| `external_ids` | list of [OrcidExternalId](#orcidexternalid) | yes | Native group identifiers in source order. |

### OrcidWorksScope

Scope of one received ORCID collection.

Values: `whole_public_works_reply`.

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp author papers`](/reference/commands/author-papers/) gives the command's arguments.
