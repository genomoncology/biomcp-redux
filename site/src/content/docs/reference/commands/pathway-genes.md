---
title: biomcp pathway genes
description: "The BioMCP 1.0.0 command biomcp pathway genes: Lists the genes and
  gene products of one pathway."
goal: Look up the arguments, sections and examples of biomcp pathway genes.
reader: A BioMCP user or agent checking one command
sidebar:
  label: pathway genes
kind: reference
verified: 2026-11-09
---

`biomcp pathway genes` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It lists the genes and gene products of one pathway. This page lists its arguments, examples and reply.

The program's help describes it:

> List genes in a biological pathway. Sources: reactome, wikipathways.

## Usage

```text
biomcp pathway genes [OPTIONS] <id> [details]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--limit` | integer | no | min 1, max 50 |  | Five complete relation rows by default, at most fifty. Value form: integer. Example: --limit 5. Command sources: reactome, wikipathways; source restrictions in the field description apply. |
| `--offset` | integer | no | min 0 |  | Offset within the retained selected relation. Value form: integer. Example: --offset 0. Command sources: reactome, wikipathways; source restrictions in the field description apply. |
| `--pool` | string | no |  |  | Immutable relation snapshot. Value form: string. Example: --pool &lt;pool&gt; (syntax; replace the placeholder with your retained value). Command sources: reactome, wikipathways; source restrictions in the field description apply. |
| `id` (position 1) | string | yes |  |  | Checked source-owned human pathway identity. |
| `details` (position 2) | string | no |  | `details` | Include every physical entity context for these references;; help pathway genes Include every physical entity context for these references; never clips source text. Listed choices accept any letter case; replies use their listed spelling. |

## Examples

The program's help gives these examples.

```sh
biomcp pathway genes R-HSA-5673001
```

```sh
biomcp pathway genes R-HSA-5673001 details
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.pathway-genes/9`](/reference/schemas/biomcp-pathway-genes-9/), whose page lists its fields.

## Next

The [pathway guide](/user-guide/pathway/) shows every pathway command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
