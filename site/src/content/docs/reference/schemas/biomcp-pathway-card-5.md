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

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get pathway summary`](/reference/commands/get-pathway/) and [`biomcp get pathway`](/reference/commands/get-pathway/) reply with `schema` set to `biomcp.pathway_card/5`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp get pathway R-HSA-9652817 summary
biomcp get pathway R-HSA-9652817
```

`biomcp schema biomcp.pathway_card/5` prints the full JSON Schema.

## Example reply

`biomcp get pathway R-HSA-9652817 summary` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

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
      "summations": […],
      "references": […],
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
    {"kind": "section", "args": ["get", "pathway", "R-HSA-9652817", "genes"]},
    … 1 more
  ]
}
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [PathwayCard](#pathwaycard). `query` holds [PathwayGet](#pathwayget). `query` repeats the request as BioMCP read it.

### PathwayCard

One checked source pathway with supported disclosures and whole native text.

| Field | Type | Meaning |
| --- | --- | --- |
| `description_truncated` | boolean or null | Whether the source itself shortened its description, rather than the client. Optional. |
| `events` | [Section](/reference/schemas/types/#section) of [PathwayEvents](/reference/schemas/types/#pathwayevents) or null | Requested bounded event page, including unresolved integer references. Optional. |
| `genes` | [Section](/reference/schemas/types/#section) of [PathwayGenes](/reference/schemas/types/#pathwaygenes) or null | Requested bounded membership page, including its explicit source state. Optional. |
| `pathway` | [Pathway](#pathway) | Source-native identity, species, versions and requested description. |
| `release` | [Text](/reference/schemas/types/#text) or null | Native Reactome database release, separate from pathway version. Optional. |
| `sections` | list of [PathwaySection](#pathwaysection) | Source-supported named sections; WikiPathways excludes events. |

### PathwayGet

Get an exact human Reactome R-HSA or WikiPathways WP card. Read whole summary, genes, Reactome events or bounded all sections. Membership names a source and release; it implies no therapy, response or actionability. No enrichment is calculated.

| Field | Type | Meaning |
| --- | --- | --- |
| `fresh` | boolean | Request a new source observation. Optional. |
| `id` | string | Exact human Reactome or WikiPathways identity without version suffixes. |
| `pool` | [Text](/reference/schemas/types/#text) | Retained snapshot, when continuing a source relation. Optional. |
| `section` | [PathwaySection](#pathwaysection) | Omit for the card; WikiPathways has summary and genes sections. Optional. |
| `section_limit` | integer (uint32) | Bounded initial section size; each all section has its own first page. Optional. |
| `section_offset` | integer (uint32) | Native relation offset for a selected genes or events section. Optional. |

### Pathway

Neutral pathway facts, without display sections or source calls.

| Field | Type | Meaning |
| --- | --- | --- |
| `cited_in` | string or null | WikiPathways catalog citedIn cell, including a native empty cell. Optional. |
| `description` | string or null | Complete native description when present; empty is distinct from absent. Optional. |
| `id` | [PathwayId](/reference/schemas/types/#pathwayid) | Checked source-owned identity. |
| `name` | [Text](/reference/schemas/types/#text) | Whole native pathway name. |
| `references` | list of [PathwayLiteratureRef](#pathwayliteratureref) or null | Native pathway-level literature references, separate from each summation. Optional. |
| `revision` | [Text](/reference/schemas/types/#text) or null | WikiPathways native revision, separate from identity. Optional. |
| `species` | [PathwaySpecies](/reference/schemas/types/#pathwayspecies) | Explicit source-stated human species. |
| `summations` | list of [PathwaySummation](#pathwaysummation) or null | Native Reactome summation objects in source order; not concatenated. Optional. |
| `version` | [Text](/reference/schemas/types/#text) or null | Whole native Reactome stIdVersion, separate from stable identity. Optional. |

### PathwayLiteratureRef

A native pathway-level literature reference; never inferred from prose.

| Field | Type | Meaning |
| --- | --- | --- |
| `db_id` | integer (uint64) | Native literature database reference. |
| `pubmed` | [Pmid](/reference/schemas/types/#pmid) or null | Native PubMed identifier when supplied. Optional. |
| `title` | [Text](/reference/schemas/types/#text) or null | Native title when supplied. Optional. |
| `url` | [Text](/reference/schemas/types/#text) or null | Source record URL when supplied. Optional. |

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

| Field | Type | Meaning |
| --- | --- | --- |
| `db_id` | integer (uint64) | Native summation database identity, not the pathway identity. |
| `text` | string | Complete native text; no sentence clipping or reference reassignment. |

## Next

[`biomcp get pathway`](/reference/commands/get-pathway/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
