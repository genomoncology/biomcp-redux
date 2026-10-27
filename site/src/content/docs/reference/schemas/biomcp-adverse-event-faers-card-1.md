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

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get adverse-event`](/reference/commands/get-adverse-event/) and [`biomcp get adverse-event drugs`](/reference/commands/get-adverse-event/) reply with `schema` set to `biomcp.adverse_event_faers_card/1`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp get adverse-event 26907716
biomcp get adverse-event 100 drugs
```

`biomcp schema biomcp.adverse_event_faers_card/1` prints the full JSON Schema.

## Example reply

`biomcp get adverse-event 26907716` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

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
…
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [AdverseEventFaersCard](#adverseeventfaerscard). `query` holds [AdverseEventGet](/reference/schemas/types/#adverseeventget), a shared type. `query` repeats the request as BioMCP read it.

### AdverseEventFaersCard

FAERS card with native report-level facts and independent collection lengths.

| Field | Type | Meaning |
| --- | --- | --- |
| `available_sections` | list of [AdverseEventSection](/reference/schemas/types/#adverseeventsection) | Named sections available in this report. |
| `components` | list of [ComponentLabel](/reference/schemas/types/#componentlabel) | Applicable component terms. |
| `drug_count` | integer (uint32) | Actual drug-array length, never count bucket. |
| `first_received_format` | string or null | Native first_received_format string; present empty values remain present. Optional. |
| `header` | [AdverseEventHeader](/reference/schemas/types/#adverseeventheader) | Exact selected report identity. |
| `latest_received` | string or null | Native latest_received string; present empty values remain present. Optional. |
| `latest_received_format` | string or null | Native latest_received_format string; present empty values remain present. Optional. |
| `patient` | [FaersPatient](#faerspatient) or null | Fixed native patient strings. Optional. |
| `reaction_count` | integer (uint32) | Actual reaction-array length, never causal pairs. |
| `report_url` | [Text](/reference/schemas/types/#text) | Exact successful source request URL, never guessed. |
| `reporter` | [FaersReporter](#faersreporter) or null | Optional reporter facts. Optional. |
| `seriousness` | [FaersSeriousness](#faersseriousness) or null | Independent source report flags. Optional. |
| `snapshot` | [Text](/reference/schemas/types/#text) | Opaque checked retained-report snapshot identity. |
| `source_url` | [Text](/reference/schemas/types/#text) | Publisher overview, never a next step. |
| `transmitted` | string or null | Native transmitted string; present empty values remain present. Optional. |
| `transmitted_format` | string or null | Native transmitted_format string; present empty values remain present. Optional. |

### FaersPatient

Optional native patient strings without numeric conversion.

| Field | Type | Meaning |
| --- | --- | --- |
| `age` | string or null | Native age string, including a present empty value. Optional. |
| `age_unit` | string or null | Native age_unit string, including a present empty value. Optional. |
| `sex` | string or null | Native sex string, including a present empty value. Optional. |
| `weight` | string or null | Native weight string, including a present empty value. Optional. |

### FaersReporter

Source reporter fields; qualification may be absent.

| Field | Type | Meaning |
| --- | --- | --- |
| `country` | string or null | Native country string, including a present empty value. Optional. |
| `qualification` | string or null | Native qualification string, including a present empty value. Optional. |

### FaersSeriousness

Independent overlapping report-level flags; no CTCAE or reaction-outcome conversion.

| Field | Type | Meaning |
| --- | --- | --- |
| `congenitalanomali` | string or null | Native congenitalanomali string, including a present empty value. Optional. |
| `death` | string or null | Native death string, including a present empty value. Optional. |
| `disabling` | string or null | Native disabling string, including a present empty value. Optional. |
| `hospitalization` | string or null | Native hospitalization string, including a present empty value. Optional. |
| `lifethreatening` | string or null | Native lifethreatening string, including a present empty value. Optional. |
| `other` | string or null | Native other string, including a present empty value. Optional. |

## Next

[`biomcp get adverse-event`](/reference/commands/get-adverse-event/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
