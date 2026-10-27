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
verified: 2026-10-27
---

`biomcp get diagnostic` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It reads one diagnostic test record, and optionally one section such as its genes or conditions. A [section word](/concepts/progressive-disclosure/) after the identifier adds one more part of the record to the reply. This page lists its arguments, sections and examples.

The program's help describes it:

> Get one exact source record or supported section. A listing is not authorization; a companion row is not a treatment recommendation.

## Usage

```text
biomcp get diagnostic [OPTIONS] <id> [section]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--bundle-revision` | value | no |  |  | Immutable stored [bundle](/local-data/) revision retained by a continuation. |
| `--section-limit` | integer | no | min 1, max 50 |  | Maximum native rows; omitted means five rows, or one regulatory fragment. |
| `--section-offset` | integer | no | min 0 |  | Zero-based native row or fragment offset. |
| `--source` | string | yes |  | `gtr`, `who-ivd`, `fda-pma`, `fda-510k` | Explicit source; gtr needs accession/version, who-ivd a known Product ID, fda-pma a P number, fda-510k a K number. |
| `--supplement` | value | no |  |  | Exact native PMA supplement; absent selects the base row. |
| `id` (position 1) | string | yes |  |  | Exact native identity, including version for GTR. |
| `section` (position 2) | value | no |  |  | gtr: genes, conditions, methods. fda-pma: regulatory, supplements. WHO and 510(k) have no sections; omit for the card. |

## Sections

gtr: genes, conditions, methods. fda-pma: regulatory, supplements. WHO and 510(k) have no sections; omit for the card.

| Section | Description |
| --- | --- |
| `genes` | GTR native gene rows; five per page. |
| `conditions` | GTR native condition rows; five per page. |
| `methods` | GTR native method entries; five per page. |
| `regulatory` | Selected PMA submission statement; one fragment per page. |
| `supplements` | PMA base application supplement headers; five per page. |

## Sources

`--source` picks the source to ask: [gtr](/sources/gtr/), [who-ivd](/sources/who-ivd/), [fda-pma](/sources/fda-pma/), [fda-510k](/sources/fda-510k/).

## Examples

The program's help gives these examples.

```sh
biomcp get diagnostic 0466-046-00 --source who-ivd
```

```sh
biomcp get diagnostic GTR000006692.3 --source gtr
```

## Next

The [diagnostic guide](/user-guide/diagnostic/) shows every diagnostic command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
