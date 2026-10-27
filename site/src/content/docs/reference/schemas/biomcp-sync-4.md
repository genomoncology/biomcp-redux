---
title: Sync reply (biomcp.sync/4)
description: "The JSON reply BioMCP 1.0.0 prints for no command (sync): a real
  example and the fields of schema biomcp.sync/4."
goal: "Parse the biomcp.sync/4 reply: see a real example and what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.sync/4
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. The reply schema `biomcp.sync/4` is in the program, but no command returns it as a whole reply. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp sync hgnc
biomcp sync clingen
```

`biomcp schema biomcp.sync/4` prints the full JSON Schema.

## Example reply

`biomcp sync hgnc` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.sync/4",
  "status": "complete",
  "query": {"source": "hgnc"},
  "result": {"rows": 50582, "bytes": 17245227},
  "sources": [
    {
      "source": "hgnc",
      "use": "open",
      "terms": "CC0 1.0",
      "terms_url": "https://www.genenames.org/about/license/",
      "state": "ok"
    }
  ]
}
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [SyncResult](#syncresult). `query` holds [SyncQuery](/reference/schemas/types/#syncquery), a shared type. `query` repeats the request as BioMCP read it.

### SyncResult

Published [bundle](/local-data/) metadata after synchronization.

| Field | Type | Meaning |
| --- | --- | --- |
| `bytes` | integer (uint64) | Imported source-file bytes before native file decoding. |
| `duplicate_identities` | integer or null | Distinct repeated identities across the whole source version report. Optional. |
| `etag` | string or null | Native publisher ETag when supplied. Optional. |
| `last_modified` | string or null | Native publisher Last-Modified when supplied. Optional. |
| `rows` | integer (uint64) | Imported rows. |
| `snapshot` | [StudySnapshot](/reference/schemas/types/#studysnapshot) or null | Catalog content revision and acquisition time, separate from source headers. Optional. |

## Next

[Reply schemas](/reference/schemas/) lists every reply.
