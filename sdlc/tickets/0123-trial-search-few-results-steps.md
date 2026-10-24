# 0123 — Offer wider steps when a trial search finds few trials

Status: OPEN.

Milestone: 2

## Outcome

When `search trial` returns fewer trials than its page limit, `next` names wider searches, each changing one thing: the same search without `--variant`, and the same search with the `--gene` symbol moved to `--criteria` so that it matches eligibility text. Each step keeps every other option. `--gene` keeps its meaning as registry search text, and nothing is widened automatically.

- Why: the PM's comparison with 0.9.4 on outside judged trial topics (0098) found 1.0 ahead overall, but 29 of 118 typed searches returned zero trials as `complete` with no step forward, mostly rare variants such as KIT L576P, and 7 gene searches missed trials that name the gene only in eligibility criteria. PTCH1 with basal cell carcinoma found 5 trials through `--gene` and 11 through eligibility text. This follows 0102's article steps.
- Starts from: trial search's `next` builder, the trial rulings, `--criteria` as the trial slice defines it, and 0102's step order.
- Proof: replay cases with known positives captured on one date: a gene, variant and condition search with zero results asserts the step without `--variant`; PTCH1 with basal cell carcinoma asserts the `--criteria PTCH1` step and that its capture finds trials the `--gene` search lacked; a search that fills its page asserts no wider step. One bounded keyless capture through pm job. Then make landing-gate runs through pm job.
