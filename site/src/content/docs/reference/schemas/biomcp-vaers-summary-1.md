---
title: Reply of biomcp search adverse-event (biomcp.vaers_summary/1)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp search
  adverse-event: a real example and the fields of schema
  biomcp.vaers_summary/1."
goal: "Parse the biomcp.vaers_summary/1 reply: see a real example and what each
  field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.vaers_summary/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search adverse-event`](/reference/commands/search-adverse-event/) replies with `schema` set to `biomcp.vaers_summary/1`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp search adverse-event MMR --source vaers --limit 2
```

`biomcp schema biomcp.vaers_summary/1` prints the full JSON Schema.

## Example reply

`biomcp search adverse-event MMR --source vaers --limit 2` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

```json
{
  "schema": "biomcp.vaers_summary/1",
  "status": "failed",
  "query": {
    "text": "MMR",
    "source": "vaers",
    "limit": 2,
    "offset": 0
  },
  "sources": [
    {
      "source": "vaers",
      "use": "restricted",
      "terms": "CDC WONDER data use restrictions",
      "terms_url": "https://wonder.cdc.gov/datause.html",
      "state": "failed",
      "reason": "bad_reply"
    }
  ]
}
```

The source did not answer that run, so `status` is `failed` and `result` is missing. A reply that succeeds fills `result` with the fields below.

## Fields

`result` holds [VaersSummary](#vaerssummary). `query` holds [AdverseEventSearch](/reference/schemas/types/#adverseeventsearch), a shared type. `query` repeats the request as BioMCP read it.

### VaersSummary

Bounded page of one independent native vaccine summary table.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `dataset` | map of string | yes | Native dataset attributes including processed-through vintage. |
| `footer` | [VaersRow](#vaersrow) | yes | Whole native footer cells, never a sum calculated by this program. |
| `grouping` | map of string | yes | Native grouping attributes. |
| `interpretation` | string | yes | Reviewed interpretation boundary. |
| `measure_metadata` | list of map of string | yes | Response-specific measure metadata without denominator inference. |
| `measures` | list of map of string | yes | Source measure definitions, including original names and codes. |
| `notices` | list of [VaersNotice](#vaersnotice) | yes | Applied native messages, evaluated caveats and footnotes. |
| `report_total` | string | yes | Native report-total message, separate from the footer aggregation. |
| `rows` | list of [VaersRow](#vaersrow) | yes | Native row order with local paging. |
| `scope` | [VaersScope](#vaersscope) | yes | Native fixed State / Territory selection. |
| `vaccine` | map of string | yes | Native selected vaccine attributes. |

### VaersNotice

Applied source text, retaining its original content and source role.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `kind` | string | yes | Message, footnote or evaluated caveat. |
| `text` | string | yes | Original source text, including embedded publisher markup. |

### VaersRow

One whole native row, never reconstructed from another grouping.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `label` | map of string | yes | Native label cell; no code is invented when absent. |
| `percent` | map of string | yes | Native Percent cell, retaining values above one hundred percent. |
| `reports` | map of string | yes | Native Events Reported count for this row. A footer counts occurrences, separate from the message report total; suppressed and displayed-total text stay unchanged. |

### VaersScope

Fixed native geographic selection, without a caller-selectable geography filter.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `value` | map of string | yes | Exact selected native value and its own catalog label. |
| `variable` | map of string | yes | Native variable attributes, including source name and code. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp search adverse-event`](/reference/commands/search-adverse-event/) gives the command's arguments.
