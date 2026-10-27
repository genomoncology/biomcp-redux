---
title: Reply of biomcp get pathway summary section (biomcp.pathway_card/5)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get pathway summary:
  a real example and the fields of schema biomcp.pathway_card/5."
goal: "Parse the biomcp.pathway_card/5 reply: see a real example and what each
  field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.pathway_card/5
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get pathway summary`](/reference/commands/get-pathway/) and [`biomcp get pathway`](/reference/commands/get-pathway/) reply with `schema` set to `biomcp.pathway_card/5`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp get pathway R-HSA-9652817 summary
biomcp get pathway R-HSA-9652817
```

`biomcp schema biomcp.pathway_card/5` prints the full JSON Schema.

## Example reply

`biomcp get pathway R-HSA-9652817 summary` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

```json
{
  "schema": "biomcp.pathway_card/5",
  "status": "partial",
  "query": {
    "id": "R-HSA-9652817",
    "section": "summary",
    "section_limit": 5,
    "section_offset": 0,
    "fresh": false
  },
  "result": {
    "pathway": {
      "id": {"source": "reactome", "id": "R-HSA-9652817"},
      "name": "Signaling by MAPK mutants",
      "species": "Homo sapiens",
      "summations": [
        {
          "db_id": 9653039,
          "text": "Mutations in ERK proteins (MAPK1 and MAPK2) are rare, with…"
        }
      ],
      "references": [
        {
          "db_id": 9652901,
          "pubmed": "24390348",
          "title": "Landscape of genomic alterations in cervical carcinomas",
          "url": "http://www.ncbi.nlm.nih.gov/pubmed/24390348"
        },
        … 4 more
      ],
      "version": "R-HSA-9652817.2"
    },
    "sections": ["summary", "genes", "events"]
  },
  "sources": [
    {
      "source": "reactome",
      "use": "open",
      "terms": "CC0 1.0 database data",
      "terms_url": "https://reactome.org/license",
      "state": "failed",
      "reason": "rejected"
    }
  ],
  "next": [
    {
      "kind": "section",
      "args": ["get", "pathway", "R-HSA-9652817", "genes"]
    },
    … 1 more
  ]
}
```

## Fields

`result` holds [PathwayCard](#pathwaycard). `query` holds [PathwayGet](#pathwayget). `query` repeats the request as BioMCP read it.

### PathwayCard

One checked source pathway with supported disclosures and whole native text.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `description_truncated` | boolean or null | no | Whether the source itself shortened its description, rather than the client. |
| `events` | [Section](/reference/schemas/types/#section) of [PathwayEvents](/reference/schemas/types/#pathwayevents) or null | no | Requested bounded event page, including unresolved integer references. |
| `genes` | [Section](/reference/schemas/types/#section) of [PathwayGenes](/reference/schemas/types/#pathwaygenes) or null | no | Requested bounded membership page, including its explicit source state. |
| `pathway` | [Pathway](#pathway) | yes | Source-native identity, species, versions and requested description. |
| `release` | [Text](/reference/schemas/types/#text) or null | no | Native Reactome database release, separate from pathway version. |
| `sections` | list of [PathwaySection](#pathwaysection) | yes | Source-supported named sections; WikiPathways excludes events. |

### PathwayGet

Get an exact human Reactome R-HSA or WikiPathways WP card. Read whole summary, genes, Reactome events or bounded all sections. Membership names a source and release; it implies no therapy, response or actionability. No enrichment is calculated.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `fresh` | boolean | no | Request a new source observation. |
| `id` | string | yes | Exact human Reactome or WikiPathways identity without version suffixes. |
| `pool` | [Text](/reference/schemas/types/#text) | no | Retained snapshot, when continuing a source relation. |
| `section` | [PathwaySection](#pathwaysection) | no | Omit for the card; WikiPathways has summary and genes sections. |
| `section_limit` | integer (uint32) | no | Bounded initial section size; each all section has its own first page. |
| `section_offset` | integer (uint32) | no | Native relation offset for a selected genes or events section. |

### Pathway

Neutral pathway facts, without display sections or source calls.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `cited_in` | string or null | no | WikiPathways catalog citedIn cell, including a native empty cell. |
| `description` | string or null | no | Complete native description when present; empty is distinct from absent. |
| `id` | [PathwayId](/reference/schemas/types/#pathwayid) | yes | Checked source-owned identity. |
| `name` | [Text](/reference/schemas/types/#text) | yes | Whole native pathway name. |
| `references` | list of [PathwayLiteratureRef](#pathwayliteratureref) or null | no | Native pathway-level literature references, separate from each summation. |
| `revision` | [Text](/reference/schemas/types/#text) or null | no | WikiPathways native revision, separate from identity. |
| `species` | [PathwaySpecies](/reference/schemas/types/#pathwayspecies) | yes | Explicit source-stated human species. |
| `summations` | list of [PathwaySummation](#pathwaysummation) or null | no | Native Reactome summation objects in source order; not concatenated. |
| `version` | [Text](/reference/schemas/types/#text) or null | no | Whole native Reactome stIdVersion, separate from stable identity. |

### PathwayLiteratureRef

A native pathway-level literature reference; never inferred from prose.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `db_id` | integer (uint64) | yes | Native literature database reference. |
| `pubmed` | [Pmid](/reference/schemas/types/#pmid) or null | no | Native PubMed identifier when supplied. |
| `title` | [Text](/reference/schemas/types/#text) or null | no | Native title when supplied. |
| `url` | [Text](/reference/schemas/types/#text) or null | no | Source record URL when supplied. |

### PathwaySection

Supported source-owned pathway disclosures.

| Value | Meaning |
| --- | --- |
| `summary` | Complete native source description. |
| `genes` | Namespaced native participants with exact resolved gene joins. |
| `events` | Native Reactome event objects and unresolved database references. |
| `all` | Each source-supported section's bounded first page. |

### PathwaySummation

One native summation, with its own database identity and whole text.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `db_id` | integer (uint64) | yes | Native summation database identity, not the pathway identity. |
| `text` | string | yes | Complete native text; no sentence clipping or reference reassignment. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp get pathway`](/reference/commands/get-pathway/) gives the command's arguments.
