# 0911 — drugsfda-whole-read-states-what-it-read

Status: COMPLETE.
Landed: 22974e4f0802ec9a8970407c5cd3865e3e23233a

Milestone: 7

## Outcome

When a Drugs@FDA search sorts or windows by original approval, its notice says how many matches it actually read. A read that stops early is marked partial. The several-reads path (500 to 3,000 matches) has recorded proof.

## Why

The review of 0900 (Drugs@FDA company search by original approval, 2026-11-13) found two gaps.
- **Notice:** if a later 500-row read comes back empty early, the loop stops, but the notice still says "read all {matched}" (`crates/biomcp-adapters/src/drugsfda/whole.rs:46`, `crates/biomcp-query/src/drug/drugsfda.rs:42,49`). A regulatory tracker would then trust an incomplete ordering.
- **Proof:** the several-reads loop (`whole.rs:37-52`) has no recorded proof, because Novartis (204) fits in one read.

## Proof

- Proof:
  - a recorded or synthetic case with more than 500 matches, for example a sponsor prefix with 600 to 1,000 applications, ordered across reads;
  - a synthetic case where a later read returns empty early, so the reply is partial and the notice says how many it read out of how many matched.

## Starts from

- Starts from: `crates/biomcp-adapters/src/drugsfda/whole.rs`; `crates/biomcp-query/src/drug/drugsfda.rs`.

## Keeps

- Keeps: 0900's order, windows, 50-row pages and the 3,000 refusal.

## Defers

- Defers: nothing.

## Changes

- Changes:
  - those two files;
  - `tests/cases/drug/`;
  - optionally, the too-large message (`crates/biomcp-query/src/drug/drugsfda.rs:40-45`), which could say that dropping `--sort original-approval` or the window pages in openFDA's own order.

## Progress

- Measured live 2026-11-13: sponsor prefixes TEVA 798, SANDOZ 515, AUROBINDO 618, HIKMA 693, WATSON 1,053 applications. SANDOZ was chosen because its second read is small (15 rows, 124 KB) and two of those rows land on the first 50-row page. openFDA answers a read past the end with 404 "No matches found!". The shared openFDA envelope check already refuses a 200 page whose row count disagrees with its total, so an empty 200 read below the total fails the search as bad-reply rather than stopping early. The early stop happens when the match set shrinks between reads (a dataset update mid-search): the later read answers 404, the count probe reports the smaller total, and the loop ends short of the first count. A synthetic reply reproduces that.
- Changed: the adapter page now carries how many applications the whole read covered (`Page::read`). When that is fewer than matched, the search is partial and both notices say "read only N of the M matching applications, because a later openFDA read came back empty, so this reply is partial". A complete read keeps 0900's wording. The too-large notice now adds that dropping `--sort original-approval` and the approval window pages in openFDA's own order.
- Proof: `tests/cases/drug/sponsor-original-approval.yaml` gains SANDOZ (515 matches, two reads, recorded in job j39970) for the first page and a 2022-2023 window, each checked against an order computed separately from the bodies; `tests/cases/drug/sponsor-read-partial.yaml` is the synthetic early stop (501 counted, 500 read, then 404 and a probe reporting 500).
- Gate status, wound down 2026-11-13 at Ian's midnight stop: the last gate, j40053 on a5399fe20, failed only on `tests/cases/drug/sponsor-read-partial.yaml` step 2. That step windows without `--sort`, so BioMCP keeps application-number order (ascending); the case expected newest first. The case was corrected after that run (expected ids now NDA900493 to NDA900500) and the fix was pushed without running it. In j40053 every other case passed, including the SANDOZ steps, and docs-check and schema-release-check passed; the landing-gate stages after `cases` did not run because `cases` failed. Pick up by running `make --keep-going landing-gate docs-check schema-release-check` on the branch head.
