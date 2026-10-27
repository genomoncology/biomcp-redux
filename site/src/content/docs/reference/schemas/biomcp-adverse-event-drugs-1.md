---
title: biomcp.adverse_event_drugs/1
description: "The BioMCP 1.0.0 reply schema biomcp.adverse_event_drugs/1: its
  envelope fields, its result type AdverseEventDrugs and the commands that
  return it."
goal: Look up the fields of a biomcp.adverse_event_drugs/1 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.adverse_event_drugs/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.adverse_event_drugs/1`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.adverse_event_drugs/1`.

This call returns it.

- `biomcp get adverse-event 26907716 drugs` ([`biomcp get adverse-event`](/reference/commands/get-adverse-event/))

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | [CacheProblem](#cacheproblem) or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of [Step](#step) | no | Available next commands. |
| `page` | [PageInfo](#pageinfo) or null | no | Search page metadata. |
| `problems` | array of [Problem](#problem) | no | Rejected source fields. |
| `query` | [AdverseEventGet](#query-adverseeventget) | yes | Parsed query. |
| `result` | [AdverseEventDrugs](#result-adverseeventdrugs) or null | no | Answer when available. |
| `schema` | "biomcp.adverse_event_drugs/1" | yes |  |
| `sources` | array of [SourceOutcome](#sourceoutcome) | no | Sources actually asked. |
| `status` | [Status](#status) | yes | Overall outcome. |

## Result: AdverseEventDrugs

A bounded report section; all rows belong to the exact header and snapshot.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `components` | array of [ComponentLabel](#componentlabel) | yes | Applicable component terms. |
| `drugs` | [AdverseEventRecords](#adverseeventrecords) | yes | One page of complete native rows. |
| `header` | [AdverseEventHeader](#adverseeventheader) | yes | Exact source report identity and version. |
| `snapshot` | [Text](#text) | yes | Opaque retained source snapshot. |

## Query: AdverseEventGet

Exact native report identity and an optional independently paged section.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `drug_index` | integer or null | no | Absolute source-array index; required only for identifier sections. |
| `id` | [ReportId](#reportid) | yes | Published report ID with no source prefix. |
| `section` | [AdverseEventSection](#adverseeventsection) or null | no | FAERS owns drugs, reactions and identifiers; MAUDE owns devices and narrative. Omitted means card only. |
| `section_limit` | integer (uint32) | no | Whole source rows per section page. |
| `section_offset` | integer (uint32) | no | Offset within the selected native section. |
| `snapshot` | [Text](#text) or null | no | Checked retained report snapshot for provider-free continuation. |
| `source` | "faers" or "maude" | no | FAERS by default; MAUDE is explicit. |

## Types

Every other type this schema uses.

### AdverseEventDrug

One absolute native drug row without its unbounded identifier arrays.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `actiondrug` | string or null | no | Native actiondrug string; present empty values remain present. |
| `drugadditional` | string or null | no | Native drugadditional string; present empty values remain present. |
| `drugadministrationroute` | string or null | no | Native drugadministrationroute string; present empty values remain present. |
| `drugauthorizationnumb` | string or null | no | Native drugauthorizationnumb string; present empty values remain present. |
| `drugbatchnumb` | string or null | no | Native drugbatchnumb string; present empty values remain present. |
| `drugcumulativedosagenumb` | string or null | no | Native drugcumulativedosagenumb string; present empty values remain present. |
| `drugcumulativedosageunit` | string or null | no | Native drugcumulativedosageunit string; present empty values remain present. |
| `drugdosageform` | string or null | no | Native drugdosageform string; present empty values remain present. |
| `drugdosagetext` | string or null | no | Native drugdosagetext string; present empty values remain present. |
| `drugenddate` | string or null | no | Native drugenddate string; present empty values remain present. |
| `drugenddateformat` | string or null | no | Native drugenddateformat string; present empty values remain present. |
| `drugintervaldosagedefinition` | string or null | no | Native drugintervaldosagedefinition string; present empty values remain present. |
| `drugintervaldosageunitnumb` | string or null | no | Native drugintervaldosageunitnumb string; present empty values remain present. |
| `drugrecurreadministration` | string or null | no | Native drugrecurreadministration string; present empty values remain present. |
| `drugseparatedosagenumb` | string or null | no | Native drugseparatedosagenumb string; present empty values remain present. |
| `drugstartdate` | string or null | no | Native drugstartdate string; present empty values remain present. |
| `drugstartdateformat` | string or null | no | Native drugstartdateformat string; present empty values remain present. |
| `drugstructuredosagenumb` | string or null | no | Native drugstructuredosagenumb string; present empty values remain present. |
| `drugstructuredosageunit` | string or null | no | Native drugstructuredosageunit string; present empty values remain present. |
| `drugtreatmentduration` | string or null | no | Native drugtreatmentduration string; present empty values remain present. |
| `drugtreatmentdurationunit` | string or null | no | Native drugtreatmentdurationunit string; present empty values remain present. |
| `identity_counts` | [AdverseEventIdentityCounts](#adverseeventidentitycounts) | yes | Sizes only; original lists live behind named pages. |
| `index` | integer (uint32) | yes | Absolute native array index. |
| `indication` | string or null | no | Native indication string; present empty values remain present. |
| `medicinalproduct` | string or null | no | Native medicinalproduct string; present empty values remain present. |
| `role` | string or null | no | Native role string; present empty values remain present. |

### AdverseEventHeader

Small source report header without a drug title, flattened lists or count buckets.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `event_type` | string or null | no | Native event_type string; present empty values remain present. |
| `first_received` | string or null | no | Native first_received string; present empty values remain present. |
| `id` | [ReportId](#reportid) | yes | Exact published source identity. |
| `serious` | string or null | no | Native serious string; present empty values remain present. |
| `source` | [AdverseEventSource](#adverseeventsource) | yes | Explicit report family. |
| `version` | string or null | no | Native version string; present empty values remain present. |

### AdverseEventIdentityCounts

Lengths of present native harmonized lists; missing stays absent.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `application_number` | integer or null | no | Actual native application_number list length. |
| `package_ndc` | integer or null | no | Actual native package_ndc list length. |
| `product_ndc` | integer or null | no | Actual native product_ndc list length. |
| `rxcui` | integer or null | no | Actual native rxcui list length. |
| `spl_id` | integer or null | no | Actual native spl_id list length. |
| `spl_set_id` | integer or null | no | Actual native spl_set_id list length. |
| `unii` | integer or null | no | Actual native unii list length. |

### AdverseEventRecords

One complete-row page tied to the retained report snapshot.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `offset` | integer (uint32) | yes | Native section offset. |
| `records` | array of [AdverseEventDrug](#adverseeventdrug) | yes | Source ordered whole rows. |
| `total` | integer (uint64) | yes | Complete selected section length, not report or patient count. |

### AdverseEventSection

One source-owned section; no all or reporter section.

Values: `drugs`, `reactions`, `unii`, `rxcui`, `product-ndc`, `package-ndc`, `spl-id`, `spl-set-id`, `application-number`, `devices`, `narrative`.

### AdverseEventSource

An explicit source; never probe the other family.

Values: `faers`, `maude`, `vaers`.

### CacheProblem

A local cache failure that leaves the provider outcome intact.

Values: `busy`, `database`, `version`.

### ComponentLabel

Terms attached to a named carried component on a consumer response.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `component` | [Text](#text) | yes | Native component name. |
| `terms` | [ComponentTerms](#componentterms) | yes | Canonical publisher terms. |

### ComponentTerms

Terms attached to one independently licensed source component.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `name` | [Text](#text) | yes | Publisher terms name. |
| `url` | [Text](#text) | yes | Publisher terms page. |
| `usage` | [Use](#use) | yes | Publisher use category. |

### PageInfo

Search counts after filtering and deduplication.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `offset` | integer (uint32) | yes | Skipped results. |
| `pool` | [PoolInfo](#poolinfo) or null | no | Optional pooled snapshot. |
| `returned` | integer (uint32) | yes | Results returned on this page. |
| `total` | integer (uint64) | yes | Total matching identities held by this snapshot. |
| `truncated` | boolean or null | no | Matching source count exceeded the retrieved window. |

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

### ReportId

Type: string.

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

This page is generated from `biomcp schema biomcp.adverse_event_drugs/1` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
