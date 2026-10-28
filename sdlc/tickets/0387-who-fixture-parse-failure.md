# 0387 — Fix the shared WHO case parse failure in full test runs

Status: OPEN.

Milestone: 5

## Outcome

`drug-who/native-rows-local-reads-and-replacement` passes in full test runs. The cause of its bare JSON parse failure is found from the diagnostic context added at 0da96b3bd and fixed at its owner, whether in the replacement recording, the case fixture or MCP parsing.

- Why: the coordinator's RCA (m11651, 2026-10-28, logged in 0060) found the same single failure in two full Gator runs on unrelated branches, 0360 j19020 and 0354 j19033, each after about 1,500 seconds, blocking both lanes. No machine or product cause is shown yet.
- Starts from: the case, the WHO fixture and replacement recording, the diagnostic job j19060, and the archived j19020 and j19033 logs.
- Rules: fix the real cause; no retry, skip, timeout change or new framework. A focused pass alone is not a root cause.
- Proof: a deterministic reproduction failed before the fix and passes after, or the diagnostic witness names the cause and a case covers it; the next full run passes this case.
- Changes: depends on the cause.
- Defers: nothing. Priority: it blocks 0360 and 0354, which wait for it; no gate bypass.
