---
title: Reply of biomcp get diagnostic (biomcp.diagnostic-summary/1)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get diagnostic: the
  fields of schema biomcp.diagnostic-summary/1."
goal: "Parse the biomcp.diagnostic-summary/1 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.diagnostic-summary/1
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get diagnostic`](/reference/commands/get-diagnostic/) and [`biomcp get`](/reference/commands/get/) reply with `schema` set to `biomcp.diagnostic-summary/1`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.diagnostic-summary/1` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_DiagnosticSummary](#reply-diagnosticsummary). `query` holds [Reply_DiagnosticGet](/reference/schemas/types/#reply-diagnosticget), a shared type. `query` repeats the request as BioMCP read it.

### Reply_DiagnosticSummary

FDA-published SSED text for one PMA base application.

| Field | Type | Meaning |
| --- | --- | --- |
| `document` | [Reply_PdfText](/reference/schemas/types/#reply-pdftext) | Mechanical document text with explicit extraction state. |

## Next

[`biomcp get diagnostic`](/reference/commands/get-diagnostic/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
