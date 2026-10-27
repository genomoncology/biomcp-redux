---
title: biomcp.drug_approvals/3
description: "The BioMCP 1.0.0 reply schema biomcp.drug_approvals/3: its
  envelope fields, its result type DrugApprovals and the commands that return
  it."
goal: Look up the fields of a biomcp.drug_approvals/3 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.drug_approvals/3
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.drug_approvals/3`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.drug_approvals/3`.

These calls return it.

- `biomcp get drug NDA202429 approvals --source drugsfda --section-limit 20` ([`biomcp get drug`](/reference/commands/get-drug/))
- `biomcp get drug NDA220910 approvals --source drugsfda` ([`biomcp get drug`](/reference/commands/get-drug/))

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | [CacheProblem](#cacheproblem) or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of [Step](#step) | no | Available next commands. |
| `page` | [PageInfo](#pageinfo) or null | no | Search page metadata. |
| `problems` | array of [Problem](#problem) | no | Rejected source fields. |
| `query` | [DrugGet](#query-drugget) | yes | Parsed query. |
| `result` | [DrugApprovals](#result-drugapprovals) or null | no | Answer when available. |
| `schema` | "biomcp.drug_approvals/3" | yes |  |
| `sources` | array of [SourceOutcome](#sourceoutcome) | no | Sources actually asked. |
| `status` | [Status](#status) | yes | Overall outcome. |

## Result: DrugApprovals

Explicit native application/product history without a global approval flag.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `approvals` | [Section](#section) | yes | Existing typed section state, preserving empty, unavailable and skipped outcomes. |
| `identity` | [DrugHeader](#drugheader) | yes | Selected native identity, checked independently of section facts. |
| `products` | array of [DrugFdaProduct](#drugfdaproduct) | yes | Native products, filtered only by an explicit product number; independent of submissions. |
| `source_updated` | [PartialDate](#partialdate) or null | no | Native FDA dataset last_updated date, independent of submission dates and retrieval. |

## Query: DrugGet

Exact native ID retrieval with an explicitly selected source.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `civic_cursor` | [Text](#text) or null | no | Native continuation cursor for that exact drug, profile and claim collection. |
| `civic_kind` | [CivicClaimKind](#civicclaimkind) or null | no | Independent native evidence or assertion collection within the selected profile. |
| `civic_profile` | integer or null | no | Selected native CIViC molecular profile for a drug-scoped claim stream. |
| `input` | [DrugId](#drugid) | yes | Published source-native identifier, never SOURCE:ID or a selected name. |
| `label_section` | [Text](#text) or null | no | Exact native label heading, required only for the label section. |
| `product_number` | [Text](#text) or null | no | Explicit native application product number, only for approvals. |
| `section` | array of [DrugSection](#drugsection) | no | Explicit named disclosures; empty means identity only. |
| `section_limit` | integer or null | no | Named section page size, refused on an identity-only card. |
| `section_offset` | integer or null | no | Named section continuation, refused on an identity-only card. |
| `source` | [DrugSource](#drugsource) | yes | Required source; there is no default exact-ID resolver. |

## Types

Every other type this schema uses.

### Attribution

Provenance for one source record.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `evaluated` | [PartialDate](#partialdate) or null | no | Source evaluation date. |
| `record` | [Text](#text) or null | no | Native assertion record. |
| `release` | [Text](#text) or null | no | Dataset release. |
| `source` | [Text](#text) | yes | Source responsible for the claim. |
| `submitter` | [Text](#text) or null | no | Source submitter or panel. |
| `url` | [Text](#text) or null | no | Source record link. |
| `version` | [Text](#text) or null | no | Record version. |

### AuthorizationDate

A native date kept with its original role.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `date` | [PartialDate](#partialdate) | yes | Source date at its stated precision. |
| `role` | [Text](#text) | yes | Native date role, without assigning approval meaning. |

### AuthorizationEvent

A source-scoped regulator event; never an identity-card field.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `attribution` | [Attribution](#attribution) | yes | One source record and its native provenance. |
| `dates` | array of [AuthorizationDate](#authorizationdate) | no | All native dates with separate roles. |
| `indication` | [Text](#text) or null | no | Original indication scope when present. |
| `jurisdiction` | [Text](#text) or null | no | Jurisdiction explicitly stated by the source. |
| `kind` | [Text](#text) or null | no | Original native event type. |
| `product_number` | [Text](#text) or null | no | Explicit native product number, never a synthesized first product. |
| `record` | [DrugId](#drugid) | yes | Native regulator record identifier. |
| `status` | [Text](#text) or null | no | Original source status; historical positives imply no current status. |

### CacheProblem

A local cache failure that leaves the provider outcome intact.

Values: `busy`, `database`, `version`.

### CivicClaimKind

CIViC native claim collection.

Values: `evidence`, `assertions`.

### DrugFdaDocument

A native application document link; the command does not download it.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `date` | [PartialDate](#partialdate) | yes | Native document date, independent of event dates. |
| `id` | [Text](#text) | yes | Source document identifier. |
| `type` | [Text](#text) | yes | Original document type. |
| `url` | [Text](#text) | yes | Original source URL. |

### DrugFdaIngredient

An unchanged native ingredient and strength, without local normalization.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `name` | [Text](#text) | yes | Native ingredient name. |
| `strength` | [Text](#text) | no | Full source strength wording. |

### DrugFdaProduct

One explicit native product within the selected FDA application.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `active_ingredients` | array of [DrugFdaIngredient](#drugfdaingredient) | yes | Complete source-listed ingredient and strength pairs. |
| `brand_name` | [Text](#text) | yes | Source brand wording, without inferred ingredient equivalence. |
| `dosage_form` | [Text](#text) | yes | Native dosage form. |
| `marketing_status` | [Text](#text) | yes | Product marketing status, independent of submission history. |
| `product_number` | [Text](#text) | yes | Exact source product number, scoped by the response application. |
| `reference_drug` | [Text](#text) | yes | Original source reference-drug designation. |
| `reference_standard` | [Text](#text) or null | no | Original source reference-standard designation when supplied. |
| `route` | [Text](#text) or null | no | Native route; absent and null remain unavailable. |
| `te_code` | [Text](#text) or null | no | Native therapeutic-equivalence code when supplied. |

### DrugFdaProperty

A source-stated submission property, without interpretation.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `code` | [Text](#text) | yes | Original property code. |

### DrugFdaSubmission

One application-owned submission with common event facts and native FDA context.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `application_docs` | array or null | no | Native documents; document dates remain separate from the submission status date. |
| `event` | [AuthorizationEvent](#authorizationevent) | yes | Historical native status, type and status date; no product applicability is implied. |
| `review_priority` | [Text](#text) or null | no | Original review priority. |
| `submission_class_code` | [Text](#text) or null | no | Native submission class code. |
| `submission_class_code_description` | [Text](#text) or null | no | Native submission class description. |
| `submission_number` | [Text](#text) | yes | Original submission number, scoped by application and event kind. |
| `submission_property_type` | array or null | no | Native property collection; missing stays missing and an empty list stays empty. |

### DrugHeader

One source candidate, without authorization, targets, label text or interactions.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `id` | [DrugId](#drugid) | yes | Published native identifier. |
| `identity_level` | [DrugIdentityLevel](#drugidentitylevel) | yes | Level established only by the endpoint or record type. |
| `name` | [Text](#text) | no | Original source display name. |
| `source` | [DrugSource](#drugsource) | yes | The single source responsible for this header. |
| `type` | [Text](#text) or null | no | Native record type. |
| `url` | [Text](#text) or null | no | Source record URL. |
| `version` | [Text](#text) or null | no | Native source revision, omitted when the source supplies none. |

### DrugId

Type: string.

### DrugIdentityLevel

Identity level established by the source endpoint or record type, never inferred from ID shape.

Values: `ingredient`, `salt`, `clinical_formulation`, `branded_product`, `spl_set`, `spl_version`, `ndc_package`, `fda_application`, `fda_product`, `ema_authorization`, `ema_application`, `orphan_designation`, `unknown`.

### DrugRecords

One bounded native row page, without fabricating an unknown total.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `offset` | integer (uint32) | yes | Source row offset. |
| `records` | array of [DrugFdaSubmission](#drugfdasubmission) | no | Native source rows in order. |
| `retrieved` | [PartialDate](#partialdate) or null | no | Actual retrieval date, never substituted for source revision. |
| `total` | integer or null | no | Native total only when supplied. |
| `truncated` | boolean or null | no | Native source truncation indicator. |
| `version` | [Text](#text) or null | no | Dataset or bundle revision. |

### DrugSection

Explicit drug disclosures; identity cards never expand into these sections.

Values: `xrefs`, `targets`, `indications`, `civic`, `label`, `approvals`, `regulatory`, `interactions`.

### DrugSource

Values: `who-pq`, `mychem`, `chembl`, `opentargets`, `civic`, `openfda-label`, `drugsfda`, `ema`, `ddinter`.

### PageInfo

Search counts after filtering and deduplication.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `offset` | integer (uint32) | yes | Skipped results. |
| `pool` | [PoolInfo](#poolinfo) or null | no | Optional pooled snapshot. |
| `returned` | integer (uint32) | yes | Results returned on this page. |
| `total` | integer (uint64) | yes | Total matching identities held by this snapshot. |
| `truncated` | boolean or null | no | Matching source count exceeded the retrieved window. |

### PartialDate

Type: string.

### PoolInfo

Fixed lifetime and size of a stored search snapshot.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `created` | integer (uint64) | yes | Publication wall time in Unix seconds. |
| `expires` | integer (uint64) | yes | Fixed expiry wall time in Unix seconds. |
| `id` | [Text](#text) | yes | Opaque snapshot identifier. |
| `size` | integer (uint32) | yes | Number of ordered identities. |

### Problem

A field that could not be accepted from a source record.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `code` | [ProblemCode](#problemcode) | yes | Why the field value was not accepted. |
| `field` | string | yes | The biodata field name. |
| `record` | [Text](#text) or null | no | The record's canonical identifier, when known. |
| `source` | [Source](#source) | yes | The source that supplied the record. |

### ProblemCode

A stable code for a rejected field value.

Values: `value.unknown`, `collection.incomplete`, `identity.conflict`.

### Reason

Why a source could not supply data.

Values: `not_found`, `not_open`, `not_pdf`, `unsupported`, `too_large`, `rate_limited`, `unavailable`, `tls`, `proxy`, `rejected`, `bad_reply`, `deadline_exceeded`, `offline`, `no_key`, `disabled`, `replay_miss`, `syncing`, `no_join_id`.

### Section

A requested section has exactly one state.

One of:

- object: The source returned data.
- object: The source answered with no data.
- object: A requested source failed.
- object: User settings prevented the source request.
- object: Entity facts rule this section out.

### Source

A source of BioMCP data, identified by its canonical short name.

Values: `hgnc`, `mygene`, `reactome`, `wikipathways`, `hpa`, `uniprot`, `interpro`, `complex-portal`, `gnomad`, `clinvar`, `erepo`, `gwas`, `clingen`, `pubmed`, `europepmc`, `litvar`, `litsense`, `semantic-scholar`, `orcid`, `pmc-oa`, `icite`, `pubtator`, `ctgov`, `nci`, `civic`, `hotspots`, `alphagenome`, `oncokb`, `myvariant`, `car`, `mutalyzer`, `variantvalidator`, `mychem`, `chembl`, `opentargets`, `openfda-label`, `drugsfda`, `ema`, `who-pq`, `ddinter`, `drugbank-open`, `drugcentral`, `gtopdb`, `unii`, `ndc`, `openfda-xref`, `ols`, `mydisease`, `monarch`, `seer`, `mondo`, `doid`, `ncit`, `hpo`, `orphanet`, `omim`, `faers`, `vaers`, `maude`, `meddra`, `gmdn`, `gtr`, `who-ivd`, `fda-cdx`, `fda-pma`, `fda-510k`, `cpic`, `clinpgx`, `cbioportal`, `cellosaurus`.

### SourceNotice

A provider query interpretation notice.

One of:

- object: A checked provider answer survived a local cache failure.
- object: The source still has graph pages after the bounded fetch window.
- object: An empty LitVar lookup can be repeated with a caller-supplied rsID.
- object: PubMed supplied a named ESearch warning without changing its phrases.
- object: PubMed did not index these words.

### SourceOutcome

One source's outcome, without provider text.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `count` | integer or null | no | Provider matching count. |
| `fetched` | integer or null | no | Retrieved identity depth. |
| `notices` | array of [SourceNotice](#sourcenotice) | no | Provider query notices that do not indicate failure. |
| `reason` | [Reason](#reason) or null | no | Failure or stale-fallback reason. |
| `source` | [Source](#source) | yes | Asked source. |
| `state` | [SourceState](#sourcestate) | yes | Retained state. |
| `term` | [Text](#text) or null | no | Exact provider search term. |
| `terms` | string | yes | Publisher terms name from the source registry. |
| `terms_url` | string | yes | Publisher page documenting the terms. |
| `use` | [Use](#use) | yes | Reviewed source use category; publisher terms remain authoritative. |

### SourceRecord

The source's own entry for an entity.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `id` | [Text](#text) | yes | Native source identifier. |
| `url` | [Text](#text) or null | no | Link to that entry. |

### SourceState

State of a source actually asked by the command.

Values: `ok`, `stale`, `syncing`, `failed`, `skipped`.

### Status

Overall command outcome.

Values: `complete`, `partial`, `absent`, `failed`.

### Step

A command argument list spelled from a typed action.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `args` | array of string | yes | CLI and MCP arguments. |
| `kind` | [StepKind](#stepkind) | yes | Purpose of the action. |

### StepKind

Purpose of an available next command.

Values: `restart`, `section`, `sort`, `page`, `fresh`, `disambiguate`, `pivot`, `sync`, `key`, `configure`.

### Text

Nonempty text with surrounding Unicode whitespace removed.

Type: string.

### Use

Reviewed source-use category, without inferring rights in individual records.

Values: `open`, `attribution`, `noncommercial`, `restricted`.

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.drug_approvals/3` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
