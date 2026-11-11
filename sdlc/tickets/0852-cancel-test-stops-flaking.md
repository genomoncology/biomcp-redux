# 0852 — cancel-test-stops-flaking

Status: OPEN.

Milestone: 7

## Outcome

The landing-gate case `mcp-parity/cancel-stops-work-and-frees-capacity` sends its cancels only after all eight held calls have opened their sockets at the test proxy, so a busy build machine can no longer fail it with "failed to fill whole buffer". The case also checks that it closed all eight sockets, not just the ones that happened to connect.

## Why

The case failed three times on 2026-11-11 on tickets that don't touch MCP code, and each passed on rerun. The cause is a race in the test, not a product bug. The test starts eight calls but waits only for the first to reach the proxy. It then waits for the ninth call's refusal, which since 0835 (queue calls over the concurrency limit) arrives after a one-second queue wait, and then cancels all eight. On a loaded machine some of the other seven calls are still opening their proxy connection after that second. Cancel shuts the socket after it connects but before the call writes its `CONNECT` line, so the test proxy's `read_exact` in `proxy_request` sees end of file and returns "failed to fill whole buffer", which fails the case. The product did the right thing: it stopped the cancelled call's work.

## Proof

- Proof: the case run 480 times (24 at a time, 20 rounds, with 8 busy-loop processes) on the build machine fails at least once on origin/main 664ebb95c and never with the fix.

## Starts from

- Starts from: `crates/biomcp/tests/runner/held_proxy.rs` (`cancel_capacity`, `hold`, `proxy_request`).

## Keeps

- Keeps: the test proxy treating a short `CONNECT` header as a failure; the one-second queue wait; the case's checks on the ninth call's refusal and on freed capacity.

## Defers

- Defers: nothing.

## Changes

- Changes: `crates/biomcp/tests/runner/held_proxy.rs`

## Evidence

- Saved pm job logs: 757 runs of the case found. Before 0835 landed (2026-11-11 17:18 UTC), 617 runs, with no "failed to fill whole buffer" failures. After it, 40 runs, with 3 of them failing that way: j38143 (0827, load 9.0 of 18 cores), j38153 (0834, load 15.7 of 18), and j38165 (0829). All three ran on m5 between 22:36 and 23:30 UTC with four or five other jobs alongside, and each failing commit includes 0835. Reruns j38154 (0827) and j38171 (0834) passed on the same commits.
- Other failures of this case have a different cause and are not part of this flake: j37645 (0835's own branch, refusal shape) and j38173 (0837's branch, source policy).
- Stress jobs: j38198 (before the fix, origin/main) and j38199 (after the fix).
