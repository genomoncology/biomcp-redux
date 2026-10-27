---
title: Reply of biomcp variant recurrence (biomcp.variant_recurrence/4)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp variant recurrence:
  a real example and the fields of schema biomcp.variant_recurrence/4."
goal: "Parse the biomcp.variant_recurrence/4 reply: see a real example and what
  each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.variant_recurrence/4
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp variant recurrence`](/reference/commands/variant-recurrence/) replies with `schema` set to `biomcp.variant_recurrence/4`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp variant recurrence 'NC_000007.13:g.140494253G>A' --source cbioportal \
  --gene BRAF --study msk_impact_2017 --profile msk_impact_2017_mutations \
  --sample-set msk_impact_2017_all --limit 3
biomcp variant recurrence 'NC_000007.13:g.140494253G>A' --source cbioportal \
  --gene BRAF --study msk_impact_2017 --profile msk_impact_2017_mutations \
  --sample-set msk_impact_2017_all --limit 3 --page 0
```

`biomcp schema biomcp.variant_recurrence/4` prints the full JSON Schema.

## Example reply

`biomcp variant recurrence 'NC_000007.13:g.140494253G>A' --source cbioportal --gene BRAF --study msk_impact_2017 --profile msk_impact_2017_mutations --sample-set msk_impact_2017_all --limit 3 --page 0` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

```json
{
  "schema": "biomcp.variant_recurrence/4",
  "status": "partial",
  "query": {
    "input": "NC_000007.13:g.140494253G>A",
    "source": "cbioportal",
    "gene": "BRAF",
    "study": "msk_impact_2017",
    "profile": "msk_impact_2017_mutations",
    "sample_set": "msk_impact_2017_all",
    "limit": 3,
    "page": 0
  },
  "result": {
    "identity": {
      "ids": ["CA168100947", "rs1008080053", "clinvar:633085"],
      "representations": […]
    },
    "matching_granularity": "exact_genomic_allele",
    "entrez_gene_id": "673",
    "study": {
      "name": "MSK-IMPACT Clinical Sequencing Cohort (MSK, Nat Med 2017)",
      "description": "Targeted sequencing of 10,000 clinical cases using the…",
      "citation": "Zehir et al. Nat Med 2017"
    },
    "observed_sample_count": 1,
    "examined_rows": 3,
    "all_pages_read": false,
    "more": true,
    "terms": {
      "unverified": true,
      "notice": "Individual studies may impose commercial restrictions; publi…",
      "url": "https://docs.cbioportal.org/user-guide/faq/"
    }
  },
  "sources": [
    {
      "source": "car",
      "use": "restricted",
      "terms": "ClinGen Allele Registry service terms; output rights require…",
      "terms_url": "https://clinicalgenome.org/about/terms-of-use/",
      "state": "ok"
    },
    … 2 more
  ],
  "problems": [
    {
      "source": "car",
      "record": "CA168100947",
      "field": "representations.expression",
      "code": "value.unknown"
    },
    … 5 more
  ],
  "next": [
    {
      "kind": "page",
      "args": [
        "variant", "recurrence", "NC_000007.13:g.140494253G>A", "--gene",
        "BRAF", "--limit", "3", "--page",
…
```

## Fields

`result` holds [VariantRecurrence](#variantrecurrence). `query` holds [VariantRecurrenceQuery](#variantrecurrencequery). `query` repeats the request as BioMCP read it.

### VariantRecurrence

Exact-allele observations confined to one explicitly bounded source page.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `all_pages_read` | boolean | yes | True only when this invocation read the whole gene query from its first page. |
| `entrez_gene_id` | [NcbiGeneId](/reference/schemas/types/#ncbigeneid) or null | no | HGNC-resolved gene query bound. |
| `examined_rows` | integer (uint32) | yes | Number of source mutation rows examined on this page. |
| `identity` | [Variant](/reference/schemas/types/#variant) or null | no | Source-qualified resolved identity; omitted when resolution failed. |
| `matching_granularity` | [RecurrenceGranularity](#recurrencegranularity) | no | Genomic allele equality only, never a protein, residue or gene match. |
| `more` | boolean | yes | Source has further rows or cannot yet establish its end. |
| `observed_sample_count` | integer or null | no | Distinct exact-matched samples on this source page only; absent on failure. |
| `selected_sample_count` | integer or null | no | Sample-list size as stated by that source; never an assayed denominator. |
| `study` | [RecurrenceStudy](#recurrencestudy) or null | no | Study title, citation and description kept as the source states them. |
| `terms` | [RecurrenceTerms](#recurrenceterms) | yes | Study-specific reuse restrictions require verification even for public data. |

### VariantRecurrenceQuery

Observe exact-allele samples within one caller-selected source page.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `gene` | [GeneSymbol](/reference/schemas/types/#genesymbol) | yes | HGNC symbol resolved to the source's Entrez gene identifier. |
| `input` | [VariantInput](/reference/schemas/types/#variantinput) | yes | Published variant identity; ambiguous identities are never selected silently. |
| `limit` | integer (uint32) | no | Source mutation rows per page, including excluded alleles. |
| `page` | integer (uint32) | no | Zero-based native source page. |
| `profile` | [Text](/reference/schemas/types/#text) | yes | Explicit mutation molecular profile. |
| `sample_set` | [Text](/reference/schemas/types/#text) | yes | Explicit sample list; selected size is never an allele denominator. |
| `source` | [RecurrenceSource](#recurrencesource) | yes | Explicit cohort source. |
| `study` | [Text](/reference/schemas/types/#text) | yes | Explicit study; there is no default cohort. |

### RecurrenceGranularity

Source fields required for a recurrence observation.

Values: `exact_genomic_allele`.

### RecurrenceSource

Source requiring explicit study, profile and sample set.

Values: `cbioportal`.

### RecurrenceStudy

Source-native descriptive metadata, without treating access groups as licenses.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `citation` | [Text](/reference/schemas/types/#text) or null | no | Source publication citation. |
| `description` | [Text](/reference/schemas/types/#text) or null | no | Complete source description, including any literal reuse notice. |
| `name` | [Text](/reference/schemas/types/#text) | yes | Source study title. |

### RecurrenceTerms

Public access grants no study-specific commercial reuse permission.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `notice` | string | yes | Source warning about possible commercial restrictions. |
| `unverified` | boolean | yes | Study-specific rights remain unverified. |
| `url` | string | yes | Publisher FAQ explaining study-specific terms. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp variant recurrence`](/reference/commands/variant-recurrence/) gives the command's arguments.
