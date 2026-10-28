# 0376 — Keep concurrent readers working during a bundle pair replacement

Status: OPEN.

Milestone: 5

## Outcome

Readers that run while a bundle pair is being replaced read the old or the new generation and never get `local_error` `busy`. The cause of the busy failure in `wikipathways/concurrent-readers-during-pair-replacement` (job j18227 on the 0362 branch) is found and fixed.

- Why: the coordinator reported the failure (m10791 ask 4, 2026-10-27). It does not involve the network fixture that 0351 covers. Ian's ruling that 1.0 ships every known fix.
- Starts from: the store's catalog replacement path, its SQLite busy and transaction handling, the 0295 generation coexistence and 0296 snapshot ownership rules, and the j18227 log.
- Rules: no sleeps, retries or longer busy timeouts as the fix unless measured and explained against the store's transaction design. Atomic pair replacement stays.
- Proof: a deterministic case that held a reader and a replacement in the failing order failed before the fix and passes after; the existing concurrent reader cases pass repeatedly in one job.
- Changes: the store replacement path and cases.
- Defers: nothing.
