---
title: Reply of biomcp get adverse-event (biomcp.adverse_event_faers_card/1)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get adverse-event: a
  real example and the fields of schema biomcp.adverse_event_faers_card/1."
goal: "Parse the biomcp.adverse_event_faers_card/1 reply: see a real example and
  what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.adverse_event_faers_card/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get adverse-event`](/reference/commands/get-adverse-event/) and [`biomcp get adverse-event drugs`](/reference/commands/get-adverse-event/) reply with `schema` set to `biomcp.adverse_event_faers_card/1`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp get adverse-event 26907716
biomcp get adverse-event 100 drugs
```

`biomcp schema biomcp.adverse_event_faers_card/1` prints the full JSON Schema.

## Example reply

`biomcp get adverse-event 26907716` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

```json
{
  "schema": "biomcp.adverse_event_faers_card/1",
  "status": "complete",
  "query": {
    "id": "26907716",
    "source": "faers",
    "section_limit": 5,
    "section_offset": 0
  },
  "result": {
    "header": {
      "id": "26907716",
      "source": "faers",
      "version": "1",
      "serious": "1",
      "first_received": "20260622"
    },
    "snapshot": "04a589ed93a30065c4532d2eb2fa5f8083ca9716d4185b8d2e91711430d7…",
    "latest_received": "20260622",
    "latest_received_format": "102",
    "transmitted": "20260709",
    "transmitted_format": "102",
    "first_received_format": "102",
    "seriousness": {
      "death": "2",
      "lifethreatening": "2",
      "hospitalization": "2",
      "disabling": "2",
      "congenitalanomali": "2",
      "other": "1"
    },
    "patient": {},
    "reporter": {"qualification": "3", "country": "EU"},
    "drug_count": 6,
    "reaction_count": 5,
    "source_url": "https://open.fda.gov/apis/drug/event/",
    "report_url": "https://api.fda.gov/drug/event.json?search=safetyreportid%…",
    "available_sections": ["drugs", "reactions"],
    "components": []
  },
  "sources": [
    {
      "source": "faers",
      "use": "open",
      "terms": "openFDA terms and license; third-party exceptions",
      "terms_url": "https://open.fda.gov/terms/",
      "state": "ok"
    }
  ],
  "next": [
    {
      "kind": "section",
      "args": [
        "get", "adverse-event", "26907716", "drugs", "--snapshot",
        "04a589ed93a30065c4532d2eb2fa5f8083ca9716d4185b8d2e91711430d7ab78"
      ]
    },
    … 1 more
  ]
}
```

## Fields

`result` holds [AdverseEventFaersCard](#adverseeventfaerscard). `query` holds [AdverseEventGet](/reference/schemas/types/#adverseeventget), a shared type. `query` repeats the request as BioMCP read it.

### AdverseEventFaersCard

FAERS card with native report-level facts and independent collection lengths.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `available_sections` | list of [AdverseEventSection](/reference/schemas/types/#adverseeventsection) | yes | Named sections available in this report. |
| `components` | list of [ComponentLabel](/reference/schemas/types/#componentlabel) | yes | Applicable component terms. |
| `drug_count` | integer (uint32) | yes | Actual drug-array length, never count bucket. |
| `first_received_format` | string or null | no | Native first_received_format string; present empty values remain present. |
| `header` | [AdverseEventHeader](/reference/schemas/types/#adverseeventheader) | yes | Exact selected report identity. |
| `latest_received` | string or null | no | Native latest_received string; present empty values remain present. |
| `latest_received_format` | string or null | no | Native latest_received_format string; present empty values remain present. |
| `patient` | [FaersPatient](#faerspatient) or null | no | Fixed native patient strings. |
| `reaction_count` | integer (uint32) | yes | Actual reaction-array length, never causal pairs. |
| `report_url` | [Text](/reference/schemas/types/#text) | yes | Exact successful source request URL, never guessed. |
| `reporter` | [FaersReporter](#faersreporter) or null | no | Optional reporter facts. |
| `seriousness` | [FaersSeriousness](#faersseriousness) or null | no | Independent source report flags. |
| `snapshot` | [Text](/reference/schemas/types/#text) | yes | Opaque checked retained-report snapshot identity. |
| `source_url` | [Text](/reference/schemas/types/#text) | yes | Publisher overview, never a next step. |
| `transmitted` | string or null | no | Native transmitted string; present empty values remain present. |
| `transmitted_format` | string or null | no | Native transmitted_format string; present empty values remain present. |

### FaersPatient

Optional native patient strings without numeric conversion.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `age` | string or null | no | Native age string, including a present empty value. |
| `age_unit` | string or null | no | Native age_unit string, including a present empty value. |
| `sex` | string or null | no | Native sex string, including a present empty value. |
| `weight` | string or null | no | Native weight string, including a present empty value. |

### FaersReporter

Source reporter fields; qualification may be absent.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `country` | string or null | no | Native country string, including a present empty value. |
| `qualification` | string or null | no | Native qualification string, including a present empty value. |

### FaersSeriousness

Independent overlapping report-level flags; no CTCAE or reaction-outcome conversion.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `congenitalanomali` | string or null | no | Native congenitalanomali string, including a present empty value. |
| `death` | string or null | no | Native death string, including a present empty value. |
| `disabling` | string or null | no | Native disabling string, including a present empty value. |
| `hospitalization` | string or null | no | Native hospitalization string, including a present empty value. |
| `lifethreatening` | string or null | no | Native lifethreatening string, including a present empty value. |
| `other` | string or null | no | Native other string, including a present empty value. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp get adverse-event`](/reference/commands/get-adverse-event/) gives the command's arguments.
