---
title: Reply of biomcp variant assertions (biomcp.variant_assertions/3)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp variant assertions:
  a real example and the fields of schema biomcp.variant_assertions/3."
goal: "Parse the biomcp.variant_assertions/3 reply: see a real example and what
  each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.variant_assertions/3
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp variant assertions`](/reference/commands/variant-assertions/) replies with `schema` set to `biomcp.variant_assertions/3`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp variant assertions clinvar:13961 --source clinvar --limit 50
biomcp variant assertions CA000434 --source erepo
```

`biomcp schema biomcp.variant_assertions/3` prints the full JSON Schema.

## Example reply

`biomcp variant assertions clinvar:13961 --source clinvar --limit 50` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.variant_assertions/3",
  "status": "complete",
  "query": {
    "input": "clinvar:13961",
    "source": "clinvar",
    "limit": 50,
    "offset": 0
  },
  "result": {
    "assertions": [
      {…},
      … 49 more
    ],
    "page": {"offset": 0, "returned": 50, "total": 88},
    "next": {
      "input": "clinvar:13961",
      "source": "clinvar",
      "limit": 50,
      "offset": 50
    }
  },
  "sources": [
    {
      "source": "clinvar",
      "use": "restricted",
      "terms": "NCBI policy; submission rights remain source-specific",
      "terms_url": "https://www.ncbi.nlm.nih.gov/clinvar/docs/maintenance_use/",
      "state": "ok"
    }
  ],
  "next": [
    {
      "kind": "page",
      "args": [
        "variant", "assertions", "clinvar:13961", "--limit", "50", "--offset",
        "50", "--source",
        … 1 more
      ]
    }
…
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [VariantAssertions](/reference/schemas/types/#variantassertions), a shared type. `query` holds [VariantAssertionsGet](/reference/schemas/types/#variantassertionsget), a shared type. `query` repeats the request as BioMCP read it.

Every type this reply uses is shared with other replies, so its fields are on the shared types page.

## Next

[`biomcp variant assertions`](/reference/commands/variant-assertions/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
