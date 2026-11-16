# 0998 — small-fixes-from-the-adversarial-review

Status: OPEN.

Milestone: 7

## Outcome

The small accepted findings of `sdlc/records/2026-11-14-adversarial-review.md` that were still open on main 3f14da1a6 are fixed, and each choice the code makes on purpose has its reason written beside it.

## Why

PM triage, 2026-11-16, against main 3f14da1a6:

- **A4:** `drug/response.rs:340`'s `total` says "exact source records" but counts examined candidates.
- **A6:** `trial.rs:223` overwrites `outcome.dropped` with `=`. It should combine the flags.
- **C2:** plain HTTP downloads a required PDF or document, then refuses the save (`command/mcp_policy.rs:17-28`, `save/reply.rs:73`). It should refuse before any publisher request.
- **C4:** the 429 comment at `request.rs:374` says the body is never read, but a keyed body is read.
- **D6:** generated help says "native" and "literal", but the details list says "source" and "exact" (`docs/reference/commands.md:958` vs `:1094`).
- **E3:** ClinVar gene counts have a fixed 15 s limit with no `command_budget` (`clinvar/mod.rs:40`), unlike VAERS and Monarch.
- **F3:** `winnow` in `Cargo.toml:46` is used by no crate.
- **F4:** `gene::in_region` (`query/gene.rs:152`) has no caller.
- **F7:** the openFDA terms URL is written out in four SPECs.
- **Reasons to write beside the code:**
  - A2: FDA name lookups read every page on purpose, because the ranking needs the whole set.
  - B4: the local cache upgrades its own tables in place.
  - F9: PMA and 510(k) dates stay raw text while recall dates are parsed.

## Proof

- Proof: C2 is a routine case showing plain-HTTP `article pdf` refused before any publisher request. It fails on the base commit.
- Proof: A6 is a unit test where an earlier `dropped` flag survives. It fails on the base commit.
- Proof: E3 gets a declared budget, like VAERS and Monarch, and a test reads it.
- Proof: the gate passes. Changed schema descriptions follow the 1.0 rules. The release notes get a line for anything users see (C2, D6, E3).

## Starts from

- Starts from: the review's finding text for each ID.

## Keeps

- Keeps: no behaviour change beyond C2's earlier refusal and E3's longer limit.

## Defers

- Defers: F10, the comments that cite ticket numbers. That is a separate ruling.

## Changes

- Changes: the files named above, the generated docs and schemas, the notes.
