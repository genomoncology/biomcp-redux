---
title: Reply of biomcp get adverse-event drugs section (biomcp.adverse_event_drugs/1)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get adverse-event
  drugs: a real example and the fields of schema biomcp.adverse_event_drugs/1."
goal: "Parse the biomcp.adverse_event_drugs/1 reply: see a real example and what
  each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.adverse_event_drugs/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get adverse-event drugs`](/reference/commands/get-adverse-event/) replies with `schema` set to `biomcp.adverse_event_drugs/1`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp get adverse-event 26907716 drugs
```

`biomcp schema biomcp.adverse_event_drugs/1` prints the full JSON Schema.

## Example reply

`biomcp get adverse-event 26907716 drugs` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.adverse_event_drugs/1",
  "status": "complete",
  "query": {
    "id": "26907716",
    "source": "faers",
    "section": "drugs",
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
    "drugs": {
      "records": […],
      "offset": 0,
      "total": 6
    },
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
      "kind": "page",
      "args": [
        "get", "adverse-event", "26907716", "drugs", "--section-offset", "5",
…
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [AdverseEventDrugs](#adverseeventdrugs). `query` holds [AdverseEventGet](/reference/schemas/types/#adverseeventget), a shared type. `query` repeats the request as BioMCP read it.

### AdverseEventDrugs

A bounded report section; all rows belong to the exact header and snapshot.

| Field | Type | Meaning |
| --- | --- | --- |
| `components` | list of [ComponentLabel](/reference/schemas/types/#componentlabel) | Applicable component terms. |
| `drugs` | [AdverseEventRecords](/reference/schemas/types/#adverseeventrecords) of [AdverseEventDrug](#adverseeventdrug) | One page of complete native rows. |
| `header` | [AdverseEventHeader](/reference/schemas/types/#adverseeventheader) | Exact source report identity and version. |
| `snapshot` | [Text](/reference/schemas/types/#text) | Opaque retained source snapshot. |

### Other types in this reply

2 more types appear only in this reply: [AdverseEventDrug](#adverseeventdrug), [AdverseEventIdentityCounts](#adverseeventidentitycounts). Open the list below to read their fields.

<details>
<summary>Fields of the 2 other types</summary>

### AdverseEventDrug

One absolute native drug row without its unbounded identifier arrays.

| Field | Type | Meaning |
| --- | --- | --- |
| `actiondrug` | string or null | Native actiondrug string; present empty values remain present. Optional. |
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
| `identity_counts` | [AdverseEventIdentityCounts](#adverseeventidentitycounts) | Sizes only; original lists live behind named pages. |
| `index` | integer (uint32) | Absolute native array index. |
| `indication` | string or null | Native indication string; present empty values remain present. Optional. |
| `medicinalproduct` | string or null | Native medicinalproduct string; present empty values remain present. Optional. |
| `role` | string or null | Native role string; present empty values remain present. Optional. |

### AdverseEventIdentityCounts

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

</details>

## Next

[`biomcp get adverse-event`](/reference/commands/get-adverse-event/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
