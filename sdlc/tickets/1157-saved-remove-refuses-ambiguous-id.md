# 1157 — saved-remove-refuses-ambiguous-id

Status: COMPLETE.
Landed: a2ada00dd0b6452259599ea6519422baa3573b2a (with 1155)

Milestone: 7

## Outcome

Removing a saved ID that matches records of more than one entity removes nothing and asks for the entity; a qualified removal removes only that record and its note.

## Why

Found 2026-11-23 in the BioMCP 1.0 adversarial review (Astra, snapshot cf9a822889ce), reproduced by the reviewer with ordinary CLI commands. After adding `article 12345` and `gene 12345` to one list with a note each, `saved list remove review 12345` exited 0 and the next export held neither record nor note. Both deletion queries match on the bare ID (`crates/biomcp-query/src/store/saved/store.rs:238`, `notes.rs:107`), though the note command already detects the ambiguity.

## Proof

- Outside-in CLI and typed MCP cases: an unqualified ambiguous removal keeps both records and notes and returns a usage reply naming the qualifiers; a qualified removal keeps the other record and its note.

## Changes

- The saved-list CLI and typed MCP arguments and the two deletion predicates. No database migration.
