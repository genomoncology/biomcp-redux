---
title: Reply of biomcp get variant (biomcp.variant-card/15)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get variant: the
  fields of schema biomcp.variant-card/15."
goal: "Parse the biomcp.variant-card/15 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.variant-card/15
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get variant`](/reference/commands/get-variant/) and [`biomcp get`](/reference/commands/get/) reply with `schema` set to `biomcp.variant-card/15`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.variant-card/15` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_VariantCard](#reply_variantcard). `query` holds [Reply_VariantGetQuery](#reply_variantgetquery). `query` repeats the request as BioMCP read it.

### Reply_VariantCard

A compact identity card, including every provider candidate and explicit rejection.

| Field | Type | Meaning |
| --- | --- | --- |
| `candidates` | list of [Reply_VariantHeader](/reference/schemas/types/#reply_variantheader) | All retained source identities. Optional. |
| `rejection` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Named input rejection from provider validation or a source-stated transcript gene mismatch. Optional. |
| `resolution` | [Reply_VariantResolution](/reference/schemas/types/#reply_variantresolution) | Identity outcome, without selecting the first candidate. |
| `source` | [Reply_Source](/reference/schemas/types/#reply_source) | Responsible identity source. |

### Reply_VariantGetQuery

Variant request with any assembly default that changed the source request.

| Field | Type | Meaning |
| --- | --- | --- |
| `build` | [Reply_Build](/reference/schemas/types/#reply_build) or null | Assembly MyVariant identity assembly defaults to GRCh38; chromosome genomic HGVS requires an explicit build. Optional. |
| `build_defaulted` | boolean or null | True when BioMCP selected the echoed assembly for MyVariant. Optional. |
| `civic_cursor` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Cursor Native CIViC cursor, retained unchanged. Optional. |
| `civic_kind` | [Reply_CivicClaimKind](/reference/schemas/types/#reply_civicclaimkind) or null | Claims Native CIViC claim collection within the selected profile. Optional. |
| `civic_profile` | integer or null | Selected CIViC molecular profile for native claim paging. Optional. |
| `evidence_level` | list of [Reply_CivicEvidenceLevel](/reference/schemas/types/#reply_civicevidencelevel) | A–E Native CIViC evidence levels; repeat to include several levels. Optional. |
| `evidence_status` | [Reply_CivicEvidenceStatus](/reference/schemas/types/#reply_civicevidencestatus) or null | Status Native CIViC evidence status; omitted preserves the source's existing default. Optional. |
| `input` | [Reply_VariantLookupInput](/reference/schemas/types/#reply_variantlookupinput) | rs/CA/clinvar:N/VCV/HGVS/VCF rsID, CAid, clinvar:N, VCV accession, reference-qualified HGVS or VCF-style genomic coordinates. |
| `limit` | integer (uint32) | Candidate rows per page; more remain reachable through typed next commands. Optional. |
| `offset` | integer (uint32) | Candidate rows to skip in source order. Optional. |
| `resolver` | [Reply_VariantResolver](/reference/schemas/types/#reply_variantresolver) | Resolver. Resolver for reference-qualified notation; never repairs the input locally. Optional. |
| `section` | list of [Reply_VariantSection](/reference/schemas/types/#reply_variantsection) | Named source sections; omitted by default. Each calls its own source. all keeps bounded pages and source skips; it never enables a missing key. Optional. |
| `section_limit` | integer (uint32) | Rows per named source section page; omitted uses the native source default. Optional. |
| `section_offset` | integer (uint32) | Source rows to skip independently of identity candidate paging. Optional. |
| `tumor_type` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Tumor Explicit tumor context for keyed OncoKB annotation. Optional. |

## Next

[`biomcp get variant`](/reference/commands/get-variant/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
