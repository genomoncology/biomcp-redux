---
title: Reply of biomcp search variant (biomcp.variant-page/16)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp search variant: the
  fields of schema biomcp.variant-page/16."
goal: "Parse the biomcp.variant-page/16 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.variant-page/16
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search variant`](/reference/commands/search-variant/) and [`biomcp search`](/reference/commands/search/) reply with `schema` set to `biomcp.variant-page/16`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.variant-page/16` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_VariantPage](#reply-variantpage). `query` holds [Reply_VariantSearch](#reply-variantsearch). `query` repeats the request as BioMCP read it.

### Reply_VariantPage

One native variant candidate page; its total and offset are in the shared envelope.

| Field | Type | Meaning |
| --- | --- | --- |
| `build` | [Reply_Build](/reference/schemas/types/#reply-build) or null | Assembly sent to the source, never inferred from a candidate label. Optional. |
| `excluded` | [Reply_VariantSearchExclusions](#reply-variantsearchexclusions) or null | Fallback candidates excluded because their reference allele was not confirmed. Optional. |
| `filter_terms` | [Reply_ComponentTerms](/reference/schemas/types/#reply-componentterms) or null | Independent publisher terms for named ClinVar filters, present only when requested. Optional. |
| `items` | list of [Reply_VariantHeader](/reference/schemas/types/#reply-variantheader) | Native source order, preserving every candidate including repeated source identities. Optional. |
| `selector_terms` | [Reply_ComponentTerms](/reference/schemas/types/#reply-componentterms) or null | Independent publisher terms for the declared dbNSFP gene/protein selector. Optional. |
| `source_query` | string | Exact query sent to the named source, without rewriting raw query text. |

### Reply_VariantSearch

Discover source candidates using one declared selector, without interpreting free text.

| Field | Type | Meaning |
| --- | --- | --- |
| `build` | [Reply_Build](/reference/schemas/types/#reply-build) | Optional caller assembly sent to MyVariant; regions require it explicitly. Optional. |
| `coding` | [Reply_CodingChange](#reply-codingchange) | Coding HGVS change searched literally in ClinVar with --gene, without choosing a transcript. Optional. |
| `condition` | [Reply_Text](/reference/schemas/types/#reply-text) | Literal MyVariant clinvar.rcv.conditions.name phrase; source text is not a local diagnosis. Requires a declared protein or region selector. Optional. |
| `gene` | [Reply_GeneSymbol](/reference/schemas/types/#reply-genesymbol) | Checked gene symbol for the dbNSFP substitution or ClinVar coding/synonymous selector. Optional. |
| `has` | [Reply_VariantSearchField](#reply-variantsearchfield) | Require the selected component's identifier field. Only source-proven mappings are offered. Optional. |
| `limit` | integer (uint32) | Native candidates per page, default 10, maximum 50; all candidates remain candidates. Optional. |
| `offset` | integer (uint32) | Native offset within this command's bounded result window; larger native totals remain visible. Optional. |
| `protein` | [Reply_ProteinQuery](/reference/schemas/types/#reply-proteinquery) | Explicit substitution or synonymous change using checked amino-acid codes. Optional. |
| `query` | [Reply_MyVariantQuery](#reply-myvariantquery) | Raw MyVariant query, passed and echoed unchanged when component policy allows; requires --source myvariant. With disabled components use declared selectors instead. Optional. |
| `region` | [Reply_VariantRegion](#reply-variantregion) | Exact one-based inclusive MyVariant region, such as chr7:55259500-55259600; requires --build. Optional. |
| `review_status` | [Reply_Text](/reference/schemas/types/#reply-text) | Literal MyVariant clinvar.rcv.review_status phrase; no star conversion. Requires a declared protein or region selector. Optional. |
| `source` | [Reply_VariantSearchSource](#reply-variantsearchsource) | Explicit source language; required for a raw query. Optional. |

### Other types in this reply

7 more types appear only in this reply: [Reply_CodingChange](#reply-codingchange), [Reply_MyVariantQuery](#reply-myvariantquery), [Reply_VariantRegion](#reply-variantregion), [Reply_VariantSearchExclusionReason](#reply-variantsearchexclusionreason), [Reply_VariantSearchExclusions](#reply-variantsearchexclusions), [Reply_VariantSearchField](#reply-variantsearchfield), [Reply_VariantSearchSource](#reply-variantsearchsource). Open the list below to read their fields.

<details>
<summary>Fields of the 7 other types</summary>

### Reply_CodingChange

HGVS 21.1 coding edit syntax without a reference. The parser additionally requires increasing range endpoints, different substitution bases and insertion adjacency where literal positions establish it. It does not check transcript sequence or exon boundaries.

Type: string.

### Reply_MyVariantQuery

Explicit MyVariant query text. The schema limits Unicode character count; the parser additionally limits UTF-8 byte count to the same maximum.

Type: string.

### Reply_VariantRegion

Primary-chromosome region. The parser additionally requires the end to be at least the start.

Type: string.

### Reply_VariantSearchExclusionReason

Fixed fallback exclusion reasons.

Values: `reference-unconfirmed`.

### Reply_VariantSearchExclusions

Count of candidates withheld from a fallback search, with a stable reason.

| Field | Type | Meaning |
| --- | --- | --- |
| `count` | integer (uint32) | Native candidate count, independent of pagination's consumed positions. |
| `reason` | [Reply_VariantSearchExclusionReason](#reply-variantsearchexclusionreason) | Why these candidates could not be admitted. |

### Reply_VariantSearchField

Source-proven component fields supported by the presence filter.

Values: `clinvar`.

### Reply_VariantSearchSource

The source-native query language exposed by variant search.

Values: `myvariant`.

</details>

## Next

[`biomcp search variant`](/reference/commands/search-variant/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
