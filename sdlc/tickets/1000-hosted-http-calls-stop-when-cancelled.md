# 1000 — hosted-http-calls-stop-when-cancelled

Status: COMPLETE.
Landed: c70d5bce8216717b6c641be126981969c86163ec

Milestone: 7

## Outcome

A running hosted HTTP call stops promptly when its client goes away or cancels it. Its source requests close, and its capacity slot comes back within seconds, not at the call's deadline. What HTTP does with `notifications/cancelled` is stated in the docs and proven.

## Why

The hosting team's core handoff (2026-11-16) asked core for this, and Ian asked for hosting's core work to be ticketed and prioritized. PM research on main 82360be61:

- **stdio:** `notifications/cancelled` marks the call cancelled, closes its source sockets and drops it from the queue (`crates/biomcp/src/mcp.rs:157-163,237-245`).
- **HTTP:** every notification gets a plain 202 and is dropped (`mcp/http.rs:241`). Nothing maps a request ID to running work.
- **HTTP, client leaves while queued:** the call gives up its place (`mcp/http/call.rs:162-165,190-203`).
- **HTTP, client leaves while running:** this isn't noticed. The work runs to its deadline (20 s, or 60 s for VAERS) and holds its slot until then (`call.rs:232-262`).

0781 (refresh shutdown) and 0852 (a test race) don't cover this.

## Proof

- Proof: a runner test over hosted HTTP starts a call held on a slow recorded source, then closes the client connection. The source request closes, and the slot is free well before the deadline: a second call at the call limit is admitted without waiting. The test fails on the base commit.
- Proof: either `notifications/cancelled` over HTTP cancels the caller's own running request, matched by account and request ID so one caller can't cancel another's, or the docs state that HTTP ignores it and that a client cancels by closing the connection. The developer chooses, records why in Progress, and a test shows the chosen behaviour.
- Proof: `docs/configuration.md` states the hosted timeout and cancellation contract: the default deadline, slow-source budgets, queue wait, call limit and what cancels a call.

## Starts from

- Starts from: the files above; the stdio cancellation path; `biomcp-fetch/src/cancellation.rs`; the hosted runner tests in `crates/biomcp/tests/runner/`.

## Keeps

- Keeps: slow-source budgets (0909, 0912); shutdown waiting for real worker exit; one request per connection. Synchronous work stops at its next cancellation check. This is not an async rewrite.

## Defers

- Defers: progress notifications and streaming, unless hosting shows a supported client failing.

## Changes

- Changes: hosted HTTP call handling, one runner test, configuration docs, the notes.

## Progress

- Test: runner trial `mcp-hosted/client-that-leaves-stops-running-call` (`crates/biomcp/tests/runner/hosted_cancel.rs`). A hosted server with `http_max_calls` 1, a 60 s queue wait and a 120 s deadline runs `get protein P15056` through a proxy that holds its CONNECT. The test sends `notifications/cancelled` for that request ID (HTTP 202, source socket stays open), closes the client, then requires the source socket to close within 5 s and a second call to answer within the request's 5 s read limit.
- Change: while a hosted call runs, its waiter checks the client socket on each 20 ms poll, the same check a queued call already made. A closed client cancels the call's scope, which shuts its source sockets and stops new requests, and the slot frees when the worker exits. Plain HTTP is unchanged: it has no shared call scope, and the ticket covers hosted calls.
- `notifications/cancelled` over HTTP: ignored, and the docs say a client cancels by closing its connection. Why: every HTTP call has its own connection and no session, request IDs are chosen by each client and repeat across clients, and a hosted call need not carry an account header, so a request ID alone names no single caller. Matching by account would still let two sessions of one account cancel each other's calls. Closing the connection can only cancel the closer's own call. Our replies are plain JSON with no resumable stream, so a client that left can never get the answer; stopping its work loses nothing.
- Docs: one paragraph in `docs/configuration.md` (Hosted servers) and one release-notes line. The paragraph links to "Command deadlines" for the slow-source budgets instead of repeating them, so it stays right when 0998 adds ClinVar gene counts (30 s) there.
