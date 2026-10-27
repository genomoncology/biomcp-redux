---
title: biomcp get diagnostic
description: "Reference for the BioMCP 1.0.0 command biomcp get diagnostic: Get
  one exact source record or supported section."
goal: Look up the arguments, sections and examples of biomcp get diagnostic.
reader: A BioMCP user or agent checking one command
sidebar:
  label: get diagnostic
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 command `biomcp get diagnostic`.

Get one exact source record or supported section. A listing is not authorization; a companion row is not a treatment recommendation.

## Usage

```text
biomcp get diagnostic [OPTIONS] <id> [section]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--bundle-revision` | value | no |  |  | Immutable stored bundle revision retained by a continuation. |
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

`--source` accepts these sources: [gtr](/sources/gtr/), [who-ivd](/sources/who-ivd/), [fda-pma](/sources/fda-pma/), [fda-510k](/sources/fda-510k/).

## Examples

```sh
biomcp get diagnostic 0466-046-00 --source who-ivd
biomcp get diagnostic GTR000006692.3 --source gtr
```

## Reply schema

The help examples of this command return [`biomcp.diagnostic_card/1`](/reference/schemas/biomcp-diagnostic-card-1/). Print the full JSON schema with `biomcp schema biomcp.diagnostic_card/1`.

## Exit codes

See [exit codes](/reference/error-codes/). Options that apply to every command are in [global options](/reference/global-options/).

## Next

The [diagnostic entity](/user-guide/diagnostic/) page lists every diagnostic command and links to a worked guide.

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
