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

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp variant normalize`](/reference/commands/variant-normalize/) replies with `schema` set to `biomcp.variant_normalization/3`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp variant normalize --source mutalyzer --hgvs 'NM_004333.4:c.1799T>A'
biomcp variant normalize --source variantvalidator \
  --hgvs 'NM_004333.4:c.1799T>A' --build GRCh38
```

`biomcp schema biomcp.variant_normalization/3` prints the full JSON Schema.

## Example reply

`biomcp variant normalize --source mutalyzer --hgvs 'NM_004333.4:c.1799T>A'` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

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
          {
            "field": "protein.description",
            "value": "NM_004333.4(NP_004324.2):p.(Val600Glu)"
          },
          {
            "field": "rna.description",
            "value": "NM_004333.4:r.(1799u>a)"
          }
        ],
        "total_mappings": 2,
        "messages": [
          {
            "category": "infos",
            "code": "IWHOLETRANSCRIPTEXON",
            "text": "No exon features found in the 'NM_004333.4' reference seq…"
          },
          {
            "category": "infos",
            "code": "IMRNAGENOMICTIP",
            "text": "An 'mRNA' sequence was used with the 'c.' coordinate syst…"
          }
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

`result` holds [VariantNormalization](#variantnormalization). `query` holds [VariantNormalize](#variantnormalize). `query` repeats the request as BioMCP read it.

### VariantNormalization

Independent service results without local correction or transcript selection.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `results` | list of [NormalizationResult](#normalizationresult) | yes | Service results in declared order. |

### VariantNormalize

Send unchanged reference-qualified notation to explicitly named services.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `build` | [Build](/reference/schemas/types/#build) or null | no | Caller assembly; required when VariantValidator is selected. |
| `hgvs` | [HgvsText](/reference/schemas/types/#hgvstext) | yes | Unchanged reference-qualified HGVS; no local molecular parse. |
| `limit` | integer (uint32) | no | Source mapping rows per service, with typed continuation for remaining rows. |
| `offset` | integer (uint32) | no | Source mapping rows to skip per service. |
| `source` | [NormalizationSource](#normalizationsource) | yes | Explicit service; all preserves independent results without consensus. |

### NormalizationField

A provider field name and its unchanged text.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `field` | string | yes | Native field path or assembly label. |
| `value` | string | yes | Unchanged source text. |

### NormalizationMessage

Provider warning, error or informational message.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `category` | string | yes | Native message category. |
| `code` | string or null | no | Native code when supplied. |
| `text` | string | yes | Unchanged message body, including whitespace. |

### NormalizationResult

One provider's own descriptions, messages and version metadata.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `corrected` | string or null | no | Source corrected description, without local edits. |
| `mappings` | list of [NormalizationField](#normalizationfield) | yes | Mapping rows in native service order, labeled by their provider field. |
| `messages` | list of [NormalizationMessage](#normalizationmessage) | yes | Provider messages with original codes, text and category. |
| `normalized` | string or null | no | Source normalized description, without local edits. |
| `outcome` | [VariantResolution](/reference/schemas/types/#variantresolution) | yes | Provider outcome; warning rejection remains distinct from transport failure. |
| `source` | [Source](/reference/schemas/types/#source) | yes | Responsible service. |
| `total_mappings` | integer (uint32) | yes | Count before mapping pagination. |
| `total_messages` | integer (uint32) | no | Count before provider message pagination. |
| `version` | list of [NormalizationField](#normalizationfield) | yes | Version fields exactly as returned; absent when the service states none. |

### NormalizationSource

Explicit normalization service selection.

Values: `car`, `mutalyzer`, `variantvalidator`, `all`.

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp variant normalize`](/reference/commands/variant-normalize/) gives the command's arguments.
