---
title: Reply of biomcp get study (biomcp.study-profiles/4)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get study: the
  fields of schema biomcp.study-profiles/4."
goal: "Parse the biomcp.study-profiles/4 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.study-profiles/4
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get study`](/reference/commands/get-study/) and [`biomcp get`](/reference/commands/get/) reply with `schema` set to `biomcp.study-profiles/4`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.study-profiles/4` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_StudyProfiles](/reference/schemas/types/#reply-studyprofiles), a shared type. `query` holds [Reply_StudyGet](/reference/schemas/types/#reply-studyget), a shared type. `query` repeats the request as BioMCP read it.

Every type this reply uses is shared with other replies, so its fields are on the shared types page.

## Next

[`biomcp get study`](/reference/commands/get-study/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
