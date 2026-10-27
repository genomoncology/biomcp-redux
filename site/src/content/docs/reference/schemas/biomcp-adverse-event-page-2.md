---
title: Reply of biomcp search adverse-event (biomcp.adverse_event_page/2)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp search
  adverse-event: a real example and the fields of schema
  biomcp.adverse_event_page/2."
goal: "Parse the biomcp.adverse_event_page/2 reply: see a real example and what
  each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.adverse_event_page/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search adverse-event`](/reference/commands/search-adverse-event/) replies with `schema` set to `biomcp.adverse_event_page/2`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp search adverse-event vemurafenib --limit 3
biomcp search adverse-event pacemaker --source maude --limit 2
```

`biomcp schema biomcp.adverse_event_page/2` prints the full JSON Schema.

## Example reply

`biomcp search adverse-event pacemaker --source maude --limit 2` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.adverse_event_page/2",
  "status": "complete",
  "query": {
    "text": "pacemaker",
    "source": "maude",
    "limit": 2,
    "offset": 0
  },
  "result": {
    "source_query": "device.generic_name:\"pacemaker\"",
    "source_updated": "2026-10-13",
    "fetched": "2026-10-27",
    "items": [
      {…},
      … 1 more
    ],
    "unit": "reports",
    "components": [
      {…}
    ]
  },
  "page": {"offset": 0, "returned": 2, "total": 756370},
  "sources": [
    {
      "source": "maude",
      "use": "open",
      "terms": "openFDA terms and license; third-party exceptions",
      "terms_url": "https://open.fda.gov/terms/",
      "state": "ok"
    }
  ],
  "next": [
    {
      "kind": "disambiguate",
      "args": ["get", "adverse-event", "50941416", "--source", "maude"]
    },
    … 2 more
  ]
}
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [AdverseEventPage](#adverseeventpage). `query` holds [AdverseEventSearch](/reference/schemas/types/#adverseeventsearch), a shared type. `query` repeats the request as BioMCP read it.

### AdverseEventPage

Ordered report headers; totals count reports with no population denominator.

| Field | Type | Meaning |
| --- | --- | --- |
| `components` | list of [ComponentLabel](/reference/schemas/types/#componentlabel) | Applicable component terms. |
| `fetched` | [PartialDate](/reference/schemas/types/#partialdate) or null | Original retrieval date belonging to this reply, absent for undated legacy recordings. Optional. |
| `items` | list of [AdverseEventHeader](/reference/schemas/types/#adverseeventheader) | Native ordered report candidates. |
| `reaction_match` | [AdverseEventReactionMatch](#adverseeventreactionmatch) or null | Explicit analyzed reaction selector semantics when a reaction was supplied. Optional. |
| `source_query` | string | Exact literal provider expression; reaction selectors use phrase match. |
| `source_updated` | [PartialDate](/reference/schemas/types/#partialdate) or null | The publisher last_updated date, separate from retrieval. Optional. |
| `unit` | [ReportUnit](#reportunit) | Explicit reports unit for envelope page totals. |

### AdverseEventReactionMatch

Source reaction selector matching semantics.

Values: `phrase_match`.

### ReportUnit

The count describes reports, never exposed people or causal events.

| Value | Meaning |
| --- | --- |
| `reports` | Native report count without a population denominator. |

## Next

[`biomcp search adverse-event`](/reference/commands/search-adverse-event/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
