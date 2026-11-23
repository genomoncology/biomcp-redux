# 1167 — gtr-genes-conditions-and-versioned-url

Status: COMPLETE.
Landed: 4b4736a4e6749565559be5218f242c2d0e0b56cc

Milestone: 7

## Outcome

`get diagnostic GTR... --source gtr` with `genes` or `conditions` returns the genes and conditions NCBI's registry lists for that test version, and the record's URL keeps the version. When BioMCP's data holds none, the reply says the section is missing or incomplete, never `complete` with an empty list.

## Why

Found 2026-11-23 while verifying the docs workflow "Registered genetic tests" (hosting ticket 0119). After `biomcp sync gtr`, `biomcp get diagnostic GTR000527948.3 --source gtr` named version 3 but returned an unversioned URL, and adding `genes` or `conditions` returned `status: complete` with empty lists. NCBI's versioned record (https://www.ncbi.nlm.nih.gov/gtr/tests/527948.3/) lists 70 genes and the condition "Solid tumor". A complete-but-empty reply is a wrong answer an agent will repeat.

## Proof

- Proof: a replay of the synced GTR export for GTR000527948.3 returns 70 genes and "Solid tumor", and the URL ends in `/527948.3/`; a test whose export row truly lacks genes returns an incomplete or missing marker, not `complete` with `[]`.

## Changes

- Changes: the GTR adapter's sync parsing and section readers, and GTR replay cases.
