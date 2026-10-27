---
title: biomcp search variant
description: "The BioMCP 1.0.0 command biomcp search variant: Finds variants by
  gene, protein change, region or another single selector."
goal: Look up the arguments, sections and examples of biomcp search variant.
reader: A BioMCP user or agent checking one command
sidebar:
  label: search variant
kind: reference
verified: 2026-10-27
---

`biomcp search variant` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It finds variants by gene, protein change, region or another single selector. This page lists its arguments, examples and reply.

The program's help describes it:

> Discover source candidates using one declared selector, without interpreting free text.

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

`--source` picks the source to ask: [myvariant](/sources/myvariant/).

## Examples

The program's help gives these examples.

```sh
biomcp search variant --gene EGFR --protein L858R --build GRCh37
```

```sh
biomcp search variant --gene CHEK2 --protein Arg117Gly --build GRCh37
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.variant_page/6`](/reference/schemas/biomcp-variant-page-6/), whose page shows a real reply and its fields.

## Next

The [variant guide](/user-guide/variant/) shows every variant command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
