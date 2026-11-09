---
title: Reply of biomcp trial document (biomcp.trial-document/7)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp trial document: the
  fields of schema biomcp.trial-document/7."
goal: "Parse the biomcp.trial-document/7 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.trial-document/7
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp trial document`](/reference/commands/trial-document/) and [`biomcp trial`](/reference/commands/trial/) reply with `schema` set to `biomcp.trial-document/7`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.trial-document/7` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_TrialDocumentResult](#reply-trialdocumentresult). `query` holds [Reply_TrialDocumentGet](#reply-trialdocumentget). `query` repeats the request as BioMCP read it.

### Reply_TrialDocumentResult

Original trial PDF download, never document text.

| Field | Type | Meaning |
| --- | --- | --- |
| `file` | [Reply_Section_TrialDocumentSaved](#reply-section-trialdocumentsaved) | Saved receipt, confirmed absence, or a source failure. |

### Reply_TrialDocumentGet

Download one exact ClinicalTrials.gov manifest PDF to an explicit caller path; never read its text. CLI and stdio MCP support saves; HTTP refuses with the source URL.

| Field | Type | Meaning |
| --- | --- | --- |
| `file` | [Reply_Text](/reference/schemas/types/#reply-text) | Exact filename from the registry manifest; never a URL or inferred path. |
| `input` | [Reply_NctId](/reference/schemas/types/#reply-nctid) | Exact registry identity. |
| `replace` | boolean | Replace an existing file only when explicitly requested. Optional. |
| `save` | [Reply_Text](/reference/schemas/types/#reply-text) | Caller-owned file or existing folder. |

### Other types in this reply

2 more types appear only in this reply: [Reply_Section_TrialDocumentSaved](#reply-section-trialdocumentsaved), [Reply_TrialDocumentSaved](#reply-trialdocumentsaved). Open the list below to read their fields.

<details>
<summary>Fields of the 2 other types</summary>

### Reply_Section_TrialDocumentSaved

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_TrialDocumentSaved](#reply-trialdocumentsaved)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply-sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_TrialDocumentSaved

Receipt for exactly the original source bytes written by the caller.

| Field | Type | Meaning |
| --- | --- | --- |
| `bytes` | integer (uint64) | Saved byte count. |
| `document` | [Reply_TrialDocument](/reference/schemas/types/#reply-trialdocument) | Original manifest metadata and publisher URL. |
| `path` | [Reply_Text](/reference/schemas/types/#reply-text) | Absolute caller-owned path. |
| `sha256` | [Reply_Text](/reference/schemas/types/#reply-text) | SHA-256 of the complete saved bytes. |

</details>

## Next

[`biomcp trial document`](/reference/commands/trial-document/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
