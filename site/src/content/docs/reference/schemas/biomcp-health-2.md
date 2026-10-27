---
title: Reply of biomcp health (biomcp.health/2)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp health: a real
  example and the fields of schema biomcp.health/2."
goal: "Parse the biomcp.health/2 reply: see a real example and what each field
  means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.health/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp health`](/reference/commands/health/) replies with `schema` set to `biomcp.health/2`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp health
```

`biomcp schema biomcp.health/2` prints the full JSON Schema.

## Example reply

`biomcp health` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.health/2",
  "status": "complete",
  "query": null,
  "result": {
    "disable_groups": [
      {
        "group": "open",
        "sources": [
          "hgnc", "reactome", "wikipathways", "interpro", "complex-portal",
          "gnomad", "clingen", "litvar",
          … 11 more
        ],
        "components": [
          {"source": "mychem", "name": "drugbank"},
          … 5 more
        ]
      },
      … 2 more
    ],
    "sources": [
      {
        "source": "cellosaurus",
        "terms": "Cellosaurus CC BY 4.0; cite Bairoch (2018)",
        "terms_url": "https://www.cellosaurus.org/description.html",
        "usage": "open",
        "disabled": false,
        "state": "ok",
        "latency_ms": 656
      },
      … 56 more
    ]
  }
}
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Health](#health). `query` repeats the request as BioMCP read it.

### Health

Source reachability and publisher terms, independent of local HTTP service health.

| Field | Type | Meaning |
| --- | --- | --- |
| `disable_groups` | list of [DisableGroup](#disablegroup) | Sources and carried components excluded by each supported terms group. Optional. |
| `sources` | list of [SourceHealth](#sourcehealth) | Every implemented adapter, in declaration order. |

### ComponentHealth

Policy labels for one carried source component.

| Field | Type | Meaning |
| --- | --- | --- |
| `disabled` | boolean | Explicit source or component group policy prevents this component. |
| `name` | [Text](/reference/schemas/types/#text) | Native component name. |
| `terms` | [ComponentTerms](/reference/schemas/types/#componentterms) | Independently reviewed publisher terms. |

### DisableGroup

The consequences of one source-disable selector, regardless of current configuration.

| Field | Type | Meaning |
| --- | --- | --- |
| `components` | list of [DisabledComponent](#disabledcomponent) | Carried fields stopped while their aggregators remain available. |
| `group` | [Use](/reference/schemas/types/#use) | Terms category accepted by disable_sources. |
| `sources` | list of [Source](/reference/schemas/types/#source) | Whole sources stopped by this group. |

### DisabledComponent

One independently labeled carried field excluded by a terms group.

| Field | Type | Meaning |
| --- | --- | --- |
| `name` | string | Registry-owned component name. |
| `source` | [Source](/reference/schemas/types/#source) | Aggregator carrying the field. |

### SourceHealth

One bounded source probe, without credentials or provider error text.

| Field | Type | Meaning |
| --- | --- | --- |
| `components` | list of [ComponentHealth](#componenthealth) | Independently licensed carried components, without separate network probes. Optional. |
| `disabled` | boolean | Whether source policy prevents all requests and cached reads. |
| `latency_ms` | integer or null | Probe duration in milliseconds; omitted when policy skipped the probe. Optional. |
| `reason` | [Reason](/reference/schemas/types/#reason) or null | Typed failure or skip reason. Optional. |
| `source` | [Source](/reference/schemas/types/#source) | Implemented source identity. |
| `state` | [SourceState](/reference/schemas/types/#sourcestate) | Probe outcome; disabled and missing credentials remain skipped. |
| `terms` | [Text](/reference/schemas/types/#text) | Publisher license or terms name. |
| `terms_url` | [Text](/reference/schemas/types/#text) | Publisher terms page. |
| `usage` | [Use](/reference/schemas/types/#use) | Reviewed source category, not blanket permission for each record. |

## Next

[`biomcp health`](/reference/commands/health/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
