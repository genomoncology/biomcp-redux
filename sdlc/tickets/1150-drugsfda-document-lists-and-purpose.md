# 1150 — drugsfda-document-lists-and-purpose

Status: OPEN.

Milestone: 7

## Outcome

Drugs@FDA approval-document reads either accept several document IDs or refuse a list clearly, and report a supplement's purpose when the letter states it.

## Why

Found 2026-11-22 by scouts exploring BioMCP for multi-entity questions (about 760 CLI calls on build b7def4651). `biomcp get drug BLA761363 approvals --source drugsfda --document-id 83960,84364,85547,86877` returned `partial` and `not-found`, echoing the whole comma-separated value as one ID, which looks like a source miss. `get drug NDA215256 approvals --source drugsfda --document-id 83118` returned `supplement_purpose: not-stated` while the letter it returned states the purpose; the same happened for a second product's letters.

## Changes

- A comma list of document IDs is read as several documents, each with its own status, or refused as a list with the per-ID call.
- When the parsed purpose is absent but the letter text states it, the reply says the purpose is in the letter rather than `not-stated`.

## Proof

- Replays: a four-ID list returns four documents (or a clear list refusal); a letter stating its purpose is not reported as `not-stated`.

## Amendment 2026-11-22

Found by the developer of ticket 1140: when a supplement letter is unavailable and the caller selects only `submission_number`, the reply's incomplete-data note points to `supplement_purpose`, a field the caller left out. A note about an unselected field should name what is missing in words the caller can act on, or move to the selected field.
