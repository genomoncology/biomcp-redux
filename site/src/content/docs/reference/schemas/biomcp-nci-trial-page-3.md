---
title: Reply of biomcp search trial (biomcp.nci_trial_page/3)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp search trial: a real
  example and the fields of schema biomcp.nci_trial_page/3."
goal: "Parse the biomcp.nci_trial_page/3 reply: see a real example and what each
  field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.nci_trial_page/3
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search trial`](/reference/commands/search-trial/) replies with `schema` set to `biomcp.nci_trial_page/3`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp search trial --keyword melanoma --source nci --limit 1
```

`biomcp schema biomcp.nci_trial_page/3` prints the full JSON Schema.

## Example reply

`biomcp search trial --keyword melanoma --source nci --limit 1` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.nci_trial_page/3",
  "status": "failed",
  "query": {
    "source": "nci",
    "keyword": "melanoma",
    "count_only": false,
    "has_results": false,
    "limit": 1
  },
  "sources": [
    {
      "source": "nci",
      "use": "restricted",
      "terms": "NCI Clinical Trials Search publisher terms; user's API key re…",
      "terms_url": "https://www.cancer.gov/syndication/api",
      "state": "skipped",
      "reason": "no_key"
    }
  ],
  "next": [{"kind": "key", "args": ["help", "key", "nci"]}]
}
```

The source did not answer that run, so `status` is `failed` and `result` is missing. A reply that succeeds fills `result` with the fields below.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [NciTrialPage](#ncitrialpage). `query` holds [TrialSearch](/reference/schemas/types/#trialsearch), a shared type. `query` repeats the request as BioMCP read it.

### NciTrialPage

NCI search page in native order with native count.

| Field | Type | Meaning |
| --- | --- | --- |
| `attribution` | string | Publisher attribution. |
| `condition_resolution` | [TrialConditionResolution](/reference/schemas/types/#trialconditionresolution) or null | Exact condition identity and source-provided mapping, absent for literal text. Optional. |
| `items` | list of [NciTrial](/reference/schemas/types/#ncitrial) | Source rows, never merged with another provider. |
| `request` | list of [TrialParameter](/reference/schemas/types/#trialparameter) | Exact source parameters, excluding credentials. |

## Next

[`biomcp search trial`](/reference/commands/search-trial/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
