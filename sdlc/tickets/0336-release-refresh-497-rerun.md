# 0336 — Refresh the 1.0 release candidate with the 497 rerun and docs findings fixes

Status: OPEN.

Milestone: 5

Depends on: 0334, 0335, 0338, 0339, 0340, 0341, 0342, 0343

## Outcome

`release/1.0` gains only the 0334, 0335 and 0338 to 0343 fix commits, cherry-picked from main, with its schema baseline re-exported from the new release product source if schemas change, and one approved hosted release.yml run passes on the new head.

- Why: the experiment 497 rerun found two remaining 1.0 gaps after 0332. PM ruling 2026-10-27 (Jarvis m9946). Amendment 2026-10-27: the docs findings report found six release blockers in help, arguments, configuration, schema IDs, health and output; their fixes join this one refresh so one hosted run covers them all.
- Starts from: the `release/1.0` head after 0332, the 0332 rules and the 0314 snapshot recipe.
- Rules: the 0332 rules apply: only these commits, no AlphaGenome transport file, the manifest names the real product source commit, and the hosted run on the new head is the installed proof.
- Proof: the gate passes, any baseline equals the release head's generated schemas, the cases of each listed ticket pass on the release head, and the hosted run passes.
- Changes: the `release/1.0` branch and, if needed, its baseline.
- Defers: publication and the history append, which need Ian.
