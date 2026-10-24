# 0103 — Measure the HTTP close event before test cleanup

Status: COMPLETE.
Landed: 6d6206c3b55e9a7aa16af94b367706086948074a

Milestone: 1

## Outcome

The slow-header outside-in case measures the server close event separately from test cleanup, and its failure identifies the read result and event duration.

- Why: the article gate failed the HTTP deadline case, while a bounded isolated reproduction passed. Its elapsed assertion currently includes shutdown and joining the writer, so the failure cannot distinguish the server deadline from cleanup scheduling.
- Starts from: the existing slow-header check in `http_limits.rs`, failure j10684 and targeted reproduction j10692.
- Changes: capture elapsed immediately after the checked read; keep cleanup unconditional; report the read result, close-event elapsed and cleanup elapsed without secrets. Preserve the existing server deadline and asserted close bound. Do not change HTTP product behavior or weaken rejection of an open connection, read failure or late close.
- Proof: run the existing targeted case through pm job and show the assertion measures the read event before cleanup. Run the relevant formatting, lint and ratchet checks. Integrate the accepted fix with the article lane before its next full gate; do not run another full suite only for this test-only change.
- Defers: changing product timeouts, new timing gates, repeated load experiments and claims about the original failure's cause without evidence.
