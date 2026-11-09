---
title: Reply of biomcp get adverse-event (biomcp.adverse-event-faers-card/5)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get adverse-event:
  the fields of schema biomcp.adverse-event-faers-card/5."
goal: "Parse the biomcp.adverse-event-faers-card/5 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.adverse-event-faers-card/5
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get adverse-event`](/reference/commands/get-adverse-event/) and [`biomcp get`](/reference/commands/get/) reply with `schema` set to `biomcp.adverse-event-faers-card/5`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.adverse-event-faers-card/5` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_AdverseEventFaersCard](#reply_adverseeventfaerscard). `query` holds [Reply_AdverseEventGet](/reference/schemas/types/#reply_adverseeventget), a shared type. `query` repeats the request as BioMCP read it.

### Reply_AdverseEventFaersCard

FAERS card with native report-level facts and independent collection lengths.

| Field | Type | Meaning |
| --- | --- | --- |
| `available_sections` | list of [Reply_AdverseEventSection](/reference/schemas/types/#reply_adverseeventsection) | Named sections available in this report. Optional. |
| `components` | list of [Reply_ComponentLabel](/reference/schemas/types/#reply_componentlabel) | Applicable component terms. Optional. |
| `drug_count` | integer (uint32) | Actual drug-array length, never count bucket. |
| `first_received_format` | string or null | Native first_received_format string; present empty values remain present. Optional. |
| `header` | [Reply_AdverseEventHeader](/reference/schemas/types/#reply_adverseeventheader) | Exact selected report identity. |
| `latest_received` | string or null | Native latest_received string; present empty values remain present. Optional. |
| `latest_received_format` | string or null | Native latest_received_format string; present empty values remain present. Optional. |
| `patient` | [Reply_AdverseEventPatient](#reply_adverseeventpatient) or null | Fixed native patient strings. Optional. |
| `reaction_count` | integer (uint32) | Actual reaction-array length, never causal pairs. |
| `report_url` | [Reply_Text](/reference/schemas/types/#reply_text) | Exact successful source request URL, never guessed. |
| `reporter` | [Reply_AdverseEventReporter](#reply_adverseeventreporter) or null | Optional reporter facts. Optional. |
| `seriousness` | [Reply_FaersSeriousness](#reply_faersseriousness) or null | Independent source report flags. Optional. |
| `snapshot` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Opaque checked retained-report snapshot identity. Optional. |
| `source_url` | [Reply_Text](/reference/schemas/types/#reply_text) | Publisher overview, never a next step. |
| `transmitted` | string or null | Native transmitted string; present empty values remain present. Optional. |
| `transmitted_format` | string or null | Native transmitted_format string; present empty values remain present. Optional. |

### Other types in this reply

5 more types appear only in this reply: [Reply_AdverseEventPatient](#reply_adverseeventpatient), [Reply_AdverseEventReporter](#reply_adverseeventreporter), [Reply_FaersAgeUnit](#reply_faersageunit), [Reply_FaersSeriousness](#reply_faersseriousness), [Reply_WithUnit_FiniteNumber_FaersAgeUnit](#reply_withunit_finitenumber_faersageunit). Open the list below to read their fields.

<details>
<summary>Fields of the 5 other types</summary>

### Reply_AdverseEventPatient

Native patient facts with the published word beside the sex code.

| Field | Type | Meaning |
| --- | --- | --- |
| `age` | [Reply_WithUnit_FiniteNumber_FaersAgeUnit](#reply_withunit_finitenumber_faersageunit) or null | Native numeric age with the exact publisher unit code, when stated. Optional. |
| `sex` | string or null | Native sex string, including a present empty value. Optional. |
| `sex_label` | string or null | Published word for the sex code, when the code is on openFDA's list. Optional. |
| `weight` | string or null | Native weight string, including a present empty value. Optional. |

### Reply_AdverseEventReporter

Native reporter facts with the published word beside the qualification code.

| Field | Type | Meaning |
| --- | --- | --- |
| `country` | string or null | Native country string, including a present empty value. Optional. |
| `qualification` | string or null | Native qualification string, including a present empty value. Optional. |
| `qualification_label` | string or null | Published word for the qualification code, when the code is on openFDA's list. Optional. |

### Reply_FaersAgeUnit

Exact openFDA patientonsetageunit codes.

Values: `800`, `801`, `802`, `803`, `804`, `805`.

### Reply_FaersSeriousness

Independent overlapping report-level flags; no CTCAE or reaction-outcome conversion.

| Field | Type | Meaning |
| --- | --- | --- |
| `congenitalanomali` | string or null | Native congenitalanomali string, including a present empty value. Optional. |
| `death` | string or null | Native death string, including a present empty value. Optional. |
| `disabling` | string or null | Native disabling string, including a present empty value. Optional. |
| `hospitalization` | string or null | Native hospitalization string, including a present empty value. Optional. |
| `lifethreatening` | string or null | Native lifethreatening string, including a present empty value. Optional. |
| `other` | string or null | Native other string, including a present empty value. Optional. |

### Reply_WithUnit_FiniteNumber_FaersAgeUnit

A source number with its explicitly stated unit; missing units stay absent.

| Field | Type | Meaning |
| --- | --- | --- |
| `unit` | [Reply_FaersAgeUnit](#reply_faersageunit) or null | Publisher unit or code, without conversion or inference. Optional. |
| `value` | [Reply_FiniteNumber](/reference/schemas/types/#reply_finitenumber) | Unchanged numeric source value. |

</details>

## Next

[`biomcp get adverse-event`](/reference/commands/get-adverse-event/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
