---
title: biomcp get diagnostic
description: "The BioMCP 1.0.0 command biomcp get diagnostic: Reads one
  diagnostic test record, and optionally one section such as its genes or
  conditions."
goal: Look up the arguments, sections and examples of biomcp get diagnostic.
reader: A BioMCP user or agent checking one command
sidebar:
  label: get diagnostic
kind: reference
verified: 2026-11-09
---

`biomcp get diagnostic` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It reads one diagnostic test record, and optionally one section such as its genes or conditions. A [section word](/concepts/progressive-disclosure/) after the identifier adds one more part of the record to the reply. This page lists its arguments, sections, examples and reply.

The program's help describes it:

> Get diagnostic test or device details. Sources: fda-510k, fda-device-recall, fda-pma, gtr, who-ivd.

## Usage

```text
biomcp get diagnostic [OPTIONS] <id> [section]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--bundle-revision` | string | no |  |  | Immutable stored [bundle](/local-data/) revision retained by a continuation. Value form: string. Example: --bundle-revision &lt;bundle-revision&gt; (syntax; replace the placeholder with your retained value). Command sources: fda-510k, fda-device-recall, fda-pma, gtr, who-ivd; source restrictions in the field description apply. |
| `--section-limit` | integer | no | min 1, max 50 |  | Maximum source rows; omitted means five rows, or one regulatory fragment. Value form: integer. Example: --section-limit 2. Command sources: fda-510k, fda-device-recall, fda-pma, gtr, who-ivd; source restrictions in the field description apply. |
| `--section-offset` | integer | no | min 0 |  | Zero-based source row or fragment offset. Value form: integer. Example: --section-offset 5. Command sources: fda-510k, fda-device-recall, fda-pma, gtr, who-ivd; source restrictions in the field description apply. |
| `--source` | string | yes |  | `gtr`, `who-ivd`, `fda-pma`, `fda-510k`, `fda-device-recall` | Explicit source: GTR accession/version, WHO Product ID,; help get diagnostic Explicit source: GTR accession/version, WHO Product ID, PMA P number, 510(k) K number or device recall Z-NNNN-YYYY. Listed choices accept any letter case; replies use their listed spelling. Value form: string. Example: --source fda-device-recall. Command sources: fda-510k, fda-device-recall, fda-pma, gtr, who-ivd; source restrictions in the field description apply. |
| `--supplement` | string | no |  |  | Exact source PMA supplement; absent selects the base row. Value form: string. Example: --supplement S020. Command sources: fda-510k, fda-device-recall, fda-pma, gtr, who-ivd; source restrictions in the field description apply. |
| `id` (position 1) | string | yes |  |  | Exact source identity, including version for GTR. |
| `section` (position 2) | string | no |  | `genes`, `conditions`, `methods`, `summary`, `regulatory`, `supplements` | GTR: genes, conditions, methods. PMA: regulatory,; help get diagnostic Listed choices accept any letter case; replies use their listed spelling. |

## Sections

GTR: genes, conditions, methods. PMA: regulatory, supplements, summary.
WHO, 510(k) and device recalls have no sections; omit for the card.

| Section | Description |
| --- | --- |
| `genes` | GTR source gene rows; five per page. |
| `conditions` | GTR source condition rows; five per page. |
| `methods` | GTR source method entries; five per page. |
| `summary` | FDA SSED PDF text: 25 MiB, 100 pages and 6144 escaped bytes per fragment; scans without text are explicit. |
| `regulatory` | Selected PMA submission statement; one fragment per page. |
| `supplements` | PMA base application supplement headers; five per page. |

## Sources

`--source` picks the source to ask: [gtr](/sources/gtr/), [who-ivd](/sources/who-ivd/), [fda-pma](/sources/fda-pma/), [fda-510k](/sources/fda-510k/), [fda-device-recall](/sources/fda-device-recall/).

## Examples

The program's help gives these examples.

```sh
biomcp get diagnostic Z-0001-2024 --source fda-device-recall
```

```sh
biomcp get diagnostic 0466-046-00 --source who-ivd
```

```sh
biomcp get diagnostic GTR000006692.3 --source gtr
```

```sh
biomcp get diagnostic P150013 --source fda-pma --supplement S020
```

```sh
biomcp get diagnostic GTR000006692.3 --source gtr methods --section-limit 2
```

```sh
biomcp get diagnostic P150013 --source fda-pma supplements --section-offset 5
```

## Reply

The reply is one JSON object. Its schema is one of [`biomcp.diagnostic-card/4`](/reference/schemas/biomcp-diagnostic-card-4/), [`biomcp.diagnostic-conditions/5`](/reference/schemas/biomcp-diagnostic-conditions-5/), [`biomcp.diagnostic-genes/5`](/reference/schemas/biomcp-diagnostic-genes-5/), [`biomcp.diagnostic-methods/4`](/reference/schemas/biomcp-diagnostic-methods-4/), [`biomcp.diagnostic-regulatory/3`](/reference/schemas/biomcp-diagnostic-regulatory-3/), [`biomcp.diagnostic-summary/1`](/reference/schemas/biomcp-diagnostic-summary-1/), [`biomcp.diagnostic-supplements/3`](/reference/schemas/biomcp-diagnostic-supplements-3/), whose page lists its fields.

## Next

The [diagnostic guide](/user-guide/diagnostic/) shows every diagnostic command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
