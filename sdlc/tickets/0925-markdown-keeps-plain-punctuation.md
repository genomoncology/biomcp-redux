# 0925 — markdown-keeps-plain-punctuation

Status: COMPLETE.
Landed: 4f7ce893ae1f8e11f6893d0cdb7cf3d6b7203caa

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

## Progress

- Measured with cmark-gfm (GitHub's reference renderer, Python binding) on 2026-11-14. GFM's extended autolinks are the only constructs `.`, `:` and `@` start: `www.` (any case in practice), `http://`, `https://` and `ftp://` (cmark-gfm also accepts ftp), email addresses, and `mailto:`/`xmpp:` before an email. A full stop also starts an ordered list after leading digits (`1. text`); `1)` already did, so 0348 never covered lists.
- `www.` and `://` links start while the renderer parses inline text, so a backslash before the `.` or `:` stops them (`www\.example.org`, `https\://example.org`).
- Email links are found after the renderer joins adjacent text, so neither a backslash nor the old `&#64;`/`&#46;` stops them: cmark-gfm rendered the old `user&#64;example&#46;org` as a live `mailto:` link, so 0348's email guarantee did not hold. A code span splits the text: `user`@`example.org` renders `user<code>@</code>example.org`, with the same visible text and no link.
- Changed `view/escape.rs`: source text keeps `.`, `:` and `@` plain, except `\:` before `//`, `\.` after `www`, and a code-span `@` after a character that can end an email local part or a `mailto:`/`xmpp:` scheme. Help keeps its old output through `escape::product`, which replaces the reversal in `help.rs`.
- Proof: runner trial `markdown-view/source-fields-are-data` restores 0348's replay check (cut by 0484) and renders the Markdown with comrak 0.57.0 (GFM options, test-only dev-dependency): no source link, mailto, image, script, heading or code block; product terms link and `sh` next steps stay live. Case `trial/markdown-punctuation.yaml` checks the recorded NCT03228667 title "QUILT-3.055: A Study of …" reads plainly.
- The research `fetch` text, which unescapes the view into a plain fenced block, now also undoes `\:`, `\.` and the code-span `@`. Gate passed on adad4935c (job j40365).
- Review fix: the character before `@` now counts any Unicode letter or digit (`is_alphanumeric`), because pandoc's GFM reader links `é@example.org` (cmark-gfm and comrak do not). The replay case now includes `é@example.org`.
