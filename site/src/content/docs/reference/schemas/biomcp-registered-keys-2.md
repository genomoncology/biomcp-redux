---
title: Reply of biomcp key list (biomcp.registered-keys/2)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp key list: a real
  example and the fields of schema biomcp.registered-keys/2."
goal: "Parse the biomcp.registered-keys/2 reply: see a real example and what
  each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.registered-keys/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp key list`](/reference/commands/key-list/) replies with `schema` set to `biomcp.registered-keys/2`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp key list
```

`biomcp schema biomcp.registered-keys/2` prints the full JSON Schema.

## Example reply

`biomcp key list` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.registered-keys/2",
  "status": "complete",
  "query": null,
  "result": {
    "entries": [
      {
        "source": "ncbi",
        "config": "keys.ncbi_api_key",
        "environment": "NCBI_API_KEY",
        "state": "none"
      },
      {
        "source": "semantic-scholar",
        "config": "keys.semantic_scholar_api_key",
        "environment": "S2_API_KEY",
        "state": "none"
      },
      … 5 more
    ]
  }
}
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [RegisteredKeys](#registeredkeys). `query` repeats the request as BioMCP read it.

### RegisteredKeys

Results of human credential registration.

| Field | Type | Meaning |
| --- | --- | --- |
| `entries` | list of [RegisteredKey](#registeredkey) | Redacted credential identities. |

### Other types in this reply

2 more types appear only in this reply: [KeyState](#keystate), [RegisteredKey](#registeredkey). Open the list below to read their fields.

<details>
<summary>Fields of the 2 other types</summary>

### KeyState

Where the command will obtain a credential; never its value.

Values: `stored`, `environment`, `none`.

### RegisteredKey

Credential identity and redacted presence.

| Field | Type | Meaning |
| --- | --- | --- |
| `config` | [Text](/reference/schemas/types/#text) | Configuration name. |
| `environment` | [Text](/reference/schemas/types/#text) | Environment fallback name. |
| `source` | [Text](/reference/schemas/types/#text) | Source accepted by key set. |
| `state` | [KeyState](#keystate) | Current credential origin. |

</details>

## Next

[`biomcp key list`](/reference/commands/key-list/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
