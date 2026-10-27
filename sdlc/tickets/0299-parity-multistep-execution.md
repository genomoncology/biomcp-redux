# 0299 — Run multi-step and stateful harvested cases in the parity runner

Status: OPEN.

Milestone: 5

Depends on: 0298

## Outcome

The parity runner executes the cases the 0298 inventory classes as multi-step or stateful, such as followed actions, walks, YAML-form files and bundle cases, as ordered sequences of direct commands on each binary with state kept across the steps of one case, and compares their asserted facts.

- Why: the 0298 inventory found at least 35 followed-action steps, 25 walks, 12 YAML-form files and bundle or stateful cases that the 0291 runner cannot execute. PM ruling 2026-10-26 (m8126 ask 38).
- Starts from: the landed 0291 runner, the 0298 inventory and declarations, and the 0.9.3 release binary.
- Rules: released 0.9.3 is the baseline. A step that 0.9.3 has no command for is reported with its reason, and only facts both binaries can be asked are compared. Each capture may hold several exchanges in order. No live calls and no generic framework. PM ruling 2026-10-27 (m8126 ask 43): sequence steps reuse the direct declarations of 0298 and 0300 to 0305 by command shape, with no per-case sequence declarations. A step whose command shape has no direct declaration is a counted, named gap, never labeled executable. The PM reviews the gap count after the family batch lands before any further declaration work is filed.
- Proof: one recorded end-to-end case each for a followed action, a walk, a YAML-form file and a bundle case, including one drug and one study positive case that begins with a sync, produces both verdicts, the two WikiPathways missing-bundle cases are run as missing-state scenarios with their source identity facts (PM ruling 2026-10-27, m8126 ask 41), and the dry inventory reports no remaining unexecutable class.
- Changes: the runner's sequence and state handling, ordered per-case state, and the shared navigator callback.
- Defers: the whole run, which is 0297.
