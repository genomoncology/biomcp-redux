---
title: Reply of biomcp get drug approvals section (biomcp.drug_approvals/3)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get drug approvals:
  a real example and the fields of schema biomcp.drug_approvals/3."
goal: "Parse the biomcp.drug_approvals/3 reply: see a real example and what each
  field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.drug_approvals/3
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get drug approvals`](/reference/commands/get-drug/) replies with `schema` set to `biomcp.drug_approvals/3`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp get drug NDA202429 approvals --source drugsfda --section-limit 20
biomcp get drug NDA220910 approvals --source drugsfda
```

`biomcp schema biomcp.drug_approvals/3` prints the full JSON Schema.

## Example reply

`biomcp get drug NDA220910 approvals --source drugsfda` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.drug_approvals/3",
  "status": "complete",
  "query": {
    "input": "NDA220910",
    "source": "drugsfda",
    "section": ["approvals"]
  },
  "result": {
    "identity": {
      "id": "NDA220910",
      "source": "drugsfda",
      "identity_level": "fda_application"
    },
    "products": [
      {…},
      … 1 more
    ],
    "source_updated": "2026-10-26",
    "approvals": {
      "state": "data",
      "source": "drugsfda",
      "data": {…}
    }
  },
  "page": {"offset": 0, "returned": 1, "total": 1},
  "sources": [
    {
      "source": "drugsfda",
      "use": "open",
      "terms": "openFDA terms; original label rights retained",
      "terms_url": "https://open.fda.gov/terms/",
      "state": "ok",
      "count": 1,
      "fetched": 1
    }
  ],
  "next": [
    {
      "kind": "disambiguate",
…
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [DrugApprovals](#drugapprovals). `query` holds [DrugGet](/reference/schemas/types/#drugget), a shared type. `query` repeats the request as BioMCP read it.

### DrugApprovals

Explicit native application/product history without a global approval flag.

| Field | Type | Meaning |
| --- | --- | --- |
| `approvals` | [Section](/reference/schemas/types/#section) of [DrugRecords](/reference/schemas/types/#drugrecords) of [DrugFdaSubmission](#drugfdasubmission) | Existing typed section state, preserving empty, unavailable and skipped outcomes. |
| `identity` | [DrugHeader](/reference/schemas/types/#drugheader) | Selected native identity, checked independently of section facts. |
| `products` | list of [DrugFdaProduct](/reference/schemas/types/#drugfdaproduct) | Native products, filtered only by an explicit product number; independent of submissions. |
| `source_updated` | [PartialDate](/reference/schemas/types/#partialdate) or null | Native FDA dataset last_updated date, independent of submission dates and retrieval. Optional. |

### DrugFdaDocument

A native application document link; the command does not download it.

| Field | Type | Meaning |
| --- | --- | --- |
| `date` | [PartialDate](/reference/schemas/types/#partialdate) | Native document date, independent of event dates. |
| `id` | [Text](/reference/schemas/types/#text) | Source document identifier. |
| `type` | [Text](/reference/schemas/types/#text) | Original document type. |
| `url` | [Text](/reference/schemas/types/#text) | Original source URL. |

### DrugFdaProperty

A source-stated submission property, without interpretation.

| Field | Type | Meaning |
| --- | --- | --- |
| `code` | [Text](/reference/schemas/types/#text) | Original property code. |

### DrugFdaSubmission

One application-owned submission with common event facts and native FDA context.

| Field | Type | Meaning |
| --- | --- | --- |
| `application_docs` | list of [DrugFdaDocument](#drugfdadocument) or null | Native documents; document dates remain separate from the submission status date. Optional. |
| `event` | [AuthorizationEvent](/reference/schemas/types/#authorizationevent) | Historical native status, type and status date; no product applicability is implied. |
| `review_priority` | [Text](/reference/schemas/types/#text) or null | Original review priority. Optional. |
| `submission_class_code` | [Text](/reference/schemas/types/#text) or null | Native submission class code. Optional. |
| `submission_class_code_description` | [Text](/reference/schemas/types/#text) or null | Native submission class description. Optional. |
| `submission_number` | [Text](/reference/schemas/types/#text) | Original submission number, scoped by application and event kind. |
| `submission_property_type` | list of [DrugFdaProperty](#drugfdaproperty) or null | Native property collection; missing stays missing and an empty list stays empty. Optional. |

## Next

[`biomcp get drug`](/reference/commands/get-drug/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
