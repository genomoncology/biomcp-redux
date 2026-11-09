---
title: Reply of biomcp variant normalize (biomcp.variant-normalization/8)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp variant normalize:
  the fields of schema biomcp.variant-normalization/8."
goal: "Parse the biomcp.variant-normalization/8 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.variant-normalization/8
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp variant normalize`](/reference/commands/variant-normalize/) and [`biomcp variant`](/reference/commands/variant/) reply with `schema` set to `biomcp.variant-normalization/8`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.variant-normalization/8` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_VariantNormalization](#reply-variantnormalization). `query` holds [Reply_VariantNormalize](#reply-variantnormalize). `query` repeats the request as BioMCP read it.

### Reply_VariantNormalization

Independent service results without local correction or transcript selection.

| Field | Type | Meaning |
| --- | --- | --- |
| `results` | list of [Reply_NormalizationResult](#reply-normalizationresult) | Service results in declared order. Optional. |

### Reply_VariantNormalize

Send unchanged reference-qualified notation to explicitly named services.

| Field | Type | Meaning |
| --- | --- | --- |
| `build` | [Reply_Build](/reference/schemas/types/#reply-build) or null | Caller assembly; required when VariantValidator is selected. Optional. |
| `hgvs` | string | Reference-qualified HGVS, or chromosome genomic HGVS with an explicit build for CAR. Checked for reference and syntax only; no full HGVS parser. |
| `limit` | integer (uint32) | Source mapping rows per service, with typed continuation for remaining rows. Optional. |
| `offset` | integer (uint32) | Source mapping rows to skip per service. Optional. |
| `source` | [Reply_NormalizationSource](#reply-normalizationsource) | Explicit service; all preserves independent results without consensus. |

### Other types in this reply

4 more types appear only in this reply: [Reply_NormalizationField](#reply-normalizationfield), [Reply_NormalizationMessage](#reply-normalizationmessage), [Reply_NormalizationResult](#reply-normalizationresult), [Reply_NormalizationSource](#reply-normalizationsource). Open the list below to read their fields.

<details>
<summary>Fields of the 4 other types</summary>

### Reply_NormalizationField

A provider field name and its unchanged text.

| Field | Type | Meaning |
| --- | --- | --- |
| `field` | [Reply_SourceText](/reference/schemas/types/#reply-sourcetext) | Native field path or assembly label. |
| `value` | [Reply_SourceText](/reference/schemas/types/#reply-sourcetext) | Unchanged source text. |

### Reply_NormalizationMessage

Provider warning, error or informational message.

| Field | Type | Meaning |
| --- | --- | --- |
| `category` | [Reply_SourceText](/reference/schemas/types/#reply-sourcetext) | Native message category. |
| `code` | [Reply_SourceText](/reference/schemas/types/#reply-sourcetext) or null | Native code when supplied. Optional. |
| `text` | [Reply_SourceText](/reference/schemas/types/#reply-sourcetext) | Unchanged message body, including whitespace. |

### Reply_NormalizationResult

One provider's own descriptions, messages and version metadata.

| Field | Type | Meaning |
| --- | --- | --- |
| `corrected` | [Reply_SourceText](/reference/schemas/types/#reply-sourcetext) or null | Source corrected description, without local edits. Optional. |
| `mappings` | list of [Reply_NormalizationField](#reply-normalizationfield) | Mapping rows in native service order, labeled by their provider field. Optional. |
| `messages` | list of [Reply_NormalizationMessage](#reply-normalizationmessage) | Provider messages with original codes, text and category. Optional. |
| `normalized` | [Reply_SourceText](/reference/schemas/types/#reply-sourcetext) or null | Source normalized description, without local edits. Optional. |
| `outcome` | [Reply_VariantResolution](/reference/schemas/types/#reply-variantresolution) | Provider outcome; warning rejection remains distinct from transport failure. |
| `source` | [Reply_Source](/reference/schemas/types/#reply-source) | Responsible service. |
| `total_mappings` | integer (uint32) | Count before mapping pagination. |
| `total_messages` | integer (uint32) | Count before provider message pagination. Optional. |
| `version` | list of [Reply_NormalizationField](#reply-normalizationfield) | Version fields exactly as returned; absent when the service states none. Optional. |

### Reply_NormalizationSource

Explicit normalization service selection.

Values: `car`, `mutalyzer`, `variantvalidator`, `all`.

</details>

## Next

[`biomcp variant normalize`](/reference/commands/variant-normalize/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
