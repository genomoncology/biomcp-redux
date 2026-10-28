# 0388 — Fix the shared history secret scan failure

Status: COMPLETE.
Landed: 25138f26feb8f5bfa7d981bae8a90e16196704a8

Milestone: 5

## Outcome

`make secrets` passes on the final history scan of every ticket branch again. The cause of the generic suppressed exception is found from the redacted diagnostic and fixed at its owner, whether in the scanner invocation, its allowlist bindings or a recorded blob.

- Why: the coordinator (m11750, 2026-10-28) found the same failure in three unrelated lanes, 0370 j19105, 0379 j19106 and 0351 j19115. Each passed every earlier stage and failed only the final history scan. The six synthetic ZIP notices are informational and are not shown to be the cause.
- Starts from: the scan-only redacted diagnostic j19127, the three archived logs and the scanner configuration from 0347.
- Rules: fix the real cause. No new ignore, broader allowlist, skipped stage or scanner policy change. Any new allowlist entry is an exact fingerprint for a reviewed non-secret, as 0347 requires. The diagnostic and any report never print scanner match contents.
- Proof: the diagnostic names the cause, a focused check covers it, and the next full run of one of the three lanes passes the history scan.
- Changes: depends on the cause.
- Defers: nothing. Priority: it blocks 0345, 0351, 0370, 0379, 0381, 0383 and 0387, which wait for it; no gate bypass.
