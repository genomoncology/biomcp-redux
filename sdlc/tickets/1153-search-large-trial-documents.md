# 1153 — search-large-trial-documents

Status: COMPLETE.
Landed: 7cac19fefa2a1f5e126f18b6b0d784e8e77664fc

Milestone: 7

## Outcome

A caller over MCP can find a phrase in, and read parts of, a large posted trial document, such as a 335-page statistical analysis plan, or gets a reply that names a call that works.

## Why

Found 2026-11-23 in the round 2 pilot (batch pilot-b-o-sol, case trials-2). Sol 6.2 over MCP called `trial document NCT05104866 SAP_001.pdf --find race` and `--section-offset 0`, with the plan and the protocol. Each reply was `status: partial` with `text.state: too-large`, `pages: 335`, `total: 0`, `hits: []`, and no `next`. The 7,713,908-byte SAP trips the PDF reader's whole-document object or expanded-byte limits (`crates/biomcp-adapters/src/pdf/scan.rs`, `OBJECTS` and `LOAD_BYTES`). The chatbot left the question unanswered in both runs. The CLI agents answered it by working around BioMCP. Posted plans of phase 3 trials are often this size.

## Changes

- Phrase search and part reads work on large posted documents within bounded memory and time, for example by reading page by page, and keep the existing safety limits on any single page or object.
- When a document still cannot be read, the reply says why, gives the page count, and names a call that works (a page range, or a save over the CLI).

## Proof

- Replay of the NCT05104866 SAP (or a fixture with its structure): `--find race` over typed MCP and CLI returns matching passages with their pages.
- A document over the remaining limit returns a refusal with a working `next`, never `partial` with no hits and no next.
- Existing PDF safety tests pass unchanged. No safety limit is raised.
