---
title: Command map from 0.9 to 1.0
description: "Every BioMCP 0.9 command and what BioMCP 1.0.0 does with it:
  ported or renamed, dropped, or deferred, with the options it drops."
goal: Find what happened to one 0.9 command or option in BioMCP 1.0.
reader: A BioMCP 0.9 user moving scripts or agents to 1.0
kind: reference
verified: 2026-10-27
---

This map lists 110 BioMCP 0.9 commands and their outcome in 1.0.0: 67 ported or renamed, 38 dropped by ruling, 5 deferred by ruling. [Upgrading from 0.9](/upgrade-from-0-9/) explains the main changes.

| 0.9 command | Outcome in 1.0 | Dropped options | Deferred options |
| --- | --- | --- | --- |
| `search adverse-event` | Ported or renamed | `--drug`, `--outcome`, `--suspect-only`, `--sex`, `--age-min`, `--age-max`, `--reporter`, `--count`, `--classification`, `--manufacturer`, `--product-code` | — |
| `get adverse-event` | Ported or renamed | — | — |
| `drug adverse-events` | Dropped by ruling | `--reaction`, `--outcome`, `--serious`, `--date-from`, `--date-to`, `--suspect-only`, `--sex`, `--age-min`, `--age-max`, `--reporter`, `--count`, `--type`, `--limit`, `--offset` | — |
| `search article` | Ported or renamed | `--ranking-mode`, `--weight-semantic`, `--weight-lexical`, `--weight-citations`, `--weight-position`, `--max-per-source`, `--session`, `--debug-plan`, `--full` | — |
| `get article` | Ported or renamed | `--asset-view`, `--asset-limit`, `--asset-offset` | — |
| `article authors` | Ported or renamed | — | — |
| `article entities` | Ported or renamed | `--limit`, `--full` | — |
| `article batch` | Ported or renamed | — | — |
| `article citations` | Ported or renamed | — | — |
| `article references` | Ported or renamed | — | — |
| `article citation-evidence` | Ported or renamed | — | — |
| `article supplement-search` | Ported or renamed | — | — |
| `article recommendations` | Ported or renamed | — | — |
| `variant articles` | Ported or renamed | `--input`, `--debug-plan`, `--verify-identity`, `--confirmed-only`, `--strategy` | — |
| `disease articles` | Ported or renamed | — | — |
| `search author` | Ported or renamed | — | — |
| `get author` | Ported or renamed | — | — |
| `author papers` | Ported or renamed | — | — |
| `search cell-line` | Ported or renamed | — | — |
| `get cell-line` | Ported or renamed | — | — |
| `cell-line drug-response` | Deferred by ruling | — | `--dataset`, `--limit`, `--offset` |
| `cache clean` | Ported or renamed | `--dry-run`, `--max-age`, `--max-size` | — |
| `chart bar` | Dropped by ruling | — | — |
| `chart box` | Dropped by ruling | — | — |
| `chart density` | Dropped by ruling | — | — |
| `chart heatmap` | Dropped by ruling | — | — |
| `chart histogram` | Dropped by ruling | — | — |
| `chart pie` | Dropped by ruling | — | — |
| `chart ridgeline` | Dropped by ruling | — | — |
| `chart scatter` | Dropped by ruling | — | — |
| `chart stacked-bar` | Dropped by ruling | — | — |
| `chart survival` | Dropped by ruling | — | — |
| `chart violin` | Dropped by ruling | — | — |
| `chart waterfall` | Dropped by ruling | — | — |
| `discover` | Dropped by ruling | `--full`, `--limit`, `--offset`, `--search` | — |
| `gencc sync` | Deferred by ruling | — | — |
| `serve-http` | Ported or renamed | `--unsafe-allow-any-host` | — |
| `skill install` | Ported or renamed | — | — |
| `skill list` | Ported or renamed | — | — |
| `skill render` | Ported or renamed | — | — |
| `skill status` | Ported or renamed | — | — |
| `list` | Dropped by ruling | — | — |
| `search all` | Dropped by ruling | `--counts-only`, `--debug-plan`, `--disease`, `--drug`, `--gene`, `--keyword`, `--limit`, `--since`, `--variant` | — |
| `cache clear` | Dropped by ruling | `--yes` | — |
| `cache path` | Dropped by ruling | — | — |
| `cache stats` | Dropped by ruling | — | — |
| `health` | Ported or renamed | `--api`, `--apis-only`, `--fail-on-error` | — |
| `mcp tools` | Dropped by ruling | — | — |
| `mcp-config` | Ported or renamed | — | — |
| `version` | Ported or renamed | — | — |
| `update` | Dropped by ruling | `--check` | — |
| `uninstall` | Dropped by ruling | — | — |
| `search diagnostic` | Ported or renamed | `--disease`, `--full` | — |
| `get diagnostic` | Ported or renamed | — | — |
| `gtr sync` | Ported or renamed | — | — |
| `who-ivd sync` | Ported or renamed | — | — |
| `search disease` | Ported or renamed | `--query`, `--inheritance`, `--phenotype`, `--onset`, `--no-fallback` | — |
| `get disease` | Ported or renamed | `--name` | — |
| `disease drugs` | Dropped by ruling | `--limit`, `--offset` | — |
| `search drug` | Ported or renamed | `--mechanism`, `--interactions`, `--product-type` | `--target`, `--indication`, `--type`, `--atc`, `--pharm-class` |
| `get drug` | Ported or renamed | — | — |
| `drug interactions` | Ported or renamed | — | — |
| `drug cell-lines` | Dropped by ruling | `--cell-line`, `--dataset`, `--limit`, `--offset` | — |
| `ema sync` | Ported or renamed | — | — |
| `who sync` | Ported or renamed | — | — |
| `ddinter sync` | Ported or renamed | — | — |
| `cvx sync` | Dropped by ruling | — | — |
| `search gene` | Ported or renamed | `--query`, `--region`, `--pathway`, `--go` | — |
| `get gene` | Ported or renamed | — | — |
| `gene definition` | Ported or renamed | — | — |
| `gene trials` | Ported or renamed | — | — |
| `gene drugs` | Dropped by ruling | `--limit`, `--offset` | — |
| `gene articles` | Ported or renamed | — | — |
| `gene pathways` | Ported or renamed | — | — |
| `gene cell-lines` | Deferred by ruling | — | `--group`, `--limit`, `--offset` |
| `search gwas` | Ported or renamed | — | — |
| `search pathway` | Ported or renamed | `--query`, `--type` | — |
| `get pathway` | Ported or renamed | — | — |
| `pathway drugs` | Dropped by ruling | `--limit`, `--offset` | — |
| `pathway articles` | Dropped by ruling | `--limit`, `--offset` | — |
| `pathway trials` | Dropped by ruling | `--limit`, `--offset`, `--source` | — |
| `enrich` | Dropped by ruling | `--limit` | — |
| `search pgx` | Ported or renamed | `--drug`, `--evidence` | `--pgx-testing` |
| `get pgx` | Ported or renamed | `--full` | — |
| `search phenotype` | Dropped by ruling | `--limit`, `--offset` | — |
| `search protein` | Ported or renamed | `--all-species` | `--include-unreviewed`, `--disease`, `--existence` |
| `get protein` | Ported or renamed | — | — |
| `protein structures` | Ported or renamed | — | — |
| `study list` | Ported or renamed | — | — |
| `study download` | Ported or renamed | `--list` | — |
| `study query` | Dropped by ruling | `--study`, `--gene`, `--type`, `--chart`, `--terminal`, `--output`, `--title`, `--theme`, `--palette`, `--cols`, `--rows`, `--width`, `--height`, `--scale`, `--mcp-inline` | — |
| `study top-mutated` | Dropped by ruling | `--study`, `--limit` | — |
| `study filter` | Dropped by ruling | `--study`, `--mutated`, `--amplified`, `--deleted`, `--expression-above`, `--expression-below`, `--cancer-type` | — |
| `study cohort` | Dropped by ruling | `--study`, `--gene` | — |
| `study survival` | Dropped by ruling | `--study`, `--gene`, `--endpoint`, `--chart`, `--terminal`, `--output`, `--title`, `--theme`, `--palette`, `--cols`, `--rows`, `--width`, `--height`, `--scale`, `--mcp-inline` | — |
| `study compare` | Dropped by ruling | `--study`, `--gene`, `--type`, `--target`, `--chart`, `--terminal`, `--output`, `--title`, `--theme`, `--palette`, `--cols`, `--rows`, `--width`, `--height`, `--scale`, `--mcp-inline` | — |
| `study co-occurrence` | Dropped by ruling | `--study`, `--genes`, `--chart`, `--terminal`, `--output`, `--title`, `--theme`, `--palette`, `--cols`, `--rows`, `--width`, `--height`, `--scale`, `--mcp-inline` | — |
| `search trial` | Ported or renamed | `--no-alias-expand`, `--prior-therapies`, `--progression-on`, `--line-of-therapy` | — |
| `get trial` | Ported or renamed | — | — |
| `batch` | Ported or renamed | `--mode`, `--pdf` | — |
| `variant trials` | Ported or renamed | — | — |
| `disease trials` | Ported or renamed | — | — |
| `drug trials` | Ported or renamed | `--no-alias-expand` | — |
| `search variant` | Ported or renamed | — | `--significance`, `--missing`, `--max-frequency`, `--min-cadd`, `--consequence`, `--population`, `--revel-min`, `--gerp-min`, `--tumor-site`, `--impact`, `--lof`, `--therapy` |
| `get variant` | Ported or renamed | — | — |
| `variant structure` | Deferred by ruling | — | — |
| `variant oncokb` | Ported or renamed | — | — |
| `variant erepo` | Ported or renamed | `--gene` | — |
| `variant normalize` | Ported or renamed | — | — |
| `gene cspec document` | Deferred by ruling | — | — |
