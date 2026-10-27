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

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search adverse-event`](/reference/commands/search-adverse-event/) replies with `schema` set to `biomcp.vaers_summary/1`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp search adverse-event MMR --source vaers --limit 2
```

`biomcp schema biomcp.vaers_summary/1` prints the full JSON Schema.

## Example reply

`biomcp search adverse-event MMR --source vaers --limit 2` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

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

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [VaersSummary](#vaerssummary). `query` holds [AdverseEventSearch](/reference/schemas/types/#adverseeventsearch), a shared type. `query` repeats the request as BioMCP read it.

### VaersSummary

Bounded page of one independent native vaccine summary table.

| Field | Type | Meaning |
| --- | --- | --- |
| `dataset` | map of string | Native dataset attributes including processed-through vintage. |
| `footer` | [VaersRow](#vaersrow) | Whole native footer cells, never a sum calculated by this program. |
| `grouping` | map of string | Native grouping attributes. |
| `interpretation` | string | Reviewed interpretation boundary. |
| `measure_metadata` | list of map of string | Response-specific measure metadata without denominator inference. |
| `measures` | list of map of string | Source measure definitions, including original names and codes. |
| `notices` | list of [VaersNotice](#vaersnotice) | Applied native messages, evaluated caveats and footnotes. |
| `report_total` | string | Native report-total message, separate from the footer aggregation. |
| `rows` | list of [VaersRow](#vaersrow) | Native row order with local paging. |
| `scope` | [VaersScope](#vaersscope) | Native fixed State / Territory selection. |
| `vaccine` | map of string | Native selected vaccine attributes. |

### Other types in this reply

3 more types appear only in this reply: [VaersNotice](#vaersnotice), [VaersRow](#vaersrow), [VaersScope](#vaersscope). Open the list below to read their fields.

<details>
<summary>Fields of the 3 other types</summary>

### VaersNotice

Applied source text, retaining its original content and source role.

| Field | Type | Meaning |
| --- | --- | --- |
| `kind` | string | Message, footnote or evaluated caveat. |
| `text` | string | Original source text, including embedded publisher markup. |

### VaersRow

One whole native row, never reconstructed from another grouping.

| Field | Type | Meaning |
| --- | --- | --- |
| `label` | map of string | Native label cell; no code is invented when absent. |
| `percent` | map of string | Native Percent cell, retaining values above one hundred percent. |
| `reports` | map of string | Native Events Reported count for this row. A footer counts occurrences, separate from the message report total; suppressed and displayed-total text stay unchanged. |

### VaersScope

Fixed native geographic selection, without a caller-selectable geography filter.

| Field | Type | Meaning |
| --- | --- | --- |
| `value` | map of string | Exact selected native value and its own catalog label. |
| `variable` | map of string | Native variable attributes, including source name and code. |

</details>

## Next

[`biomcp search adverse-event`](/reference/commands/search-adverse-event/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
