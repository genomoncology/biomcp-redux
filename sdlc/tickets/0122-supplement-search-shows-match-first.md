# 0122 — Show the queried match on the first page of a supplement search

Status: COMPLETE.
Landed: c6d961785814901819f968cd38589b9ce0c6929e

Milestone: 1

## Outcome

When an agent searches an article's supplementary material for a term, the first page shows the rows that contain the term before any other rows, or says plainly that no row on the page contains it. The first page no longer spends its budget on unrelated row fragments.

- Why: 0121's capture for the F1761I supplement question filled the first page with 25 row fragments and a continuation, and the queried match did not appear. The page cost 863 tokens against 0.9's 649 and still did not answer the question.
- Starts from: the supplement reader and its cases, and the 0121 capture kept outside the repo.
- Proof: an outside-in case from the 0121 recording asserts the matching row appears on the first page, a case with no match asserts the stated absence, and the first page's tokens are at or below 0.9's 649 or the case states why not. Matching is literal on the term the agent gave, with no rewriting. No provider call. Then make landing-gate runs through pm job.
