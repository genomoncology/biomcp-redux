# 0983 — saved-notes-and-limits

Status: OPEN.

Milestone: 9

## Outcome

A user attaches a note to a project, list, search or list item with `saved note TARGET TEXT`; notes come back labelled as user data, and the saved file enforces the design's limits (name 80 characters, note 2,000 characters).

## Why

Step 4 of the saved-work design's Order (`sdlc/planning/saved-work.md`, Records: `biomcp.saved-note/1`, Privacy).

## Proof

- Proof: routine cases add, show, export and delete notes; a repeated note write changes nothing (ID is a hash of target and text); over-limit names and notes are refused with a usage error.

## Starts from

- Starts from: 0831's store and commands; the design's Records and Privacy sections.

## Keeps

- Keeps: 0831's compatibility rules; notes never read as instructions; no source content stored.

## Defers

- Defers: nothing.

## Changes

- Changes: the saved-work code and its cases.

## Progress
