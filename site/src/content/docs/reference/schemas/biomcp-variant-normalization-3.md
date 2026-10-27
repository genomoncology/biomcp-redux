---
title: Reply of biomcp variant normalize (biomcp.variant_normalization/3)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp variant normalize: a
  real example and the fields of schema biomcp.variant_normalization/3."
goal: "Parse the biomcp.variant_normalization/3 reply: see a real example and
  what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.variant_normalization/3
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp variant normalize`](/reference/commands/variant-normalize/) replies with `schema` set to `biomcp.variant_normalization/3`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp variant normalize --source mutalyzer --hgvs 'NM_004333.4:c.1799T>A'
biomcp variant normalize --source variantvalidator \
  --hgvs 'NM_004333.4:c.1799T>A' --build GRCh38
```

`biomcp schema biomcp.variant_normalization/3` prints the full JSON Schema.

## Example reply

`biomcp variant normalize --source mutalyzer --hgvs 'NM_004333.4:c.1799T>A'` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.variant_normalization/3",
  "status": "complete",
  "query": {
    "source": "mutalyzer",
    "hgvs": "NM_004333.4:c.1799T>A",
    "limit": 10,
    "offset": 0
  },
  "result": {
    "results": [
      {
        "source": "mutalyzer",
        "outcome": "resolved",
        "normalized": "NM_004333.4:c.1799T>A",
        "corrected": "NM_004333.4:c.1799T>A",
        "mappings": [
          {…},
          … 1 more
        ],
        "total_mappings": 2,
        "messages": [
          {…},
          … 1 more
        ],
        "total_messages": 2,
        "version": []
      }
    ]
  },
  "sources": [
    {
      "source": "mutalyzer",
      "use": "restricted",
      "terms": "Mutalyzer service output terms require review",
      "terms_url": "https://mutalyzer.nl/",
      "state": "ok"
    }
  ]
}
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [VariantNormalization](#variantnormalization). `query` holds [VariantNormalize](#variantnormalize). `query` repeats the request as BioMCP read it.

### VariantNormalization

Independent service results without local correction or transcript selection.

| Field | Type | Meaning |
| --- | --- | --- |
| `results` | list of [NormalizationResult](#normalizationresult) | Service results in declared order. |

### VariantNormalize

Send unchanged reference-qualified notation to explicitly named services.

| Field | Type | Meaning |
| --- | --- | --- |
| `build` | [Build](/reference/schemas/types/#build) or null | Caller assembly; required when VariantValidator is selected. Optional. |
| `hgvs` | [HgvsText](/reference/schemas/types/#hgvstext) | Unchanged reference-qualified HGVS; no local molecular parse. |
| `limit` | integer (uint32) | Source mapping rows per service, with typed continuation for remaining rows. Optional. |
| `offset` | integer (uint32) | Source mapping rows to skip per service. Optional. |
| `source` | [NormalizationSource](#normalizationsource) | Explicit service; all preserves independent results without consensus. |

### NormalizationField

A provider field name and its unchanged text.

| Field | Type | Meaning |
| --- | --- | --- |
| `field` | string | Native field path or assembly label. |
| `value` | string | Unchanged source text. |

### NormalizationMessage

Provider warning, error or informational message.

| Field | Type | Meaning |
| --- | --- | --- |
| `category` | string | Native message category. |
| `code` | string or null | Native code when supplied. Optional. |
| `text` | string | Unchanged message body, including whitespace. |

### NormalizationResult

One provider's own descriptions, messages and version metadata.

| Field | Type | Meaning |
| --- | --- | --- |
| `corrected` | string or null | Source corrected description, without local edits. Optional. |
| `mappings` | list of [NormalizationField](#normalizationfield) | Mapping rows in native service order, labeled by their provider field. |
| `messages` | list of [NormalizationMessage](#normalizationmessage) | Provider messages with original codes, text and category. |
| `normalized` | string or null | Source normalized description, without local edits. Optional. |
| `outcome` | [VariantResolution](/reference/schemas/types/#variantresolution) | Provider outcome; warning rejection remains distinct from transport failure. |
| `source` | [Source](/reference/schemas/types/#source) | Responsible service. |
| `total_mappings` | integer (uint32) | Count before mapping pagination. |
| `total_messages` | integer (uint32) | Count before provider message pagination. Optional. |
| `version` | list of [NormalizationField](#normalizationfield) | Version fields exactly as returned; absent when the service states none. |

### NormalizationSource

Explicit normalization service selection.

Values: `car`, `mutalyzer`, `variantvalidator`, `all`.

## Next

[`biomcp variant normalize`](/reference/commands/variant-normalize/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
