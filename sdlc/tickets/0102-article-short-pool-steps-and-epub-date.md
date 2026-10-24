# 0102 — Offer one-term-wider steps when an article search finds few papers, and show the electronic date

Status: COMPLETE.
Landed: 6a39d6ed9f973a2885578335d71beeecacfcdfbf

Milestone: 1

## Outcome

When `search article` returns fewer papers than its page limit and the query has more than one of TEXT, `--variant`, `--disease` and `--drug`, `next` names the same search with each of those terms dropped, one step per term, in that fixed order, each with nothing else changed. A paper whose electronic publication date differs from its print date shows both, and help for `--since` and `--until` says the filter matches either date.

- Why: the PM's comparison with 0.9.4 on outside judged topics (0098) found 1.0 ahead overall, but five gene-plus-variant-plus-disease searches returned zero papers as `complete` with no step forward, 22 of 120 returned under 10, and 0.9.4 found judged-relevant papers for several of them. In the same run 2.5 percent of rows capped by `--until` showed a print year after the cap, because PubMed's `[dp]` also matches the electronic date and the row showed only the print date.
- Starts from: article.md sections 5 (term order, what the response says, next steps) and 6; `decisions/2026-10-24-no-text-heuristics.md`; 0035's term builder and next steps; 0037's date filters; ESummary's `epubdate` and `pubdate`.
- Rule: BioMCP never widens a query by itself and never reruns it. The steps are the agent's own terms minus one, built from typed options, so they stay inside the no-text-heuristics ruling. A complete zero stays `complete`. The `--gene` group is never dropped, because it names the subject.
- Keeps: status, totals, pool and page behavior, ceilings within their rule, and every existing case except the added step and date.
- Proof: recorded public cases: `search article --gene NF2 --variant K322 --disease meningioma` (or a captured gene, variant and disease search that PubMed answers with zero) asserts `complete`, zero items and the drop-variant, drop-disease steps, and following the drop-variant step returns papers; a search that returns more than a page asserts no drop steps; a capped search on a paper with an earlier electronic date shows both dates; help asserts the date wording; MCP parity. Then make landing-gate runs through pm job after current main.
- Defers: changing PubMed's date field, which would drop papers first published online inside the range.
