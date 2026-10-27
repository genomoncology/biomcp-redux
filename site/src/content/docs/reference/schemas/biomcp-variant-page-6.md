---
title: Reply of biomcp search variant (biomcp.variant_page/6)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp search variant: a
  real example and the fields of schema biomcp.variant_page/6."
goal: "Parse the biomcp.variant_page/6 reply: see a real example and what each
  field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.variant_page/6
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search variant`](/reference/commands/search-variant/) replies with `schema` set to `biomcp.variant_page/6`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp search variant --gene KRAS --protein G12C
biomcp search variant --gene EGFR --protein L858R --build GRCh37
```

`biomcp schema biomcp.variant_page/6` prints the full JSON Schema.

## Example reply

`biomcp search variant --gene KRAS --protein G12C` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.variant_page/6",
  "status": "complete",
  "query": {
    "gene": "KRAS",
    "protein": "G12C",
    "limit": 10,
    "offset": 0
  },
  "result": {
    "selector_terms": {
      "usage": "noncommercial",
      "name": "dbNSFP academic branch and individual component terms",
      "url": "https://sites.google.com/site/jpopgen/dbNSFP"
    },
    "source_query": "dbnsfp.genename:KRAS AND dbnsfp.hgvsp:\"p.G12C\"",
    "items": [
      {…}
    ]
  },
  "page": {"offset": 0, "returned": 1, "total": 1},
  "sources": [
    {
      "source": "myvariant",
      "use": "restricted",
      "terms": "MyVariant.info terms and component data licenses",
      "terms_url": "https://myvariant.info/terms",
      "state": "ok"
    }
  ],
  "next": [
    {
      "kind": "disambiguate",
      "args": ["get", "variant", "chr12:g.25398285C>A", "--build", "GRCh37"]
    }
  ]
}
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [VariantPage](#variantpage). `query` holds [VariantSearch](#variantsearch). `query` repeats the request as BioMCP read it.

### VariantPage

One native MyVariant candidate page; its total and offset are in the shared envelope.

| Field | Type | Meaning |
| --- | --- | --- |
| `build` | [Build](/reference/schemas/types/#build) or null | Assembly sent to the source, never inferred from a candidate label. Optional. |
| `filter_terms` | [ComponentTerms](/reference/schemas/types/#componentterms) or null | Independent publisher terms for named ClinVar filters, present only when requested. Optional. |
| `items` | list of [VariantHeader](/reference/schemas/types/#variantheader) | Native source order, preserving every candidate including repeated source identities. |
| `selector_terms` | [ComponentTerms](/reference/schemas/types/#componentterms) or null | Independent publisher terms for the declared dbNSFP gene/protein selector. Optional. |
| `source_query` | string | Exact query sent to the named source, without rewriting raw query text. |

### VariantSearch

Discover source candidates using one declared selector, without interpreting free text.

| Field | Type | Meaning |
| --- | --- | --- |
| `build` | [Build](/reference/schemas/types/#build) | Optional caller assembly sent to MyVariant; regions require it explicitly. Optional. |
| `condition` | [Text](/reference/schemas/types/#text) | Literal MyVariant clinvar.rcv.conditions.name phrase; source text is not a local diagnosis. Requires a declared protein or region selector. Optional. |
| `gene` | [GeneSymbol](/reference/schemas/types/#genesymbol) | Exact checked gene symbol for the fixed dbNSFP gene/protein field query; obeys component terms switches. Optional. |
| `has` | [VariantSearchField](#variantsearchfield) | Require the selected component's identifier field. Only source-proven mappings are offered. Optional. |
| `limit` | integer (uint32) | Native candidates per page, default 10, maximum 50; all candidates remain candidates. Optional. |
| `offset` | integer (uint32) | Native offset within this command's bounded result window; larger native totals remain visible. Optional. |
| `protein` | [ProteinQuery](#proteinquery) | Explicit substitution using checked one-letter or three-letter amino-acid codes. Optional. |
| `query` | [MyVariantQuery](#myvariantquery) | Raw MyVariant query, passed and echoed unchanged when component policy allows; requires --source myvariant. With disabled components use declared selectors instead. Optional. |
| `region` | [VariantRegion](#variantregion) | Exact one-based inclusive MyVariant region, such as chr7:55259500-55259600; requires --build. Optional. |
| `review_status` | [Text](/reference/schemas/types/#text) | Literal MyVariant clinvar.rcv.review_status phrase; no star conversion. Requires a declared protein or region selector. Optional. |
| `source` | [VariantSearchSource](#variantsearchsource) | Explicit source language; required for a raw query. Optional. |

### Other types in this reply

5 more types appear only in this reply: [MyVariantQuery](#myvariantquery), [ProteinQuery](#proteinquery), [VariantRegion](#variantregion), [VariantSearchField](#variantsearchfield), [VariantSearchSource](#variantsearchsource). Open the list below to read their fields.

<details>
<summary>Fields of the 5 other types</summary>

### MyVariantQuery

Type: string.

### ProteinQuery

Type: string.

### VariantRegion

Type: string.

### VariantSearchField

Source-proven component fields supported by the presence filter.

Values: `clinvar`.

### VariantSearchSource

The source-native query language exposed by variant search.

Values: `myvariant`.

</details>

## Next

[`biomcp search variant`](/reference/commands/search-variant/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
