# 0351 — Diagnose the local CONNECT and TLS fixture connection abort

Status: COMPLETE.
Landed: 4f8bb9cdff6340ee89e370a92111c7c6cdf81ea4

Milestone: 5

Depends on: 0388

## Outcome

The cause of the connection abort (OS error 53) seen once in `adverse-event-get/synthetic-source-admission-before-retained-read` during native job j18030 is found and fixed. If it does not recur, the shared local CONNECT and TLS fixture instead keeps enough evidence on failure to explain the next one: the phase labels the lander added, the peer and phase of the failing I/O, and the fixture's scratch state kept when a case fails.

- Why: the coordinator reported one unexplained abort in the shared local proxy fixture (m10551, 2026-10-27). The case passed when rerun alone, the original log lacked the I/O phase, and pm removed the fixture scratch at the end of the job, so the cause is unknown.
- Starts from: the shared local CONNECT and TLS stub, its new phase labels, the j18030 failed log and the j18053 passing log in the batch evidence.
- Rules: no sleeps, longer timeouts, retries or gate exemptions as a fix. A passing rerun is not a root cause.
- Proof: either a deterministic reproduction that fails before the fix and passes after, or a fixture case showing that a forced abort leaves its phase, peer and scratch state in the job log.
- Changes: the shared local network fixture and its tests.
- Defers: nothing. It ships in 1.0.
