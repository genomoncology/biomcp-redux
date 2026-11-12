# 0853 — protein-coordinate-note-in-reply-data

Status: COMPLETE.
Landed: b59f891f0d720a4a3354d2e9a80d7ec4c466200f

Milestone: 7

## Outcome

Protein replies that carry coordinate ranges hold the note on how to read them (where ranges start and end, and how that differs from UniProt's counting) inside the reply's structured data, so every MCP client sees it in the default single-copy reply.

## Why

0837 made local MCP servers send each reply once, in the structured field only, which drops the text note 0811 promised; Claude Code never showed it and pi and ChatGPT users on local servers now lose it.

## Proof

- Proof: a recorded protein case with coordinate ranges (features or domains) shows the note in the structured reply under the default single-copy mode, and the reply-size test from 0837 still passes.

## Starts from

- Starts from: 0837's review finding (accepted at e5a78256c), 0811's coordinate note, the protein reply schemas in `crates/biomcp-query/src/protein/`.
- PM decision: 2026-11-12. Put the note in the data (reviewer's option 2) rather than accept the narrower behaviour or send both copies for protein replies: every client sees it, at the cost of a schema change.

## Keeps

- Keeps: 0837's one-copy default and size savings; the schema ID rule (a changed reply schema takes a new major); hosted servers' behaviour.

## Defers

- Defers: nothing; other entities' text-only notes, if any, are listed in the review and ticketed separately.

## Changes

- Changes: `crates/biomcp-query/src/protein/`, protein reply schemas, `tests/cases/protein/`

## Progress

- 2026-11-12, developer: the note is now a `coordinate_note` string in the reply data of protein features, protein domains and protein cards, present only when the reply holds a span (a card states it once at the top, not again inside its domains section). Schema majors: `biomcp.protein-features/5`, `biomcp.protein-domains/8`, `biomcp.protein-card/11`. The MCP text note and its special case in reply building are gone; every reply form now carries the note inside the JSON. Markdown still leads with the note as prose, read from the same field.
- Other text-only notes 0837 drops: none found. Before 0837, reply text added only the protein coordinate note and the typed status/next summary line; status and next are already in the JSON, and 0837 moved the typed correction into `next`. Markdown-only lines (no data, failed source with reason, PubMed and LitVar notices) are rendered from fields already in the JSON, so MCP clients see the same facts.
- Gate j38428 passed on 198a53113 (`make --keep-going landing-gate docs-check schema-release-check`). The recorded case `protein/native-domains` now shows the note in the default single-copy MCP reply. 0837's reply-size test still passes: that case grows from 2,000 to 2,317 wire bytes (the note once, 317 bytes), with no text copy, and the ten-case total is 27,309 bytes against 26,992.
