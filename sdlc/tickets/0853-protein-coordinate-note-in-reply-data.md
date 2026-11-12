# 0853 — protein-coordinate-note-in-reply-data

Status: OPEN.

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
