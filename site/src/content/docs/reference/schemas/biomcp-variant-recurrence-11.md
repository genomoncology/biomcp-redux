---
title: Reply of biomcp variant recurrence (biomcp.variant-recurrence/11)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp variant recurrence:
  the fields of schema biomcp.variant-recurrence/11."
goal: "Parse the biomcp.variant-recurrence/11 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.variant-recurrence/11
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp variant recurrence`](/reference/commands/variant-recurrence/) and [`biomcp variant`](/reference/commands/variant/) reply with `schema` set to `biomcp.variant-recurrence/11`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.variant-recurrence/11` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_VariantRecurrence](#reply_variantrecurrence). `query` holds [Reply_VariantRecurrenceQuery](#reply_variantrecurrencequery). `query` repeats the request as BioMCP read it.

### Reply_VariantRecurrence

Exact-allele observations confined to one explicitly bounded source page.

| Field | Type | Meaning |
| --- | --- | --- |
| `all_pages_read` | boolean | True only when this invocation read the whole gene query from its first page. |
| `entrez_gene_id` | [Reply_NcbiGeneId](/reference/schemas/types/#reply_ncbigeneid) or null | HGNC-resolved gene query bound. Optional. |
| `examined_rows` | integer (uint32) | Number of source mutation rows examined on this page. |
| `identity` | [Reply_Variant](/reference/schemas/types/#reply_variant) or null | Source-qualified resolved identity; omitted when resolution failed. Optional. |
| `matching_granularity` | [Reply_RecurrenceGranularity](#reply_recurrencegranularity) | Genomic allele equality only, never a protein, residue or gene match. Optional. |
| `more` | boolean | Source has further rows or cannot yet establish its end. |
| `observed_sample_count` | integer or null | Distinct exact-matched samples on this source page only; absent on failure. Optional. |
| `selected_sample_count` | integer or null | Sample-list size as stated by that source; never an assayed denominator. Optional. |
| `study` | [Reply_RecurrenceStudy](#reply_recurrencestudy) or null | Study title, citation and description kept as the source states them. Optional. |
| `terms` | [Reply_RecurrenceTerms](#reply_recurrenceterms) | Study-specific reuse restrictions require verification even for public data. |

### Reply_VariantRecurrenceQuery

Observe exact-allele samples within one caller-selected source page.

| Field | Type | Meaning |
| --- | --- | --- |
| `build` | [Reply_Build](/reference/schemas/types/#reply_build) or null | Identity lookup assembly; omitted uses the shared GRCh38 default. Optional. |
| `gene` | [Reply_GeneSymbol](/reference/schemas/types/#reply_genesymbol) | HGNC symbol resolved to the source's Entrez gene identifier. |
| `input` | [Reply_RecurrenceInput](#reply_recurrenceinput) | Variant identity resolving to one unique source-qualified genomic replacement with nonempty reference and alternate alleles (pure insertions and deletions are unavailable). |
| `limit` | integer (uint32) | Source mutation rows per page, including excluded alleles. Optional. |
| `page` | integer (uint32) | Zero-based native source page. Optional. |
| `profile` | [Reply_Text](/reference/schemas/types/#reply_text) | Explicit mutation molecular profile. |
| `sample_set` | [Reply_Text](/reference/schemas/types/#reply_text) | Explicit sample list; selected size is never an allele denominator. |
| `source` | [Reply_RecurrenceSource](#reply_recurrencesource) | Explicit cohort source. |
| `study` | [Reply_Text](/reference/schemas/types/#reply_text) | Explicit study; there is no default cohort. |

### Other types in this reply

5 more types appear only in this reply: [Reply_RecurrenceGranularity](#reply_recurrencegranularity), [Reply_RecurrenceInput](#reply_recurrenceinput), [Reply_RecurrenceSource](#reply_recurrencesource), [Reply_RecurrenceStudy](#reply_recurrencestudy), [Reply_RecurrenceTerms](#reply_recurrenceterms). Open the list below to read their fields.

<details>
<summary>Fields of the 5 other types</summary>

### Reply_RecurrenceGranularity

Source fields required for a recurrence observation.

Values: `exact-genomic-allele`.

### Reply_RecurrenceInput

Accepted forms: rsID, CAid, clinvar:N, VCV accession, reference-qualified HGVS, chromosome genomic HGVS, or VCF-style chromosome-position-reference-alternate coordinates, defaulting to GRCh38. Reference bases are validated by the source. Requires one unique source-qualified genomic replacement with nonempty reference and alternate alleles (pure insertions and deletions are unavailable).

Type: string.

### Reply_RecurrenceSource

Source requiring explicit study, profile and sample set.

Values: `cbioportal`.

### Reply_RecurrenceStudy

Source-native descriptive metadata, without treating access groups as licenses.

| Field | Type | Meaning |
| --- | --- | --- |
| `citation` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Source publication citation. Optional. |
| `description` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Complete source description, including any literal reuse notice. Optional. |
| `name` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Source study title. Optional. |

### Reply_RecurrenceTerms

Public access grants no study-specific commercial reuse permission.

| Field | Type | Meaning |
| --- | --- | --- |
| `notice` | string | Source warning about possible commercial restrictions. |
| `unverified` | boolean | Whether the source terms have not been reviewed. |
| `url` | string (uri) | Publisher FAQ explaining study-specific terms. |

</details>

## Next

[`biomcp variant recurrence`](/reference/commands/variant-recurrence/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
