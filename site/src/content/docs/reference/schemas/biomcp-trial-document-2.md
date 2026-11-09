---
title: Reply of biomcp trial document (biomcp.trial_document/2)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp trial document: a
  real example and the fields of schema biomcp.trial_document/2."
goal: "Parse the biomcp.trial_document/2 reply: see a real example and what each
  field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.trial_document/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp trial document`](/reference/commands/trial-document/) replies with `schema` set to `biomcp.trial_document/2`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp trial document NCT04303780 SAP_001.pdf --save protocol.pdf --replace
```

`biomcp schema biomcp.trial_document/2` prints the full JSON Schema.

## Example reply

`biomcp trial document NCT04303780 SAP_001.pdf --save protocol.pdf --replace` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.trial_document/2",
  "status": "failed",
  "query": {
    "input": "NCT04303780",
    "file": "SAP_001.pdf",
    "save": "protocol.pdf",
    "replace": true
  },
  "result": {
    "file": {
      "state": "unavailable",
      "source": "ctgov",
      "reason": "proxy"
    }
  },
  "sources": [
    {
      "source": "ctgov",
      "use": "restricted",
      "terms": "ClinicalTrials.gov attribution and source processing date; pu…",
      "terms_url": "https://clinicaltrials.gov/about-site/terms-conditions",
      "state": "failed",
      "reason": "proxy"
    }
  ],
  "next": [
    {
      "kind": "configure",
      "args": ["help", "network", "proxy"]
    }
  ]
}
```

The source did not answer that run, so `status` is `failed` and `result` is missing. A reply that succeeds fills `result` with the fields below.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [TrialDocumentResult](#trialdocumentresult). `query` holds [TrialDocumentGet](#trialdocumentget). `query` repeats the request as BioMCP read it.

### TrialDocumentResult

Original trial PDF download, never document text.

| Field | Type | Meaning |
| --- | --- | --- |
| `file` | [Section](/reference/schemas/types/#section) of [TrialDocumentSaved](#trialdocumentsaved) | Saved receipt, confirmed absence, or a source failure. |

### TrialDocumentGet

Download one exact ClinicalTrials.gov manifest PDF to an explicit caller path; never read its text. CLI and stdio MCP support saves; HTTP refuses with the source URL.

| Field | Type | Meaning |
| --- | --- | --- |
| `file` | [Text](/reference/schemas/types/#text) | Exact filename from the registry manifest; never a URL or inferred path. |
| `input` | [NctId](/reference/schemas/types/#nctid) | Exact registry identity. |
| `replace` | boolean | Replace an existing file only when explicitly requested. Optional. |
| `save` | [Text](/reference/schemas/types/#text) | Caller-owned file or existing folder. |

### Other types in this reply

One more type appear only in this reply: [TrialDocumentSaved](#trialdocumentsaved). Open the list below to read their fields.

<details>
<summary>Fields of the 1 other type</summary>

### TrialDocumentSaved

Receipt for exactly the original source bytes written by the caller.

| Field | Type | Meaning |
| --- | --- | --- |
| `bytes` | integer (uint64) | Saved byte count. |
| `document` | [TrialDocument](/reference/schemas/types/#trialdocument) | Original manifest metadata and publisher URL. |
| `path` | [Text](/reference/schemas/types/#text) | Absolute caller-owned path. |
| `sha256` | [Text](/reference/schemas/types/#text) | SHA-256 of the complete saved bytes. |

</details>

## Next

[`biomcp trial document`](/reference/commands/trial-document/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
