# 0852 — cancel-test-stops-flaking

Status: COMPLETE.
Landed: 6646a9bb180be64a2a2eda82aa2126123fced857

Milestone: 7

## Outcome

The landing-gate case `mcp-parity/cancel-stops-work-and-frees-capacity` no longer fails with "failed to fill whole buffer" when the build machine is busy. Its test proxy accepts a cancelled call that closes its socket before it finishes the `CONNECT` line, because closing is what a cancelled call should do. The proxy still checks that every socket it accepted ends up closed.

## Why

The case failed three times on 2026-11-11 on tickets that don't touch MCP code (0827, 0829 and 0834), and each passed on rerun. The cause is a race in the test, not a product bug.

The case starts eight UniProt calls, but UniProt's quota (`crates/biomcp-adapters/src/quotas.rs`, `UNIPROT`) lets only two run at once, at two per second. So two calls open proxy sockets and six wait in the rate limiter. The test cancels the first call (`call`) first, then calls 1 to 7 one at a time. When the first call's slot frees, a waiting call whose cancel the server hasn't read yet takes the slot and opens a socket. Its cancel arrives a moment later and closes that socket before the `CONNECT` line is complete. The test proxy reads the header with `read_exact` in `proxy_request`, gets end of file, and fails with "failed to fill whole buffer".

It began with 0835 (queue calls over the concurrency limit), which made the ninth call's refusal wait one second before the cancels go out. Before that, the cancels went out within milliseconds, when the rate limiter had no token yet, so a waiting call saw its cancel before it could start. One second later the token is back, so a waiting call starts at once.

## Proof

- Proof: the case run 480 times (24 at a time, 20 rounds, with 8 busy-loop processes) on the build machine fails at least once on origin/main and never with the fix.

## Starts from

- Starts from: `crates/biomcp/tests/runner/held_proxy.rs` (`proxy_request`, `hold`, `cancel_capacity`) and `crates/biomcp-fetch/src/quota/limiter.rs`.

## Keeps

- Keeps: the UniProt quota; the one-second queue wait; the order of the cancels; a short or wrong header failing every mode where the test waits for a `CONNECT`; the checks on the ninth call's refusal and on freed capacity.

## Defers

- Defers: nothing.

## Changes

- Changes: `crates/biomcp/tests/runner/held_proxy.rs`

## Evidence

- Saved pm job logs: 757 runs of the case found. Before 0835 landed (2026-11-11 17:18 UTC), 617 runs, with no "failed to fill whole buffer" failures. After it, 40 runs, with 3 of them failing that way: j38143 (0827, load 9.0 of 18 cores), j38153 (0834, load 15.7 of 18), and j38165 (0829). All three ran on m5 between 22:36 and 23:30 UTC with four or five other jobs alongside, and each failing commit includes 0835. Reruns j38154 (0827) and j38171 (0834) passed on the same commits.
- Other failures of this case have a different cause and are not part of this flake: j37645 (0835's own branch, refusal shape) and j38173 (0837's branch, source policy).
- j38198, origin/main 664ebb95c under the stress run in Proof: 477 of 480 passed, and the 3 failures were "failed to fill whole buffer". This reproduces the flake.
- j38199, the first fix attempt (19c367a11, wait for eight `CONNECT`s before cancelling): 0 of 480 passed, all "held CONNECT watchdog". This shows that only two calls ever reach the proxy, so the quota is part of the cause. That attempt is reverted.

## Progress

- 2026-11-11: the proxy fix is on the branch but not proven yet. Next: run the stress job from Proof on the branch tip and on origin/main, then run the landing gate.
