# 0336 — Cut the 1.0 release candidate from main once every 1.0 ticket lands

Status: OPEN.

Milestone: 5

Depends on: 0198, 0334, 0338, 0339, 0340, 0341, 0342, 0343, 0344, 0345, 0346, 0347, 0348, 0349, 0350, 0351, 0352, 0353, 0354, 0355, 0356, 0357, 0358, 0359, 0360, 0361, 0362, 0363, 0364, 0365, 0366, 0367, 0368, 0369, 0370, 0371, 0372, 0373, 0374, 0375, 0376, 0377, 0378, 0379, 0380, 0381, 0382, 0383, 0384, 0385, 0386, 0387, 0388, 0389, 0390, 0391, 0392, 0393, 0394, 0395, 0396, 0397, 0398, 0399, 0400, 0403, 0404, 0406, 0407, 0408, 0409, 0410, 0411, 0412, 0413, 0414, 0415, 0416, 0417, 0418, 0419, 0420, 0421, 0422, 0423, 0424, 0425, 0426, 0427, 0428, 0429, 0430, 0431, 0432, 0433, 0434, 0435, 0436, 0437, 0438, 0439, 0440, 0441, 0442, 0443, 0444, 0445, 0446, 0447, 0448, 0449, 0450, 0451, 0452, 0453, 0454, 0455, 0456, 0457, 0458, 0459, 0460, 0461, 0463, 0464, 0465, 0466, 0467, 0468, 0469, 0470, 0471, 0472, 0473, 0474, 0475, 0476, 0477, 0478, 0479, 0480, 0481, 0482, 0483, 0484, 0485, 0486, 0487, 0488, 0489, 0491, 0492, 0493, 0494

## Outcome

`release/1.0` takes main's tree in one merge commit once every listed ticket has landed on main and the schema baseline has been re-exported on main as its last commit before the merge, and one approved hosted release.yml run passes on the new head.

- Why: `decisions/2026-10-27-1-0-ships-every-known-fix.md`. Ian wants every known fix in 1.0, so the candidate is cut from main instead of cherry-picking chosen commits.
- Starts from: the `release/1.0` head after 0332, main after the last listed ticket lands, and the 0314 snapshot recipe.
- Rules: the merge has `release/1.0` as first parent and main as second, and its tree equals main's. The manifest names the real product source commit. The hosted run on the new head is the installed proof. The earlier cherry-pick plan and the m10484 net-commit ruling are superseded, because main's whole tree now ships.
- Baseline (PM ruling 2026-10-28, pre-release review): re-export `release/schema-baseline.*` on main from main's own source as the last commit before the merge, with its status naming the 1.0 release candidate, so the merge tree still equals main's tree and 1.0.1 compares against the shipped set.
- Proof: the merge tree equals main's tree; the gate passes; the baseline equals the release head's generated schemas; release/1.0 holds no conflict markers; the hosted run passes all five targets, containers and Homebrew.
- Changes: the `release/1.0` branch and its baseline.
- Defers: publication and the history append, which need Ian.
