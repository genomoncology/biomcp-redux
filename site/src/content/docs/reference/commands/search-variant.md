---
title: biomcp search variant
description: "Reference for the BioMCP 1.0.0 command biomcp search variant:
  Discover source candidates using one declared selector, without interpreting
  free text."
goal: Look up the arguments, sections and examples of biomcp search variant.
reader: A BioMCP user or agent checking one command
sidebar:
  label: search variant
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 command `biomcp search variant`.

Discover source candidates using one declared selector, without interpreting free text.

## Usage

```text
biomcp search variant [OPTIONS]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--build` | string | no |  | `GRCh37`, `GRCh38` | Optional caller assembly sent to MyVariant; regions require it explicitly. |
| `--condition` | string | no |  |  | Literal MyVariant clinvar.rcv.conditions.name phrase; source text is not a local diagnosis. Requires a declared protein or region selector. |
| `--gene` | string | no |  |  | Exact checked gene symbol for the fixed dbNSFP gene/protein field query; obeys component terms switches. |
| `--has` | string | no |  | `clinvar` | Require the selected component's identifier field. Only source-proven mappings are offered. |
| `--limit` | integer | no | min 1, max 50 |  | Native candidates per page, default 10, maximum 50; all candidates remain candidates. |
| `--offset` | integer | no | min 0, max 9999 |  | Native offset within this command's bounded result window; larger native totals remain visible. |
| `--protein` | string | no |  |  | Explicit substitution using checked one-letter or three-letter amino-acid codes. |
| `--query` | string | no |  |  | Raw MyVariant query, passed and echoed unchanged when component policy allows; requires --source myvariant. With disabled components use declared selectors instead. |
| `--region` | string | no |  |  | Exact one-based inclusive MyVariant region, such as chr7:55259500-55259600; requires --build. |
| `--review-status` | string | no |  |  | Literal MyVariant clinvar.rcv.review_status phrase; no star conversion. Requires a declared protein or region selector. |
| `--source` | string | no |  | `myvariant` | Explicit source language; required for a raw query. |

## Sources

`--source` accepts these sources: [myvariant](/sources/myvariant/).

## Examples

```sh
biomcp search variant --gene EGFR --protein L858R --build GRCh37
biomcp search variant --gene CHEK2 --protein Arg117Gly --build GRCh37
```

## Reply schema

The help examples of this command return [`biomcp.variant_page/6`](/reference/schemas/biomcp-variant-page-6/). Print the full JSON schema with `biomcp schema biomcp.variant_page/6`.

## Exit codes

See [exit codes](/reference/error-codes/). Options that apply to every command are in [global options](/reference/global-options/).

## Next

The [variant entity](/user-guide/variant/) page lists every variant command and links to a worked guide.

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
