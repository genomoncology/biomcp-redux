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

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get adverse-event drugs`](/reference/commands/get-adverse-event/) replies with `schema` set to `biomcp.adverse_event_drugs/1`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp get adverse-event 26907716 drugs
```

`biomcp schema biomcp.adverse_event_drugs/1` prints the full JSON Schema.

## Example reply

`biomcp get adverse-event 26907716 drugs` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

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
      "records": [
        {
          "index": 0,
          "medicinalproduct": "COBIMETINIB",
          "role": "1",
          "indication": "Langerhans^ cell histiocytosis",
          "drugrecurreadministration": "3",
          "drugseparatedosagenumb": "1",
          "actiondrug": "5",
          "drugadditional": "3",
          "identity_counts": {…}
        },
        … 4 more
      ],
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
        "--snapshot",
        "04a589ed93a30065c4532d2eb2fa5f8083ca9716d4185b8d2e91711430d7ab78"
      ]
    },
    … 3 more
  ]
}
```

## Fields

`result` holds [AdverseEventDrugs](#adverseeventdrugs). `query` holds [AdverseEventGet](/reference/schemas/types/#adverseeventget), a shared type. `query` repeats the request as BioMCP read it.

### AdverseEventDrugs

A bounded report section; all rows belong to the exact header and snapshot.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `components` | list of [ComponentLabel](/reference/schemas/types/#componentlabel) | yes | Applicable component terms. |
| `drugs` | [AdverseEventRecords](/reference/schemas/types/#adverseeventrecords) of [AdverseEventDrug](#adverseeventdrug) | yes | One page of complete native rows. |
| `header` | [AdverseEventHeader](/reference/schemas/types/#adverseeventheader) | yes | Exact source report identity and version. |
| `snapshot` | [Text](/reference/schemas/types/#text) | yes | Opaque retained source snapshot. |

### AdverseEventDrug

One absolute native drug row without its unbounded identifier arrays.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `actiondrug` | string or null | no | Native actiondrug string; present empty values remain present. |
| `drugadditional` | string or null | no | Native drugadditional string; present empty values remain present. |
| `drugadministrationroute` | string or null | no | Native drugadministrationroute string; present empty values remain present. |
| `drugauthorizationnumb` | string or null | no | Native drugauthorizationnumb string; present empty values remain present. |
| `drugbatchnumb` | string or null | no | Native drugbatchnumb string; present empty values remain present. |
| `drugcumulativedosagenumb` | string or null | no | Native drugcumulativedosagenumb string; present empty values remain present. |
| `drugcumulativedosageunit` | string or null | no | Native drugcumulativedosageunit string; present empty values remain present. |
| `drugdosageform` | string or null | no | Native drugdosageform string; present empty values remain present. |
| `drugdosagetext` | string or null | no | Native drugdosagetext string; present empty values remain present. |
| `drugenddate` | string or null | no | Native drugenddate string; present empty values remain present. |
| `drugenddateformat` | string or null | no | Native drugenddateformat string; present empty values remain present. |
| `drugintervaldosagedefinition` | string or null | no | Native drugintervaldosagedefinition string; present empty values remain present. |
| `drugintervaldosageunitnumb` | string or null | no | Native drugintervaldosageunitnumb string; present empty values remain present. |
| `drugrecurreadministration` | string or null | no | Native drugrecurreadministration string; present empty values remain present. |
| `drugseparatedosagenumb` | string or null | no | Native drugseparatedosagenumb string; present empty values remain present. |
| `drugstartdate` | string or null | no | Native drugstartdate string; present empty values remain present. |
| `drugstartdateformat` | string or null | no | Native drugstartdateformat string; present empty values remain present. |
| `drugstructuredosagenumb` | string or null | no | Native drugstructuredosagenumb string; present empty values remain present. |
| `drugstructuredosageunit` | string or null | no | Native drugstructuredosageunit string; present empty values remain present. |
| `drugtreatmentduration` | string or null | no | Native drugtreatmentduration string; present empty values remain present. |
| `drugtreatmentdurationunit` | string or null | no | Native drugtreatmentdurationunit string; present empty values remain present. |
| `identity_counts` | [AdverseEventIdentityCounts](#adverseeventidentitycounts) | yes | Sizes only; original lists live behind named pages. |
| `index` | integer (uint32) | yes | Absolute native array index. |
| `indication` | string or null | no | Native indication string; present empty values remain present. |
| `medicinalproduct` | string or null | no | Native medicinalproduct string; present empty values remain present. |
| `role` | string or null | no | Native role string; present empty values remain present. |

### AdverseEventIdentityCounts

Lengths of present native harmonized lists; missing stays absent.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `application_number` | integer or null | no | Actual native application_number list length. |
| `package_ndc` | integer or null | no | Actual native package_ndc list length. |
| `product_ndc` | integer or null | no | Actual native product_ndc list length. |
| `rxcui` | integer or null | no | Actual native rxcui list length. |
| `spl_id` | integer or null | no | Actual native spl_id list length. |
| `spl_set_id` | integer or null | no | Actual native spl_set_id list length. |
| `unii` | integer or null | no | Actual native unii list length. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp get adverse-event`](/reference/commands/get-adverse-event/) gives the command's arguments.
