---
title: Reply of biomcp schema (biomcp.schema-catalog/4)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp schema: the fields
  of schema biomcp.schema-catalog/4."
goal: "Parse the biomcp.schema-catalog/4 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.schema-catalog/4
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp schema`](/reference/commands/schema/) replies with `schema` set to `biomcp.schema-catalog/4`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.schema-catalog/4` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_SchemaCatalog](#reply_schemacatalog). `query` holds [Reply_EmptyQuery](/reference/schemas/types/#reply_emptyquery), a shared type. `query` repeats the request as BioMCP read it.

### Reply_SchemaCatalog

Discoverable contracts shipped with this executable. JSON reply readers ignore unknown fields and unknown growable enum values within a major. Growable enums list known values in x-extensible-enum; query schemas remain closed.

| Field | Type | Meaning |
| --- | --- | --- |
| `schemas` | list of [Reply_SchemaEntry](#reply_schemaentry) | Every artifact in canonical identity order. Optional. |

### Other types in this reply

One more type appear only in this reply: [Reply_SchemaEntry](#reply_schemaentry). Open the list below to read their fields.

<details>
<summary>Fields of the 1 other type</summary>

### Reply_SchemaEntry

One schema identity with its own artifact documentation.

| Field | Type | Meaning |
| --- | --- | --- |
| `description` | string | One line of artifact documentation. |
| `id` | string | Canonical versioned identity accepted by schema ID. |

</details>

## Next

[`biomcp schema`](/reference/commands/schema/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
