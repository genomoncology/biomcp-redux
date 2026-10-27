# 0319 — Refresh the 1.0 schema baseline after the eligibility paging fix

Status: COMPLETE.
Landed: 0f802db6b683574eb0477f99e564c7f10b18d6fe

Milestone: 5

Depends on: 0317, 0318

## Outcome

The schema baseline is exported from the exact landed main that includes 0317 and 0318, with a manifest naming that product source commit and proving byte equality with the candidate's generated schemas, and the commit that lands it becomes the final candidate for 0293.

- Why: 0317 adds the continued-criterion flag to the trial eligibility page, which changes the generated schemas after 0314 froze the baseline, and the product source change means earlier installed-package evidence no longer carries forward. PM ruling 2026-10-27 (m8126 ask 59).
- Starts from: the 0314 snapshot recipe and gate, and landed main after 0317 and 0318.
- Rules: the 0314 rules apply: the manifest names the real product source commit, never its own landing commit, with no other product schema change and no version bump. The prior baseline stays until 0317 and 0318 land.
- Proof: the gate passes against the refreshed baseline, and the baseline files equal the candidate's generated files byte for byte, with the differences from 0314 limited to the eligibility page.
- Changes: the baseline snapshot and its manifest.
- Defers: publication, which needs Ian.
