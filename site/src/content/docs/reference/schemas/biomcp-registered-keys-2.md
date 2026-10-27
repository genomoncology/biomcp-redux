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

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp key list`](/reference/commands/key-list/) replies with `schema` set to `biomcp.registered-keys/2`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp key list
```

`biomcp schema biomcp.registered-keys/2` prints the full JSON Schema.

## Example reply

`biomcp key list` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

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

`result` holds [RegisteredKeys](#registeredkeys). `query` repeats the request as BioMCP read it.

### RegisteredKeys

Results of human credential registration.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `entries` | list of [RegisteredKey](#registeredkey) | yes | Redacted credential identities. |

### KeyState

Where the command will obtain a credential; never its value.

Values: `stored`, `environment`, `none`.

### RegisteredKey

Credential identity and redacted presence.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `config` | [Text](/reference/schemas/types/#text) | yes | Configuration name. |
| `environment` | [Text](/reference/schemas/types/#text) | yes | Environment fallback name. |
| `source` | [Text](/reference/schemas/types/#text) | yes | Source accepted by key set. |
| `state` | [KeyState](#keystate) | yes | Current credential origin. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp key list`](/reference/commands/key-list/) gives the command's arguments.
