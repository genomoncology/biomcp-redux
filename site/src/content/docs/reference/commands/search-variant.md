---
title: biomcp search variant
description: "The BioMCP 1.0.0 command biomcp search variant: Finds variants by
  gene, protein change, region or another single selector."
goal: Look up the arguments, sections and examples of biomcp search variant.
reader: A BioMCP user or agent checking one command
sidebar:
  label: search variant
kind: reference
verified: 2026-11-09
---

`biomcp search variant` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It finds variants by gene, protein change, region or another single selector. This page lists its arguments, examples and reply.

The program's help describes it:

> Search genetic variants by gene, change or region. Sources: myvariant.

## Usage

```text
biomcp search variant [OPTIONS]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--build` | string | no |  | `GRCh37`, `GRCh38` | Optional caller assembly sent to MyVariant; regions; help search variant Optional caller assembly sent to MyVariant; regions require it explicitly. Listed choices accept any letter case; replies use their listed spelling. Value form: string. Example: --build GRCh37. Command sources: myvariant; source restrictions in the field description apply. |
| `--coding` | string | no |  |  | Coding HGVS change searched literally in ClinVar with --gene, without choosing a transcript. Value form: string. Example: --coding 'c.333G&gt;T'. Command sources: myvariant; source restrictions in the field description apply. |
| `--condition` | string | no |  |  | Exact MyVariant clinvar.rcv.conditions.name phrase; source text is not a local diagnosis. Requires a declared protein or region selector. Value form: string. Example: --condition 'Carcinoma of colon (CRC)'. Command sources: myvariant; source restrictions in the field description apply. |
| `--gene` | string | no |  |  | Checked gene symbol for the dbNSFP substitution or ClinVar coding/synonymous selector. Value form: string. Example: --gene EGFR. Command sources: myvariant; source restrictions in the field description apply. |
| `--has` | string | no |  | `clinvar` | Require the selected component's identifier field. Only; help search variant Require the selected component's identifier field. Only source-proven mappings are offered. Listed choices accept any letter case; replies use their listed spelling. Value form: string. Example: --has clinvar. Command sources: myvariant; source restrictions in the field description apply. |
| `--limit` | integer | no | min 1, max 50 |  | Source candidates per page, default 10, maximum 50; all candidates remain candidates. Value form: integer. Example: --limit 50. Command sources: myvariant; source restrictions in the field description apply. |
| `--offset` | integer | no | min 0, max 9999 |  | Source offset within this command's bounded result window; larger source totals remain visible. Value form: integer. Example: --offset 0. Command sources: myvariant; source restrictions in the field description apply. |
| `--protein` | string | no |  |  | Explicit substitution or synonymous change using checked amino-acid codes. Value form: string. Example: --protein L858R. Command sources: myvariant; source restrictions in the field description apply. |
| `--query` | string | no |  |  | Raw MyVariant query, passed and echoed unchanged when component policy allows; requires --source myvariant. With disabled components use declared selectors instead. Value form: string. Example: --query CA123643. Command sources: myvariant; source restrictions in the field description apply. |
| `--region` | string | no |  |  | Exact one-based inclusive MyVariant region, such as chr7:55259500-55259600; requires --build. Value form: string. Example: --region chr7:140453136-140453136. Command sources: myvariant; source restrictions in the field description apply. |
| `--review-status` | string | no |  |  | Exact MyVariant clinvar.rcv.review_status phrase; no star conversion. Requires a declared protein or region selector. Value form: string. Example: --review-status 'no assertion criteria provided'. Command sources: myvariant; source restrictions in the field description apply. |
| `--source` | string | no |  | `myvariant` | Explicit source language; required for a raw query; help search variant Explicit source language; required for a raw query. Listed choices accept any letter case; replies use their listed spelling. Value form: string. Example: --source myvariant. Command sources: myvariant; source restrictions in the field description apply. |

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

```sh
biomcp search variant --region chr7:140453136-140453136 --build GRCh37 \
  --condition 'Carcinoma of colon (CRC)'
```

```sh
biomcp search variant --region chr7:140453136-140453136 --build GRCh37 \
  --review-status 'no assertion criteria provided'
```

```sh
biomcp search variant --query CA123643 --source myvariant
```

```sh
biomcp search variant --gene PTEN --coding 'c.333G>T' --limit 50
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.variant-page/16`](/reference/schemas/biomcp-variant-page-16/), whose page lists its fields.

## Next

The [variant guide](/user-guide/variant/) shows every variant command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
