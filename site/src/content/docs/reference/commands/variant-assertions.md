---
title: biomcp variant assertions
description: "The BioMCP 1.0.0 command biomcp variant assertions: Lists the
  assertions one source, such as ClinVar, makes about a variant, a page at a
  time."
goal: Look up the arguments, sections and examples of biomcp variant assertions.
reader: A BioMCP user or agent checking one command
sidebar:
  label: variant assertions
kind: reference
verified: 2026-11-09
---

`biomcp variant assertions` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It lists the assertions one source, such as ClinVar, makes about a variant, a page at a time. This page lists its arguments, examples and reply.

The program's help describes it:

> Get genetic variant assertions from ClinVar or ClinGen. Sources: clinvar, erepo.

## Usage

```text
biomcp variant assertions [OPTIONS] <input>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--assertion` | string | no |  |  | Source assertion accession or document ID. Value form: string. Example: --assertion PCER000001171. Command sources: clinvar, erepo; source restrictions in the field description apply. |
| `--build` | string | no |  | `GRCh37`, `GRCh38` | Identity lookup assembly; omitted uses the shared; help variant assertions Identity lookup assembly; omitted uses the shared GRCh38 default. Listed choices accept any letter case; replies use their listed spelling. Value form: string. Example: --build GRCh37. Command sources: clinvar, erepo; source restrictions in the field description apply. |
| `--limit` | integer | no | min 1, max 50 |  | Assertions per page; omitted uses the source default. Value form: integer. Example: --limit 1. Command sources: clinvar, erepo; source restrictions in the field description apply. |
| `--offset` | integer | no | min 0 |  | Assertions to skip in source order. Value form: integer. Example: --offset 0. Command sources: clinvar, erepo; source restrictions in the field description apply. |
| `--source` | string | yes |  | `clinvar`, `erepo` | One source assertion source; shared identity; help variant assertions One source assertion source; shared identity conversion precedes source access when needed. Listed choices accept any letter case; replies use their listed spelling. Value form: string. Example: --source clinvar. Command sources: clinvar, erepo; source restrictions in the field description apply. |
| `--version` | string | no |  |  | VCV archive or ERepo document version; RCV/SCV selectors match a returned source version. Value form: string. Example: --version 1.0.0. Command sources: clinvar, erepo; source restrictions in the field description apply. |
| `input` (position 1) | string | yes |  |  | Supported variant ID; resolves one ClinVar Variation ID or ERepo CAid through the shared resolver. |

## Sources

`--source` picks the source to ask: [clinvar](/sources/clinvar/), [erepo](/sources/clingen-erepo/).

## Examples

The program's help gives these examples.

```sh
biomcp variant assertions clinvar:13961 --source clinvar
```

```sh
biomcp variant assertions CA000434 --source erepo
```

```sh
biomcp variant assertions CA000434 --source erepo --assertion PCER000001171 \
  --version 1.0.0
```

```sh
biomcp variant assertions rs113488022 --source clinvar --build GRCh37
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.variant-assertions/8`](/reference/schemas/biomcp-variant-assertions-8/), whose page lists its fields.

## Next

The [variant guide](/user-guide/variant/) shows every variant command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
