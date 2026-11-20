# 1117 — name-the-source-limit-in-pathway-get-refusals

Status: OPEN.

Milestone: 7

## Outcome

When a Reactome stable ID makes the source implicit, a pathway get refusal names the redundant `--source` flag and offers the working ID lookup.

## Why

Found by a hand solve of research tasks, 2026-11-20. `biomcp get pathway R-HSA-5693579 --source reactome` returned `unknown option: this command has no error.option`. The requested pathway resolves from its stable ID alone.

## Proof

- Proof: Replay the refusal and run its suggested call against the saved pathway reply.

## Starts from

- Starts from: `crates/biomcp-query/src/pathway/get.rs` and pathway get input checks.

## Keeps

- Keeps: The stable ID continues to determine the source.

## Defers

- Defers: New pathway count and release fields have a separate ticket.

## Changes

- Changes: Pathway get validation, guidance and cases.
