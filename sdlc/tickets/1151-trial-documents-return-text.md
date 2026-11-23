# 1151 — trial-documents-return-text

Status: OPEN.

Milestone: 7

## Outcome

A caller over MCP can read the text of a trial's posted protocol or statistical analysis plan, by part or by phrase, without a shell.

## Why

Found 2026-11-22 in a blind review of round 2 scouting questions. Over MCP stdio, `trial document NCT04191499 SAP_001.pdf` required a save destination and then returned only a local path, byte count and digest for the 919,413-byte PDF, with no text. Six of eleven scouted questions needed a trial's posted plan (a stopping boundary, an event target, a safety analysis set); CLI agents extracted the saved PDF with other tools, and a connector-only agent cannot. BioMCP already reads FDA letter PDFs as text.

## Changes

- `trial document` returns text parts (and `--find TERM` matches with their page) as it does for approval letters, in CLI and typed MCP; saving the file stays available.
- The reply names the document's date and version from the registry.

## Proof

- Replay of a posted SAP: a `--find` call returns the matching passage with its page over typed MCP and CLI; a part read returns text; the save path still works.
