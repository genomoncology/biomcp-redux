# 0938 — markdown-list-values-stay-text

Status: OPEN.

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
