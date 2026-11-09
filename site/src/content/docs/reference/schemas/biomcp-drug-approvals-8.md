---
title: Reply of biomcp get drug (biomcp.drug-approvals/8)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get drug: the fields
  of schema biomcp.drug-approvals/8."
goal: "Parse the biomcp.drug-approvals/8 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.drug-approvals/8
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get drug`](/reference/commands/get-drug/) and [`biomcp get`](/reference/commands/get/) reply with `schema` set to `biomcp.drug-approvals/8`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.drug-approvals/8` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_DrugApprovals](/reference/schemas/types/#reply_drugapprovals), a shared type. `query` holds [Reply_DrugGet](/reference/schemas/types/#reply_drugget), a shared type. `query` repeats the request as BioMCP read it.

Every type this reply uses is shared with other replies, so its fields are on the shared types page.

## Next

[`biomcp get drug`](/reference/commands/get-drug/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
