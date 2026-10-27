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

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search variant`](/reference/commands/search-variant/) replies with `schema` set to `biomcp.variant_page/6`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp search variant --gene KRAS --protein G12C
biomcp search variant --gene EGFR --protein L858R --build GRCh37
```

`biomcp schema biomcp.variant_page/6` prints the full JSON Schema.

## Example reply

`biomcp search variant --gene KRAS --protein G12C` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

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
      {
        "identity": {
          "ids": ["rs121913530", "clinvar:12578"],
          "representations": [
            {…}
          ],
          "genes": [{"label": "KRAS"}]
        },
        "source": "myvariant",
        "record": {"id": "chr12:g.25398285C>A"},
        "component_terms": [
          {
            "component": "clinvar",
            "terms": {…}
          }
        ]
      }
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
      "args": [
        "get", "variant", "chr12:g.25398285C>A", "--build", "GRCh37"
      ]
    }
  ]
}
```

## Fields

`result` holds [VariantPage](#variantpage). `query` holds [VariantSearch](#variantsearch). `query` repeats the request as BioMCP read it.

### VariantPage

One native MyVariant candidate page; its total and offset are in the shared envelope.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `build` | [Build](/reference/schemas/types/#build) or null | no | Assembly sent to the source, never inferred from a candidate label. |
| `filter_terms` | [ComponentTerms](/reference/schemas/types/#componentterms) or null | no | Independent publisher terms for named ClinVar filters, present only when requested. |
| `items` | list of [VariantHeader](/reference/schemas/types/#variantheader) | yes | Native source order, preserving every candidate including repeated source identities. |
| `selector_terms` | [ComponentTerms](/reference/schemas/types/#componentterms) or null | no | Independent publisher terms for the declared dbNSFP gene/protein selector. |
| `source_query` | string | yes | Exact query sent to the named source, without rewriting raw query text. |

### VariantSearch

Discover source candidates using one declared selector, without interpreting free text.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `build` | [Build](/reference/schemas/types/#build) | no | Optional caller assembly sent to MyVariant; regions require it explicitly. |
| `condition` | [Text](/reference/schemas/types/#text) | no | Literal MyVariant clinvar.rcv.conditions.name phrase; source text is not a local diagnosis. Requires a declared protein or region selector. |
| `gene` | [GeneSymbol](/reference/schemas/types/#genesymbol) | no | Exact checked gene symbol for the fixed dbNSFP gene/protein field query; obeys component terms switches. |
| `has` | [VariantSearchField](#variantsearchfield) | no | Require the selected component's identifier field. Only source-proven mappings are offered. |
| `limit` | integer (uint32) | no | Native candidates per page, default 10, maximum 50; all candidates remain candidates. |
| `offset` | integer (uint32) | no | Native offset within this command's bounded result window; larger native totals remain visible. |
| `protein` | [ProteinQuery](#proteinquery) | no | Explicit substitution using checked one-letter or three-letter amino-acid codes. |
| `query` | [MyVariantQuery](#myvariantquery) | no | Raw MyVariant query, passed and echoed unchanged when component policy allows; requires --source myvariant. With disabled components use declared selectors instead. |
| `region` | [VariantRegion](#variantregion) | no | Exact one-based inclusive MyVariant region, such as chr7:55259500-55259600; requires --build. |
| `review_status` | [Text](/reference/schemas/types/#text) | no | Literal MyVariant clinvar.rcv.review_status phrase; no star conversion. Requires a declared protein or region selector. |
| `source` | [VariantSearchSource](#variantsearchsource) | no | Explicit source language; required for a raw query. |

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

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp search variant`](/reference/commands/search-variant/) gives the command's arguments.
