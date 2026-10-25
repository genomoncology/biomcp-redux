# 0155 — Add one make target for the scheduled nightly live check

Status: OPEN.

Milestone: 3

## Outcome

`make nightly-live-check` runs the 0068 recipe end to end, so the scheduler only calls it. It makes a unique owned evidence folder outside the checkout and TMPDIR, runs the live check writing `report.json`, `build.log` and `exit-status`, copies the folder to the evidence store under `data/biomcp-redux/0068-scheduled/RUN` with a verified manifest, and deletes its M5 scratch. It exits nonzero when the report lists any drift or error case, or when the copy or manifest fails, so the scheduler's failure mail reaches the PM.

- Why: pm's scheduler runs the nightly check daily at 03:30 on the M5 but makes no evidence folder, copies nothing to the evidence store, deletes no scratch, and stays silent on success. A run that finds drift but exits zero would go unseen.
- Starts from: the recipe in 0068's progress, the existing nightly-checks target, and the live-check report fields `drift_cases` and `error_cases`.
- Proof: an offline run with a fake report and a local stand-in for the evidence store shows each case: no drift exits zero and copies; drift exits nonzero and still copies; a failed copy or manifest exits nonzero and keeps the scratch. No provider call and no whole-run retry. Then make landing-gate runs through pm job. After it lands, the PM asks Jarvis to switch the schedule's command to this target.
