# 0925 — markdown-keeps-plain-punctuation

Status: OPEN.

Milestone: 7

## Outcome

BioMCP's Markdown output shows source text with its ordinary full stops, colons and @ signs as plain characters, not `&#46;`, `&#58;` and `&#64;`, while source text still can't become a live link, image, heading, code fence or HTML. A trial title such as "Phase 3 Study of MK-3475 vs. Chemotherapy: KEYNOTE-061" reads that way to an agent.

## Why

The UI team, building the BioMCP 1.0 docs demo, reported it through Jarvis (m35978, 2026-11-14): trial output prints punctuation as HTML character codes. The docs site hides them, but agents reading BioMCP directly see them, which costs tokens and can mislead. The cause is `crates/biomcp/src/view/escape.rs:9-11`, from 0348 (source text reaches agents as data). It encodes every `.`, `:` and `@` so that bare web addresses and email addresses in source text can't autolink. Help already reverses those three (`crates/biomcp/src/view/help.rs:110-114`).

## Proof

- Proof: 0348's replay case still shows `<script>`, `<img src=x>`, a Markdown link, a heading, a code fence and the injection words as literal text. A new case with a recorded trial reply asserts a title with a full stop and colon renders plainly. A source field holding `https://example.org` and `name@example.org` renders so that a CommonMark/GFM renderer makes no link (check with the renderer the docs site uses, or a GFM reference renderer, in the test).

## Starts from

- Starts from: `crates/biomcp/src/view/escape.rs`, `crates/biomcp/src/view/help.rs:110-114`, 0348's ticket and its replay case.
- The design choice is the developer's, within 0348's rules: never delete or rewrite source text, escape it, and no text heuristics decide what looks like an instruction. One option is to break only the autolink forms (`://`, a leading `www.`, the `@` in an email) with a Markdown backslash escape or a zero-cost form instead of encoding every character. Check what GFM autolinks actually require before choosing.

## Keeps

- Keeps: JSON output unchanged (source text as plain strings); BioMCP's own links and next steps live; 0348's proof.

## Defers

- Defers: nothing. Jarvis asked that it be fixed before 1.0 if small; 1.0 itself stays parked (0294).

## Changes

- Changes: `crates/biomcp/src/view/escape.rs`, `help.rs` if its reversal becomes unnecessary, cases, generated docs.
