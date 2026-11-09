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

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search trial`](/reference/commands/search-trial/) replies with `schema` set to `biomcp.trial_page/5`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp search trial --condition 'non-small cell lung cancer' \
  --variant 'KRAS G12C' --status RECRUITING --limit 3
biomcp search trial --condition melanoma --has-results --count-only
```

`biomcp schema biomcp.trial_page/5` prints the full JSON Schema.

## Example reply

`biomcp search trial --condition 'non-small cell lung cancer' --variant 'KRAS G12C' --status RECRUITING --limit 3` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

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
      {…},
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
…
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [TrialPage](#trialpage). `query` holds [TrialSearch](/reference/schemas/types/#trialsearch), a shared type. `query` repeats the request as BioMCP read it.

### TrialPage

Registry page preserving source order and exact request parameters.

| Field | Type | Meaning |
| --- | --- | --- |
| `attribution` | string | Publisher attribution. |
| `condition_resolution` | [TrialConditionResolution](/reference/schemas/types/#trialconditionresolution) or null | Exact condition identity and source-provided mapping, absent for literal text. Optional. |
| `count_request` | list of [TrialParameter](/reference/schemas/types/#trialparameter) or null | Exact cached native count request when a continuation omits its total. Optional. |
| `gene_resolution` | [GeneResolution](/reference/schemas/types/#generesolution) or null | Unresolved gene identity, without a registry request. Optional. |
| `items` | list of [TrialHeader](#trialheader) | Native study headers in registry order. |
| `request` | list of [TrialParameter](/reference/schemas/types/#trialparameter) | Exact parameters sent to the study search endpoint. |
| `source_updated` | [Text](/reference/schemas/types/#text) or null | Registry's source processing timestamp, independently of retrieval time. Optional. |

### Other types in this reply

One more type appear only in this reply: [TrialHeader](#trialheader). Open the list below to read their fields.

<details>
<summary>Fields of the 1 other type</summary>

### TrialHeader

Small source-attributed trial discovery record.

| Field | Type | Meaning |
| --- | --- | --- |
| `conditions` | [Capped](/reference/schemas/types/#capped) of [Text](/reference/schemas/types/#text) | Source condition names. |
| `interventions` | [Capped](/reference/schemas/types/#capped) of [Text](/reference/schemas/types/#text) | Source intervention names. |
| `nct_id` | [NctId](/reference/schemas/types/#nctid) | Exact registry identity. |
| `phases` | list of [TrialPhase](/reference/schemas/types/#trialphase) | Source phases; absent phases remain absent. |
| `status` | [TrialStatus](/reference/schemas/types/#trialstatus) | Registry's overall status, independent of site status. |
| `title` | [Text](/reference/schemas/types/#text) | Complete brief title. |
| `updated` | [PartialDate](/reference/schemas/types/#partialdate) | Last posted registry update. |
| `url` | [Text](/reference/schemas/types/#text) | Publisher's canonical record URL. |

</details>

## Next

[`biomcp search trial`](/reference/commands/search-trial/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
