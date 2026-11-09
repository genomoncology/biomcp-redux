---
title: Reply of biomcp get pgx (biomcp.pgx-annotation/5)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get pgx: the fields
  of schema biomcp.pgx-annotation/5."
goal: "Parse the biomcp.pgx-annotation/5 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.pgx-annotation/5
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get pgx`](/reference/commands/get-pgx/) and [`biomcp get`](/reference/commands/get/) reply with `schema` set to `biomcp.pgx-annotation/5`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.pgx-annotation/5` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_PgxAnnotation](#reply-pgxannotation). `query` holds [Reply_PgxAnnotationGet](#reply-pgxannotationget). `query` repeats the request as BioMCP read it.

### Reply_PgxAnnotation

Complete proved base-tier annotation.

| Field | Type | Meaning |
| --- | --- | --- |
| `annotation` | [Reply_PgxAnnotationDetail](#reply-pgxannotationdetail) | Complete native base record, including history and genotype text. |
| `header` | [Reply_PgxAnnotationIdentity](#reply-pgxannotationidentity) | Exact selected source identity. |

### Reply_PgxAnnotationGet

Exact single clinical annotation get, with no inferred source or annotation kind.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [Reply_ClinPgxAccession](/reference/schemas/types/#reply-clinpgxaccession) | Checked ClinPGx accession, never a shared PaId. |
| `source` | [Reply_PgxSource](/reference/schemas/types/#reply-pgxsource) | Required ClinPGx source. |

### Other types in this reply

4 more types appear only in this reply: [Reply_PgxAllelePhenotype](#reply-pgxallelephenotype), [Reply_PgxAnnotationDetail](#reply-pgxannotationdetail), [Reply_PgxAnnotationHistory](#reply-pgxannotationhistory), [Reply_PgxAnnotationIdentity](#reply-pgxannotationidentity). Open the list below to read their fields.

<details>
<summary>Fields of the 4 other types</summary>

### Reply_PgxAllelePhenotype

Source-native PgxAllelePhenotype fields; no clinical interpretation.

| Field | Type | Meaning |
| --- | --- | --- |
| `allele` | string or null | The source's own `allele` value, unchanged. Optional. |
| `id` | [Reply_Identifier_int64](/reference/schemas/types/#reply-identifier-int64) | The source's own `id` value, unchanged. |
| `limitedEvidence` | boolean or null | The source's own `limitedEvidence` value, unchanged. Optional. |
| `phenotype` | string or null | The source's own `phenotype` value, unchanged. Optional. |

### Reply_PgxAnnotationDetail

Source-native PgxAnnotationDetail fields; no clinical interpretation.

| Field | Type | Meaning |
| --- | --- | --- |
| `accessionId` | [Reply_ClinPgxAccession](/reference/schemas/types/#reply-clinpgxaccession) | The source's own `accessionId` value, unchanged. |
| `allelePhenotypes` | list of [Reply_PgxAllelePhenotype](#reply-pgxallelephenotype) | The source's own `allelePhenotypes` value, unchanged. Optional. |
| `conflictingVariantAnnotationIds` | list of [Reply_Identifier_int64](/reference/schemas/types/#reply-identifier-int64) | The source's own `conflictingVariantAnnotationIds` value, unchanged. Optional. |
| `history` | list of [Reply_PgxAnnotationHistory](#reply-pgxannotationhistory) | The source's own `history` value, unchanged. Optional. |
| `id` | [Reply_Identifier_int64](/reference/schemas/types/#reply-identifier-int64) | The source's own `id` value, unchanged. |
| `levelOfEvidence` | [Reply_PgxEvidenceTerm](/reference/schemas/types/#reply-pgxevidenceterm) or null | The source's own `levelOfEvidence` value, unchanged. Optional. |
| `location` | [Reply_PgxLocation](/reference/schemas/types/#reply-pgxlocation) or null | The source's own `location` value, unchanged. Optional. |
| `name` | string or null | The source's own `name` value, unchanged. Optional. |
| `objCls` | string or null | The source's own `objCls` value, unchanged. Optional. |
| `overrideLevel` | boolean or null | The source's own `overrideLevel` value, unchanged. Optional. |
| `overrideLevelDescription` | string or null | Native `overrideLevelDescription` when supplied. Optional. |
| `pediatric` | boolean or null | The source's own `pediatric` value, unchanged. Optional. |
| `relatedAlleles` | list of [Reply_PgxObjectReference](/reference/schemas/types/#reply-pgxobjectreference) | The source's own `relatedAlleles` value, unchanged. Optional. |
| `relatedChemicals` | list of [Reply_PgxObjectReference](/reference/schemas/types/#reply-pgxobjectreference) | The source's own `relatedChemicals` value, unchanged. Optional. |
| `relatedChemicalsLogic` | string or null | The source's own `relatedChemicalsLogic` value, unchanged. Optional. |
| `relatedDiseases` | list of [Reply_PgxObjectReference](/reference/schemas/types/#reply-pgxobjectreference) | The source's own `relatedDiseases` value, unchanged. Optional. |
| `relatedGuidelines` | list of [Reply_PgxObjectReference](/reference/schemas/types/#reply-pgxobjectreference) | The source's own `relatedGuidelines` value, unchanged. Optional. |
| `relatedLabels` | list of [Reply_PgxObjectReference](/reference/schemas/types/#reply-pgxobjectreference) | The source's own `relatedLabels` value, unchanged. Optional. |
| `score` | [Reply_FiniteNumber](/reference/schemas/types/#reply-finitenumber) or null | The source's own `score` value, unchanged. Optional. |
| `scoreDetails` | string or null | The source's own `scoreDetails` value, unchanged. Optional. |
| `types` | list of string | The source's own `types` value, unchanged. Optional. |

### Reply_PgxAnnotationHistory

Source-native PgxAnnotationHistory fields; no clinical interpretation.

| Field | Type | Meaning |
| --- | --- | --- |
| `date` | string or null | The source's own `date` value, unchanged. Optional. |
| `description` | string or null | Native `description` when supplied. Optional. |
| `id` | [Reply_Identifier_int64](/reference/schemas/types/#reply-identifier-int64) | The source's own `id` value, unchanged. |
| `type` | string or null | The source's own `type` value, unchanged. Optional. |
| `version` | integer or null | The source's own `version` value, unchanged. Optional. |

### Reply_PgxAnnotationIdentity

Exact clinical annotation identity, distinct from any genomic allele identity.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [Reply_ClinPgxAccession](/reference/schemas/types/#reply-clinpgxaccession) | Selected exact accession. |
| `source` | [Reply_PgxSource](/reference/schemas/types/#reply-pgxsource) | Explicit ClinPGx source. |

</details>

## Next

[`biomcp get pgx`](/reference/commands/get-pgx/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
