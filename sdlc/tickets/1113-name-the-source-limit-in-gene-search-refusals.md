# 1113 — name-the-source-limit-in-gene-search-refusals

Status: OPEN.

Milestone: 7

## Outcome

A gene search refusal names `--source`, says which lookup the command performs and points callers to a working supported route for the chosen curation source.

## Why

Found by a hand solve of research tasks, 2026-11-20. `biomcp search gene 'Fanconi anemia' --source clingen` returned `unknown option: this command has no error.option`. `biomcp search gene 'Fanconi anemia' --source gencc` gives the same message.

## Proof

- Proof: Replay both source choices and assert that the reply names the rejected flag, describes the supported lookup and offers a command that passes validation.

## Starts from

- Starts from: The gene search request checks and shared usage wording from ticket 1064.

## Keeps

- Keeps: The default HGNC name and alias search remains available.

## Defers

- Defers: Disease-first curation lookup has its own ticket.

## Changes

- Changes: Gene search validation, guidance and cases.
