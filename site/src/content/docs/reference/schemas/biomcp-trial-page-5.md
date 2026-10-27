---
title: Reply of biomcp search trial (biomcp.trial_page/5)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp search trial: a real
  example and the fields of schema biomcp.trial_page/5."
goal: "Parse the biomcp.trial_page/5 reply: see a real example and what each
  field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.trial_page/5
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search trial`](/reference/commands/search-trial/) replies with `schema` set to `biomcp.trial_page/5`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp search trial --condition 'non-small cell lung cancer' \
  --variant 'KRAS G12C' --status RECRUITING --limit 3
biomcp search trial --condition melanoma --has-results --count-only
```

`biomcp schema biomcp.trial_page/5` prints the full JSON Schema.

## Example reply

`biomcp search trial --condition 'non-small cell lung cancer' --variant 'KRAS G12C' --status RECRUITING --limit 3` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

```json
{
  "schema": "biomcp.trial_page/5",
  "status": "complete",
  "query": {
    "condition": "non-small cell lung cancer",
    "variant": "KRAS G12C",
    "status": "RECRUITING",
    "count_only": false,
    "has_results": false,
    "limit": 3
  },
  "result": {
    "items": [
      {
        "nct_id": "NCT05254184",
        "title": "KRAS-Targeted Vaccine With Chemoimmunotherapy, Nivolumab an…",
        "status": "RECRUITING",
        "phases": ["PHASE1"],
        "conditions": {"items": ["Non-Small Cell Lung Cancer"], "total": 1},
        "interventions": {…},
        "updated": "2026-03-30",
        "url": "https://clinicaltrials.gov/study/NCT05254184"
      },
      … 2 more
    ],
    "request": [
      {"name": "countTotal", "value": "true"},
      … 5 more
    ],
    "source_updated": "2026-10-27T09:00:04",
    "attribution": "ClinicalTrials.gov; source-native discovery, no eligibilit…"
  },
  "page": {"offset": 0, "returned": 3, "total": 53},
  "sources": [
    {
      "source": "ctgov",
      "use": "restricted",
      "terms": "ClinicalTrials.gov attribution and source processing date; pu…",
      "terms_url": "https://clinicaltrials.gov/about-site/terms-conditions",
      "state": "ok",
      "count": 53,
      "fetched": 3
    }
  ],
  "next": [
    {
      "kind": "page",
      "args": [
        "search", "trial", "--condition", "non-small cell lung cancer",
        "--limit", "3", "--next-page",
        "ZVNj7o2Elu8o3lpzTd6s47mtmpOQJJxsY_Wi3vYW",
        … 4 more
      ]
    }
  ]
}
```

## Fields

`result` holds [TrialPage](#trialpage). `query` holds [TrialSearch](/reference/schemas/types/#trialsearch), a shared type. `query` repeats the request as BioMCP read it.

### TrialPage

Registry page preserving source order and exact request parameters.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `attribution` | string | yes | Publisher attribution. |
| `condition_resolution` | [TrialConditionResolution](/reference/schemas/types/#trialconditionresolution) or null | no | Exact condition identity and source-provided mapping, absent for literal text. |
| `count_request` | list of [TrialParameter](/reference/schemas/types/#trialparameter) or null | no | Exact cached native count request when a continuation omits its total. |
| `gene_resolution` | [GeneResolution](/reference/schemas/types/#generesolution) or null | no | Unresolved gene identity, without a registry request. |
| `items` | list of [TrialHeader](#trialheader) | yes | Native study headers in registry order. |
| `request` | list of [TrialParameter](/reference/schemas/types/#trialparameter) | yes | Exact parameters sent to the study search endpoint. |
| `source_updated` | [Text](/reference/schemas/types/#text) or null | no | Registry's source processing timestamp, independently of retrieval time. |

### TrialHeader

Small source-attributed trial discovery record.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `conditions` | [Capped](/reference/schemas/types/#capped) of [Text](/reference/schemas/types/#text) | yes | Source condition names. |
| `interventions` | [Capped](/reference/schemas/types/#capped) of [Text](/reference/schemas/types/#text) | yes | Source intervention names. |
| `nct_id` | [NctId](/reference/schemas/types/#nctid) | yes | Exact registry identity. |
| `phases` | list of [TrialPhase](/reference/schemas/types/#trialphase) | yes | Source phases; absent phases remain absent. |
| `status` | [TrialStatus](/reference/schemas/types/#trialstatus) | yes | Registry's overall status, independent of site status. |
| `title` | [Text](/reference/schemas/types/#text) | yes | Complete brief title. |
| `updated` | [PartialDate](/reference/schemas/types/#partialdate) | yes | Last posted registry update. |
| `url` | [Text](/reference/schemas/types/#text) | yes | Publisher's canonical record URL. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp search trial`](/reference/commands/search-trial/) gives the command's arguments.
