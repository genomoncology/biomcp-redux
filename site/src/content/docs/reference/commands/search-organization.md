---
title: biomcp search organization
description: "The BioMCP 1.0.0 command biomcp search organization: Search
  clinical research organizations."
goal: Look up the arguments, sections and examples of biomcp search organization.
reader: A BioMCP user or agent checking one command
sidebar:
  label: search organization
kind: reference
verified: 2026-11-09
---

`biomcp search organization` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It search clinical research organizations. This page lists its arguments, examples and reply.

The program's help describes it:

> Search clinical research organizations. Sources: nci.

## Usage

```text
biomcp search organization [OPTIONS]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--city` | string | no |  |  | Source organization city. Value form: string. Example: --city Boston. Command sources: nci; source restrictions in the field description apply. |
| `--country` | string | no |  |  | Source organization country. Value form: string. Example: --country 'United States'. Command sources: nci; source restrictions in the field description apply. |
| `--distance` | string | no |  |  | Positive radius with explicit mi or km units. Value form: string. Example: --distance 10mi. Command sources: nci; source restrictions in the field description apply. |
| `--family` | string | no |  |  | Source organization family. Value form: string. Example: --family 'Dana-Farber/Harvard Cancer Center'. Command sources: nci; source restrictions in the field description apply. |
| `--lat` | string | no |  |  | Radius center latitude in degrees; requires lon and distance. Value form: string. Example: --lat 42.3469. Command sources: nci; source restrictions in the field description apply. |
| `--limit` | integer | no | min 1, max 50 |  | Five headers by default, at most fifty. Value form: integer. Example: --limit 5. Command sources: nci; source restrictions in the field description apply. |
| `--lon` | string | no |  |  | Radius center longitude in degrees; requires lat and distance. Value form: string. Example: --lon -71.1025. Command sources: nci; source restrictions in the field description apply. |
| `--name` | string | no |  |  | NCI partial organization name; names are not unique. Value form: string. Example: --name Dana-Farber. Command sources: nci; source restrictions in the field description apply. |
| `--offset` | integer | no | min 0 |  | Offset within the complete retained organization snapshot. Value form: integer. Example: --offset 0. Command sources: nci; source restrictions in the field description apply. |
| `--pool` | string | no |  |  | Snapshot identity retained by continuation commands. Value form: string. Example: --pool &lt;pool&gt; (syntax; replace the placeholder with your retained value). Command sources: nci; source restrictions in the field description apply. |
| `--postal-code` | string | no |  |  | Source postal code. Value form: string. Example: --postal-code 02215. Command sources: nci; source restrictions in the field description apply. |
| `--source` | string | yes |  | `nci` | Explicit keyed NCI source; help search organization Explicit keyed NCI source. Listed choices accept any letter case; replies use their listed spelling. Value form: string. Example: --source nci. Command sources: nci; source restrictions in the field description apply. |
| `--state` | string | no |  |  | Source state or province. Value form: string. Example: --state MA. Command sources: nci; source restrictions in the field description apply. |

## Sources

`--source` picks the source to ask: [nci](/sources/nci/).

## Examples

The program's help gives these examples.

```sh
biomcp search organization --source nci --name Dana-Farber --limit 5
```

```sh
biomcp search organization --source nci --city Boston --state MA --limit 5
```

```sh
biomcp search organization --source nci --name Dana-Farber --city Boston \
  --state MA --country 'United States' --postal-code 02215 \
  --family 'Dana-Farber/Harvard Cancer Center' --lat 42.3469 --lon -71.1025 \
  --distance 10mi
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.organization-page/4`](/reference/schemas/biomcp-organization-page-4/), whose page lists its fields.

## Next

The [organization guide](/user-guide/organization/) shows every organization command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
