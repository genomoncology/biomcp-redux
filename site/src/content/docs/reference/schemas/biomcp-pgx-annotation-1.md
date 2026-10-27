---
title: Reply of biomcp get pgx (biomcp.pgx_annotation/1)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get pgx: a real
  example and the fields of schema biomcp.pgx_annotation/1."
goal: "Parse the biomcp.pgx_annotation/1 reply: see a real example and what each
  field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.pgx_annotation/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get pgx`](/reference/commands/get-pgx/) replies with `schema` set to `biomcp.pgx_annotation/1`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp get pgx --annotation-id PA166169507 --source clinpgx
```

`biomcp schema biomcp.pgx_annotation/1` prints the full JSON Schema.

## Example reply

`biomcp get pgx --annotation-id PA166169507 --source clinpgx` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

```json
{
  "schema": "biomcp.pgx_annotation/1",
  "status": "complete",
  "query": {"id": "PA166169507", "source": "clinpgx"},
  "result": {
    "header": {"source": "clinpgx", "id": "PA166169507"},
    "annotation": {
      "id": 1448640301,
      "accessionId": "PA166169507",
      "allelePhenotypes": […],
      "conflictingVariantAnnotationIds": [],
      "history": […],
      "levelOfEvidence": {…},
      "location": {…},
      "name": "rs3918290 (DPYD); fluorouracil; Neoplasms (level 4 Efficacy)",
      "objCls": "SummaryAnnotation",
      "overrideLevel": false,
      "pediatric": true,
      "relatedAlleles": […],
      "relatedChemicals": […],
      "relatedChemicalsLogic": "Or",
      "relatedDiseases": […],
      "relatedGuidelines": [],
      "relatedLabels": [],
      "score": -3.5,
      "scoreDetails": "{\"vaScores\": [{\"step5A\": 1.0, \"step5B\": -1.0, \"…",
      "types": ["Efficacy"]
    }
  },
  "sources": [
    {
      "source": "clinpgx",
      "use": "noncommercial",
      "terms": "ClinPGx data use policy; noncommercial access restriction; ca…",
      "terms_url": "https://www.clinpgx.org/page/dataUsagePolicy",
      "state": "ok"
    }
  ]
}
```

## Fields

`result` holds [PgxAnnotation](#pgxannotation). `query` holds [PgxAnnotationGet](#pgxannotationget). `query` repeats the request as BioMCP read it.

### PgxAnnotation

Complete proved base-tier annotation.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `annotation` | [PgxAnnotationDetail](#pgxannotationdetail) | yes | Complete native base record, including history and genotype text. |
| `header` | [PgxAnnotationIdentity](#pgxannotationidentity) | yes | Exact selected source identity. |

### PgxAnnotationGet

Exact single clinical annotation get, with no inferred source or annotation kind.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `id` | [ClinPgxAccession](/reference/schemas/types/#clinpgxaccession) | yes | Checked ClinPGx accession, never a shared PaId. |
| `source` | [PgxSource](/reference/schemas/types/#pgxsource) | yes | Required ClinPGx source. |

### PgxAllelePhenotype

Source-native PgxAllelePhenotype fields; no clinical interpretation.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `allele` | string | yes | The source's own `allele` value, unchanged. |
| `id` | integer (int64) | yes | The source's own `id` value, unchanged. |
| `limitedEvidence` | boolean | yes | The source's own `limitedEvidence` value, unchanged. |
| `phenotype` | string | yes | The source's own `phenotype` value, unchanged. |

### PgxAnnotationDetail

Source-native PgxAnnotationDetail fields; no clinical interpretation.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `accessionId` | [ClinPgxAccession](/reference/schemas/types/#clinpgxaccession) | yes | The source's own `accessionId` value, unchanged. |
| `allelePhenotypes` | list of [PgxAllelePhenotype](#pgxallelephenotype) | yes | The source's own `allelePhenotypes` value, unchanged. |
| `conflictingVariantAnnotationIds` | list of integer (int64) | yes | The source's own `conflictingVariantAnnotationIds` value, unchanged. |
| `history` | list of [PgxAnnotationHistory](#pgxannotationhistory) | yes | The source's own `history` value, unchanged. |
| `id` | integer (int64) | yes | The source's own `id` value, unchanged. |
| `levelOfEvidence` | [PgxEvidenceTerm](/reference/schemas/types/#pgxevidenceterm) | yes | The source's own `levelOfEvidence` value, unchanged. |
| `location` | [PgxLocation](/reference/schemas/types/#pgxlocation) | yes | The source's own `location` value, unchanged. |
| `name` | string | yes | The source's own `name` value, unchanged. |
| `objCls` | string | yes | The source's own `objCls` value, unchanged. |
| `overrideLevel` | boolean | yes | The source's own `overrideLevel` value, unchanged. |
| `overrideLevelDescription` | string or null | no | Native `overrideLevelDescription` when supplied. |
| `pediatric` | boolean | yes | The source's own `pediatric` value, unchanged. |
| `relatedAlleles` | list of [PgxObjectReference](/reference/schemas/types/#pgxobjectreference) | yes | The source's own `relatedAlleles` value, unchanged. |
| `relatedChemicals` | list of [PgxObjectReference](/reference/schemas/types/#pgxobjectreference) | yes | The source's own `relatedChemicals` value, unchanged. |
| `relatedChemicalsLogic` | string | yes | The source's own `relatedChemicalsLogic` value, unchanged. |
| `relatedDiseases` | list of [PgxObjectReference](/reference/schemas/types/#pgxobjectreference) | yes | The source's own `relatedDiseases` value, unchanged. |
| `relatedGuidelines` | list of [PgxObjectReference](/reference/schemas/types/#pgxobjectreference) | yes | The source's own `relatedGuidelines` value, unchanged. |
| `relatedLabels` | list of [PgxObjectReference](/reference/schemas/types/#pgxobjectreference) | yes | The source's own `relatedLabels` value, unchanged. |
| `score` | [FiniteNumber](/reference/schemas/types/#finitenumber) | yes | The source's own `score` value, unchanged. |
| `scoreDetails` | string | yes | The source's own `scoreDetails` value, unchanged. |
| `types` | list of string | yes | The source's own `types` value, unchanged. |

### PgxAnnotationHistory

Source-native PgxAnnotationHistory fields; no clinical interpretation.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `date` | string | yes | The source's own `date` value, unchanged. |
| `description` | string or null | no | Native `description` when supplied. |
| `id` | integer (int64) | yes | The source's own `id` value, unchanged. |
| `type` | string | yes | The source's own `type` value, unchanged. |
| `version` | integer (int64) | yes | The source's own `version` value, unchanged. |

### PgxAnnotationIdentity

Exact clinical annotation identity, distinct from any genomic allele identity.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `id` | [ClinPgxAccession](/reference/schemas/types/#clinpgxaccession) | yes | Selected exact accession. |
| `source` | [PgxSource](/reference/schemas/types/#pgxsource) | yes | Explicit ClinPGx source. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp get pgx`](/reference/commands/get-pgx/) gives the command's arguments.
