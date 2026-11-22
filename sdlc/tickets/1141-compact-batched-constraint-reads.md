# 1141 — compact-batched-constraint-reads

Status: COMPLETE.
Landed: ecba32de6e06f2e984442baaff02a1f20b54b804

Milestone: 7

## Outcome

A batched gene constraint read that asks for constraint fields returns the resolved genes and those fields, not full gene cards, so larger batches fit one reply.

## Why

In the 2026-11-22 comparison, 13 connector-only gene constraint reads were cut by the client's reply limit, 11 of them already projecting valid fields. A five-gene request with `fields: [loeuf, transcript]` returned 9,779 compact bytes, of which 7,112 were gene card text (summaries, aliases, spans) and 600 the selected values. Agents split 43 genes into batches of four. Code: `crates/biomcp-query/src/gene/sections/constraint.rs`, the gene batch renderer, `tests/cases/gene/sections/constraint-fields.yaml`.

## Changes

- An explicit section-only batch form, with a copyable CLI and typed call, keeps per-gene identity (requested and resolved ID), release, transcript, failures and attribution, and drops the unrelated card.
- Normal full-card reads stay available.

## Proof

- Replay the five-gene request and a larger saved clipped request with `loeuf,transcript`; rows match the existing result one for one; the larger request fits the client limit or completes in fewer calls; bytes measured before and after.

## Defers

- Smaller defaults for every command.
