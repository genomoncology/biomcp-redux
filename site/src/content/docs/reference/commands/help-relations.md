---
title: biomcp help relations
description: "The BioMCP 1.0.0 command biomcp help relations: Follow source
  relationships through a named get section or a search filter."
goal: Look up the arguments, sections and examples of biomcp help relations.
reader: A BioMCP user or agent checking one command
sidebar:
  label: help relations
kind: reference
verified: 2026-11-09
---

`biomcp help relations` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It follow source relationships through a named get section or a search filter. This page lists its reply.

The program's help describes it:

> Follow source relationships through a named get section or a search filter. These links retrieve source records without deciding their clinical meaning.
> adverse-event -&gt; drug (get section drugs)
> author -&gt; article (search --author)
> biomarker -&gt; diagnostic (search --biomarker)
> cell-line -&gt; variant (get section variants)
> diagnostic -&gt; gene (get section genes)
> disease -&gt; article (search --disease), cell-line (search --disease), gene (get section genes), phenotype (get section phenotypes)
> drug -&gt; adverse-event (search --drug), article (search --drug)
> gene -&gt; article (search --gene), cell-line (search --gene), diagnostic (search --gene), diagnostic (search --gene-id), gwas (search --gene), pathway (get section pathways), pathway (search --gene), pgx (search --gene), protein (get section protein), protein (search --gene), trial (search --gene), variant (search --gene)
> pathway -&gt; gene (get section genes)
> phenotype -&gt; disease (get section diseases), gene (get section genes)
> protein -&gt; cell-line (search --protein), variant (search --protein)
> study -&gt; gwas (search --study)
> trial -&gt; disease (get section diseases)
> variant -&gt; article (search --variant), gwas (get section gwas), gwas (search --variant), trial (search --variant)

## Usage

```text
biomcp help relations
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.help/6`](/reference/schemas/biomcp-help-6/), whose page lists its fields.

## Next

The [commands index](/reference/commands/) lists every command.
