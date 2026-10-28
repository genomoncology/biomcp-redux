# 0336 — Cut the 1.0 release candidate from main once every 1.0 ticket lands

Status: OPEN.

Milestone: 5

Depends on: 0198, 0334, 0338, 0339, 0340, 0341, 0342, 0343, 0344, 0345, 0346, 0347, 0348, 0349, 0350, 0351, 0352, 0353, 0354, 0355, 0356, 0357, 0358, 0359, 0360, 0361, 0362, 0363, 0364, 0365, 0366, 0367, 0368, 0369, 0370, 0371, 0372, 0373, 0374, 0375, 0376, 0377, 0378, 0379, 0380, 0381, 0382, 0383, 0384, 0385, 0386

## Outcome

`release/1.0` takes main's tree in one merge commit once every listed ticket has landed on main, its schema baseline is re-exported from that product source, and one approved hosted release.yml run passes on the new head.

- Why: `decisions/2026-10-27-1-0-ships-every-known-fix.md`. Ian wants every known fix in 1.0, so the candidate is cut from main instead of cherry-picking chosen commits.
- Starts from: the `release/1.0` head after 0332, main after the last listed ticket lands, and the 0314 snapshot recipe.
- Rules: the merge has `release/1.0` as first parent and main as second, and its tree equals main's. The manifest names the real product source commit. The hosted run on the new head is the installed proof. The earlier cherry-pick plan and the m10484 net-commit ruling are superseded, because main's whole tree now ships.
- Proof: the merge tree equals main's tree; the gate passes; the baseline equals the release head's generated schemas; release/1.0 holds no conflict markers; the hosted run passes all five targets, containers and Homebrew.
- Changes: the `release/1.0` branch and its baseline.
- Defers: publication and the history append, which need Ian.
