# 1015 — citations-say-their-order-and-evidence-offers-fulltext

Status: COMPLETE.
Landed: bbe5572c04353fa7dcabff17281bd216f9600090

Milestone: 7

## Outcome

`article citations` says it lists the most-cited citers first and shows each citer's `cited_by`; `article citation-evidence` offers a `--fulltext` next step when Semantic Scholar has no contexts or is unavailable.

## Why

msl 3.1, 3.2, 3.4 (bench `personas/msl/runs/2026-11-16/analysis.md`; `crates/biomcp-query/src/article/order.rs` 76-116).

## Proof

- Proof: replayed cases asserting the order note, `cited_by` and the next step; fail on the base commit.

## Starts from

- Starts from: the analysis named in Why, and the BioMCP code it cites.

## Keeps

- Keeps: every other reply field and command form.

## Defers

- Defers: nothing.

## Changes

- Changes: the named command's code, help, cases and a release-notes line.

## Progress

- Cause (3.1): iCite citation and reference pools are sorted most-cited first by `cited_by` (`crates/biomcp-query/src/article/order.rs`), and rows already carry `cited_by`, but neither the reply nor the help said so. Cause (3.2, 3.4): `article citation-evidence` without `--fulltext` offered no way to the citing paper's PMC paragraphs when Semantic Scholar had no contexts or failed; its only `next` was a pool page or a retry.
- Change: edge pages gain the optional `result.order`, `most-cited-first` for iCite pools and `source-order` for Semantic Scholar pools; an empty page leaves it out. The `--source` help of `article citations` and `article references` gains a second paragraph saying iCite lists the most-cited papers first by each row's `cited_by` and Semantic Scholar keeps its own order (the short help line is unchanged). `article citation-evidence` without `--fulltext` adds a `section` next step, the same pair with `--fulltext`, when Semantic Scholar reports `no-contexts` or is unavailable, unless PMC OA is disabled; the retry step stays. One release-notes line. `skills/use-cases/04-article-follow-up.md` already shows `--fulltext`, so it is unchanged. Schemas gain the one optional field and enum; the 1.0 baseline accepts it.
- Proof: `tests/cases/article/citations/citing.yaml` asserts `/result/order` `most-cited-first` beside the existing all-rows `cited_by` nonincreasing check; `tests/cases/article/edges/references.yaml` asserts `source-order`. New `tests/cases/article/evidence/fulltext-next.yaml` (replay `no-contexts`) and `fulltext-next-unavailable.yaml` (replay `s2-unavailable`) assert the `--fulltext` next step and that following it reads 7 PMC paragraphs. Semantic Scholar replies are synthetic (noncommercial terms): the `article/evidence/source` page with the cited row's contexts emptied, and three 500 replies; PubMed and PMC OA replies are the existing real records. No live recording was needed.
- Jobs: j42030 regenerated schemas and docs and ran check-cheap (passed) and article/mcp/help cases; with base 3701a52f0's crates all four touched cases fail (exit 101). j42039 regenerated after the help split. j42043 printed the failed reply (a failed reply carries no `result`, so the case asserts the source state). Gate j42049 failed one tool proof (`checked-edge-boundaries` expects `{}` for an empty page), fixed by leaving `order` off empty pages.
- Gate j42061 on ce2d87623 (contains origin/main 84f5db681): `make --keep-going landing-gate docs-check schema-release-check` passed; 1012 cases pass.
