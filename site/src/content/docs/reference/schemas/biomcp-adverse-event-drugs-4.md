---
title: Reply of biomcp get adverse-event (biomcp.adverse-event-drugs/4)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get adverse-event:
  the fields of schema biomcp.adverse-event-drugs/4."
goal: "Parse the biomcp.adverse-event-drugs/4 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.adverse-event-drugs/4
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get adverse-event`](/reference/commands/get-adverse-event/) and [`biomcp get`](/reference/commands/get/) reply with `schema` set to `biomcp.adverse-event-drugs/4`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.adverse-event-drugs/4` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_AdverseEventDrugs](#reply_adverseeventdrugs). `query` holds [Reply_AdverseEventGet](/reference/schemas/types/#reply_adverseeventget), a shared type. `query` repeats the request as BioMCP read it.

### Reply_AdverseEventDrugs

A bounded report section; all rows belong to the exact header and snapshot.

| Field | Type | Meaning |
| --- | --- | --- |
| `components` | list of [Reply_ComponentLabel](/reference/schemas/types/#reply_componentlabel) | Applicable component terms. Optional. |
| `drugs` | [Reply_AdverseEventRecords_AdverseEventDrug](#reply_adverseeventrecords_adverseeventdrug) | One page of complete native rows. |
| `header` | [Reply_AdverseEventHeader](/reference/schemas/types/#reply_adverseeventheader) | Exact source report identity and version. |
| `snapshot` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Opaque retained source snapshot. Optional. |

### Other types in this reply

3 more types appear only in this reply: [Reply_AdverseEventDrug](#reply_adverseeventdrug), [Reply_AdverseEventIdentityCounts](#reply_adverseeventidentitycounts), [Reply_AdverseEventRecords_AdverseEventDrug](#reply_adverseeventrecords_adverseeventdrug). Open the list below to read their fields.

<details>
<summary>Fields of the 3 other types</summary>

### Reply_AdverseEventDrug

One absolute native drug row without its unbounded identifier arrays.

| Field | Type | Meaning |
| --- | --- | --- |
| `actiondrug` | string or null | Native actiondrug string; present empty values remain present. Optional. |
| `actiondrug_label` | string or null | Published word for the action code, when the code is on openFDA's list. Optional. |
| `drugadditional` | string or null | Native drugadditional string; present empty values remain present. Optional. |
| `drugadministrationroute` | string or null | Native drugadministrationroute string; present empty values remain present. Optional. |
| `drugauthorizationnumb` | string or null | Native drugauthorizationnumb string; present empty values remain present. Optional. |
| `drugbatchnumb` | string or null | Native drugbatchnumb string; present empty values remain present. Optional. |
| `drugcumulativedosagenumb` | string or null | Native drugcumulativedosagenumb string; present empty values remain present. Optional. |
| `drugcumulativedosageunit` | string or null | Native drugcumulativedosageunit string; present empty values remain present. Optional. |
| `drugdosageform` | string or null | Native drugdosageform string; present empty values remain present. Optional. |
| `drugdosagetext` | string or null | Native drugdosagetext string; present empty values remain present. Optional. |
| `drugenddate` | string or null | Native drugenddate string; present empty values remain present. Optional. |
| `drugenddateformat` | string or null | Native drugenddateformat string; present empty values remain present. Optional. |
| `drugintervaldosagedefinition` | string or null | Native drugintervaldosagedefinition string; present empty values remain present. Optional. |
| `drugintervaldosageunitnumb` | string or null | Native drugintervaldosageunitnumb string; present empty values remain present. Optional. |
| `drugrecurreadministration` | string or null | Native drugrecurreadministration string; present empty values remain present. Optional. |
| `drugseparatedosagenumb` | string or null | Native drugseparatedosagenumb string; present empty values remain present. Optional. |
| `drugstartdate` | string or null | Native drugstartdate string; present empty values remain present. Optional. |
| `drugstartdateformat` | string or null | Native drugstartdateformat string; present empty values remain present. Optional. |
| `drugstructuredosagenumb` | string or null | Native drugstructuredosagenumb string; present empty values remain present. Optional. |
| `drugstructuredosageunit` | string or null | Native drugstructuredosageunit string; present empty values remain present. Optional. |
| `drugtreatmentduration` | string or null | Native drugtreatmentduration string; present empty values remain present. Optional. |
| `drugtreatmentdurationunit` | string or null | Native drugtreatmentdurationunit string; present empty values remain present. Optional. |
| `identity_counts` | [Reply_AdverseEventIdentityCounts](#reply_adverseeventidentitycounts) | Sizes only; original lists live behind named pages. |
| `index` | integer (uint32) | Absolute native array index. |
| `indication` | string or null | Native indication string; present empty values remain present. Optional. |
| `medicinalproduct` | string or null | Native medicinalproduct string; present empty values remain present. Optional. |
| `role` | string or null | Native role string; present empty values remain present. Optional. |
| `role_label` | string or null | Published word for the role code, when the code is on openFDA's list. Optional. |

### Reply_AdverseEventIdentityCounts

Lengths of present native harmonized lists; missing stays absent.

| Field | Type | Meaning |
| --- | --- | --- |
| `application_number` | integer or null | Actual native application_number list length. Optional. |
| `package_ndc` | integer or null | Actual native package_ndc list length. Optional. |
| `product_ndc` | integer or null | Actual native product_ndc list length. Optional. |
| `rxcui` | integer or null | Actual native rxcui list length. Optional. |
| `spl_id` | integer or null | Actual native spl_id list length. Optional. |
| `spl_set_id` | integer or null | Actual native spl_set_id list length. Optional. |
| `unii` | integer or null | Actual native unii list length. Optional. |

### Reply_AdverseEventRecords_AdverseEventDrug

One complete-row page tied to the retained report snapshot.

| Field | Type | Meaning |
| --- | --- | --- |
| `offset` | integer (uint32) | Native section offset. |
| `records` | list of [Reply_AdverseEventDrug](#reply_adverseeventdrug) | Source ordered whole rows. Optional. |
| `total` | integer (uint64) | Complete selected section length, not report or patient count. |

</details>

## Next

[`biomcp get adverse-event`](/reference/commands/get-adverse-event/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
