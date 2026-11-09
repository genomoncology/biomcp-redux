---
title: Reply of biomcp variant population (biomcp.variant-population/13)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp variant population:
  the fields of schema biomcp.variant-population/13."
goal: "Parse the biomcp.variant-population/13 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.variant-population/13
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp variant population`](/reference/commands/variant-population/) and [`biomcp variant`](/reference/commands/variant/) reply with `schema` set to `biomcp.variant-population/13`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.variant-population/13` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_VariantPopulationResult](#reply_variantpopulationresult). `query` holds [Reply_VariantPopulationQueryEcho](#reply_variantpopulationqueryecho). `query` repeats the request as BioMCP read it.

### Reply_VariantPopulationResult

Population summary and one explicitly requested ancestry-detail page.

| Field | Type | Meaning |
| --- | --- | --- |
| `candidates` | list of [Reply_VariantHeader](/reference/schemas/types/#reply_variantheader) | Source allele candidates when this input does not identify one joinable allele. Optional. |
| `groups` | list of [Reply_GnomadCounts](/reference/schemas/types/#reply_gnomadcounts) | Selected native groups, qualified by sequencing subset. Optional. |
| `population` | [Reply_Section_VariantPopulation](/reference/schemas/types/#reply_section_variantpopulation) | Source population state; missing datasets stay absent within its summary. |

### Reply_VariantPopulationQueryEcho

Reply selectors with the resolver that actually answered, omitted when no named resolver answered.

| Field | Type | Meaning |
| --- | --- | --- |
| `build` | [Reply_Build](/reference/schemas/types/#reply_build) or null | Explicit source assembly when required by identity input. Optional. |
| `detail` | boolean | Read source ancestry groups, preserving overlapping groups individually. Optional. |
| `input` | [Reply_VariantLookupInput](/reference/schemas/types/#reply_variantlookupinput) | Exact published allele identity; ambiguous candidates must first be selected. |
| `limit` | integer (uint32) | Maximum rows per page. Optional. |
| `offset` | integer (uint32) | Native rows to skip. Optional. |
| `resolver` | [Reply_VariantResolver](/reference/schemas/types/#reply_variantresolver) | Resolver for reference-qualified input. Optional. |

## Next

[`biomcp variant population`](/reference/commands/variant-population/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
