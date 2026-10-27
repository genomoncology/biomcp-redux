---
title: Reply of biomcp get variant (biomcp.variant_card/5)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get variant: a real
  example and the fields of schema biomcp.variant_card/5."
goal: "Parse the biomcp.variant_card/5 reply: see a real example and what each
  field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.variant_card/5
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get variant`](/reference/commands/get-variant/) and [`biomcp get variant civic`](/reference/commands/get-variant/) reply with `schema` set to `biomcp.variant_card/5`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp get variant rs113488022
biomcp get variant CA123643 civic
```

`biomcp schema biomcp.variant_card/5` prints the full JSON Schema.

## Example reply

`biomcp get variant CA123643 civic` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

```json
{
  "schema": "biomcp.variant_card/5",
  "status": "complete",
  "query": {
    "input": "CA123643",
    "section": ["civic"],
    "section_offset": 0,
    "resolver": "car",
    "limit": 10,
    "offset": 0
  },
  "result": {
    "resolution": "unresolved",
    "source": "civic",
    "candidates": [
      {
        "identity": {},
        "requested": "CA123643",
        "source": "civic",
        "record": {"id": "CA123643"},
        "sections": {
          "civic": {
            "state": "data",
            "source": "civic",
            "data": {…}
          }
        }
      }
    ]
  },
  "sources": [
    {
      "source": "civic",
      "use": "open",
      "terms": "CC0 1.0",
      "terms_url": "https://civicdb.org/releases/licensing",
      "state": "ok"
    }
  ],
  "next": [
    {
      "kind": "page",
      "args": [
        "get", "variant", "CA123643", "civic", "--section-offset", "1"
      ]
    },
    … 2 more
  ]
}
```

## Fields

`result` holds [VariantCard](#variantcard). `query` holds [VariantGet](/reference/schemas/types/#variantget), a shared type. `query` repeats the request as BioMCP read it.

### VariantCard

A compact identity card, including every provider candidate and explicit rejection.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `candidates` | list of [VariantHeader](/reference/schemas/types/#variantheader) | yes | All retained source identities. |
| `rejection` | [Text](/reference/schemas/types/#text) or null | no | Provider rejection text, when present, separate from unchanged query input. |
| `resolution` | [VariantResolution](/reference/schemas/types/#variantresolution) | yes | Identity outcome, without selecting the first candidate. |
| `source` | [Source](/reference/schemas/types/#source) | yes | Responsible identity source. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp get variant`](/reference/commands/get-variant/) gives the command's arguments.
