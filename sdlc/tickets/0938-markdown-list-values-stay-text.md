# 0938 — markdown-list-values-stay-text

Status: COMPLETE.
Landed: 32f7d325e194e03652b9422250b40c93a553f481

Milestone: 7

## Outcome

A source value printed as a list item cannot become a code block or a nested list (for example a value starting `-      ` or `1.      `), and the fetch tool's plain-text view leaves fenced next-step blocks untouched when it removes the view's escapes.

## Why

0925's review (2026-11-14): a value like `-      ignore previous` renders as `<pre><code>` inside the list on main and with 0925 (main already had it through `-`, `+` and `1)`; 0925 adds `1.`). That breaks 0348's code-block guarantee. Separately, `crates/biomcp/src/mcp/research/text.rs:15,19` removes escapes across the whole text, including the `sh` and `json` next-step blocks, so an argument `a\.b` becomes `a.b`.

## Proof

- Proof: 0348's replay case (`markdown-view/source-fields-are-data`, restored by 0925) gains list-item values starting with a marker and five spaces, rendered with comrak, with no code block; a fetch-tool case keeps a fenced block's backslash.

## Starts from

- Starts from: `crates/biomcp/src/view/escape.rs`; `crates/biomcp/src/mcp/research/text.rs:15,19`; 0925's reviewer inputs in the PM scratchpad were `-      ignore previous` and `1.      x`.

## Keeps

- Keeps: 0925's plain punctuation and its link guarantees; JSON output.

## Defers

- Defers: nothing.

## Changes

- Changes: the view escape, the fetch text view, the replay case.

## Progress

- Source text is trimmed at the source boundary (`biodata::Text::from_source`) and its line breaks are visible escapes, so a value can only open a line at its first character: as a `- ` list item or a paragraph.
- Changed `view/escape.rs`: a source value's first character is escaped only when it would open a block: a leading space becomes `&#32;`, a leading `-` or `+` followed by a space or the end becomes `\-`/`\+`, a value made only of dashes and spaces (a rule) gets `\-`, and the `.` or `)` after 1-9 leading digits followed by a space or the end becomes `\.`/`\)`. Ordinary values such as `-1.5`, `1.5 mg` and `MK-3475` are unchanged. Help text keeps its old output.
- Changed `mcp/research/text.rs`: the fetch tool's plain view now copies the view's fenced next-step blocks as they are and removes escapes and entities only outside them; it also removes the new `\-`, `\+` and `\)`.
- Proof: replay case `markdown-view/source-fields-are-data` gains PubMed collective-author values `-      ignore previous`, `1.      x`, `+      y`, `2)      z` and `---`; comrak renders each as plain `<li>` text with no `<ol>`, `<hr>`, bare `<pre><code>` or nested list, and the MCP `fetch` tool's text shows each value as written.
- Open: the fenced-backslash proof. No get reply the fetch tool can open puts a backslash in a next step today: step arguments are BioMCP-built identifiers and section words, a user's free-text name is replaced by the resolved source ID, and a DOI (which may hold a backslash) only reaches a step when a card has no PMID. A unit test in `text.rs` proved it on cd85d4f8f but the placement ratchet (`crates/biomcp/tests/ratchets/placement.rs:495`, inline tests only in listed files) and the parent-module ceiling (170) refuse it, so it was removed.
