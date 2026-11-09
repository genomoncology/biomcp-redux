---
title: Reply of biomcp search organization (biomcp.organization-page/4)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp search organization:
  the fields of schema biomcp.organization-page/4."
goal: "Parse the biomcp.organization-page/4 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.organization-page/4
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search organization`](/reference/commands/search-organization/) and [`biomcp search`](/reference/commands/search/) reply with `schema` set to `biomcp.organization-page/4`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.organization-page/4` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_OrganizationPage](#reply_organizationpage). `query` holds [Reply_OrganizationSearch](#reply_organizationsearch). `query` repeats the request as BioMCP read it.

### Reply_OrganizationPage

Local page of a complete bounded source observation.

| Field | Type | Meaning |
| --- | --- | --- |
| `items` | list of [Reply_Organization](#reply_organization) | Published organizations in source order, with optional source details. Optional. |

### Reply_OrganizationSearch

Find NCI organizations. Names are not unique; trial counts cover every status, including closed trials. No organization ID or get is available.

| Field | Type | Meaning |
| --- | --- | --- |
| `city` | [Reply_Text](/reference/schemas/types/#reply_text) | Native organization city. Optional. |
| `country` | [Reply_Text](/reference/schemas/types/#reply_text) | Native organization country. Optional. |
| `distance` | [Reply_Text](/reference/schemas/types/#reply_text) | Positive radius with explicit mi or km units. Optional. |
| `family` | [Reply_Text](/reference/schemas/types/#reply_text) | Native organization family. Optional. |
| `lat` | [Reply_Text](/reference/schemas/types/#reply_text) | Radius center latitude in degrees; requires lon and distance. Optional. |
| `limit` | integer (uint32) | Five headers by default, at most fifty. Optional. |
| `lon` | [Reply_Text](/reference/schemas/types/#reply_text) | Radius center longitude in degrees; requires lat and distance. Optional. |
| `name` | [Reply_Text](/reference/schemas/types/#reply_text) | NCI partial organization name; names are not unique. Optional. |
| `offset` | integer (uint32) | Offset within the complete retained organization snapshot. Optional. |
| `pool` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Snapshot identity retained by continuation commands. Optional. |
| `postal_code` | [Reply_Text](/reference/schemas/types/#reply_text) | Native postal code. Optional. |
| `source` | [Reply_OrganizationSource](#reply_organizationsource) | Explicit keyed NCI source. |
| `state` | [Reply_Text](/reference/schemas/types/#reply_text) | Native state or province. Optional. |

### Other types in this reply

3 more types appear only in this reply: [Reply_Organization](#reply_organization), [Reply_OrganizationCoordinates](#reply_organizationcoordinates), [Reply_OrganizationSource](#reply_organizationsource). Open the list below to read their fields.

<details>
<summary>Fields of the 3 other types</summary>

### Reply_Organization

One NCI organization search row. Names are not unique; counts cover trials of every status.

| Field | Type | Meaning |
| --- | --- | --- |
| `contact_email` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Published public contact email. Optional. |
| `contact_name` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Published public contact name. Optional. |
| `contact_phone` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Published public contact telephone. Optional. |
| `count` | integer or null | NCI trial count across all statuses, including closed trials. Optional. |
| `name` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Exact publisher organization name, never a unique identifier. Optional. |
| `next` | [Reply_Step](/reference/schemas/types/#reply_step) or null | Search trials at this exact organization name; names may identify several locations. Optional. |
| `org_address_line_1` | [Reply_Text](/reference/schemas/types/#reply_text) or null | First native address line. Optional. |
| `org_address_line_2` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Second native address line. Optional. |
| `org_city` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Native city. Optional. |
| `org_coordinates` | [Reply_OrganizationCoordinates](#reply_organizationcoordinates) or null | Native geographic coordinates. Optional. |
| `org_country` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Native country. Optional. |
| `org_email` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Published organization email. Optional. |
| `org_family` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Native organization family. Optional. |
| `org_fax` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Published organization fax. Optional. |
| `org_postal_code` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Native postal code. Optional. |
| `org_state_or_province` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Native state or province. Optional. |
| `org_va` | boolean or null | Native Veterans Affairs marker. Optional. |

### Reply_OrganizationCoordinates

Geographic point published by NCI, in degrees.

| Field | Type | Meaning |
| --- | --- | --- |
| `lat` | number (double) | Latitude in degrees. |
| `lon` | number (double) | Longitude in degrees. |

### Reply_OrganizationSource

Organization discovery source.

Values: `nci`.

</details>

## Next

[`biomcp search organization`](/reference/commands/search-organization/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
