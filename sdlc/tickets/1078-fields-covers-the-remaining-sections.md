# 1078 — fields-covers-the-remaining-sections

Status: COMPLETE.
Landed: f0ef0d9f02272def92f61463034d2d77c4391366

Milestone: 7

## Outcome

`--fields` narrows the rows of the sections ticket 1071 left refused, and the general sweep that guards narrowing is shown to catch a section losing data it was not asked to give up.

## Why

Ticket 1071 switched `--fields` on for every section its schema-driven rule could handle and left these refused with "--fields does not narrow this call yet": adverse-event drugs and reactions; drug markets; gene civic, clinvar, constraint and oncokb; variant population; article tldr, citations and pdf; protein sequence; all cell-line sections; gwas associations; pathway summary. An agent that learns `--fields` will try it on these and pay a refusal each time.

The 1071 review also found that the general sweep, `section-fields/sections/narrowed-rows-validate`, still passes with the contacts fault put back (a second section emptied to its required keys when another section's path was chosen; `crates/biomcp/src/fields/narrow.rs:11`). Only the one case for that exact call guards it.

## Proof

- Proof: for each section above, the same page with and without `--fields` holds the same rows in the same order with only columns dropped, and the narrowed reply validates; or the ticket says why a section cannot be narrowed and its refusal says so.
- Proof: with the one-line fix in `narrow.rs` reverted, the general sweep fails. The developer shows this run.

## Starts from

- Starts from: ticket 1071 as landed (339bcd1f2), its planning file and its reviewer's job j43375.
- PM decision, 2026-11-20: after a narrowed page, a `next` step that moves to a section without `--fields` support drops the option, as today; once this ticket covers that section, it carries it.

## Keeps

- Keeps: a reply without `--fields` is byte-for-byte unchanged. Status, notices, `incomplete`, `absent`, sources and problems are never narrowed away.

## Defers

- Defers: shorter default rows; stating a list's shared values once (ticket 1076).

## Changes

- Changes: the fields module, the sections listed, the sweep, cases and help.
