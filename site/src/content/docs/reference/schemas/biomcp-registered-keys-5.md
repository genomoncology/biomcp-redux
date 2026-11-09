---
title: Reply of biomcp key list (biomcp.registered-keys/5)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp key list: the fields
  of schema biomcp.registered-keys/5."
goal: "Parse the biomcp.registered-keys/5 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.registered-keys/5
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp key list`](/reference/commands/key-list/), [`biomcp key set`](/reference/commands/key-set/), [`biomcp key remove`](/reference/commands/key-remove/) and [`biomcp key`](/reference/commands/key/) reply with `schema` set to `biomcp.registered-keys/5`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.registered-keys/5` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_RegisteredKeys](#reply_registeredkeys). `query` holds [Reply_EmptyQuery](/reference/schemas/types/#reply_emptyquery), a shared type. `query` repeats the request as BioMCP read it.

### Reply_RegisteredKeys

Results of human credential registration.

| Field | Type | Meaning |
| --- | --- | --- |
| `entries` | list of [Reply_RegisteredKey](#reply_registeredkey) | Redacted credential identities. Optional. |

### Other types in this reply

3 more types appear only in this reply: [Reply_KeyState](#reply_keystate), [Reply_KeyTier](#reply_keytier), [Reply_RegisteredKey](#reply_registeredkey). Open the list below to read their fields.

<details>
<summary>Fields of the 3 other types</summary>

### Reply_KeyState

Where the command will obtain a credential; never its value.

Values: `stored`, `environment`, `none`.

### Reply_KeyTier

Whether hosted calls may use the server's configured source credential.

Values: `no-fallback`, `fallback`.

### Reply_RegisteredKey

Credential identity and redacted presence.

| Field | Type | Meaning |
| --- | --- | --- |
| `config` | [Reply_Text](/reference/schemas/types/#reply_text) | Configuration name. |
| `environment` | [Reply_Text](/reference/schemas/types/#reply_text) | Environment fallback name. |
| `header` | [Reply_Text](/reference/schemas/types/#reply_text) | Header accepted by the hosted HTTP transport. |
| `source` | [Reply_Text](/reference/schemas/types/#reply_text) | Source accepted by key set. |
| `state` | [Reply_KeyState](#reply_keystate) | Current credential origin. |
| `tier` | [Reply_KeyTier](#reply_keytier) | Declared server credential fallback policy. |
| `usage` | [Reply_Use](/reference/schemas/types/#reply_use) | Publisher use label for the whole source. |

</details>

## Next

[`biomcp key list`](/reference/commands/key-list/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
