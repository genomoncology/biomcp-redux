# 0310 — Give the scheduled nightly live check its evidence settings from local config

Status: COMPLETE.
Landed: a50197fa4aa26533e66648c557ea3ce952737840

Milestone: 5

## Outcome

The scheduled nightly live check finds its evidence host, evidence directory and scratch directory without arguments, from an untracked local settings file on the machine that runs it, and the repo names no machine or workspace path.

- Why: scheduled job j16467 failed at argument parsing because 0279 moved the evidence host and directories out of the script into `--host`, `--evidence-dir`, `--scratch-dir` or `BIOMCP_EVIDENCE_HOST`, `BIOMCP_EVIDENCE_DIR` and `BIOMCP_SCRATCH_DIR`, and the scheduled command `make nightly-live-check` supplies none of them. No source was called.
- Starts from: `scripts/nightly_live_check.py` lines 283 to 290, the `nightly-live-check` Makefile target, the 0252 nightly rules and the pm schedule, which runs `make nightly-live-check` on the build machine and has no environment field.
- Rules: the settings live in a local file outside the repo, such as an optional Makefile include under the user's config directory, and the environment and arguments still override it. The repo documents the three settings without real values. The coordinator creates the local file on the scheduled machine with the values the script used before 0279.
- Keeps: every 0252 rule, verified evidence delivery and the existing exit codes.
- Proof: `make nightly-live-check` with only the local file set gets past argument parsing in a dry or fixture run, and with no file it still fails with the same clear message. The next scheduled run passes argument parsing.
- Changes: the Makefile target or script defaults and a short note in the nightly docs.
- Defers: nothing.
