# 0844 — call-queue-covers-plain-http-and-bounds-stdio

Status: OPEN.

Milestone: 7

## Outcome

Plain `serve --http` queues calls over its limit as `--hosted` and stdio do, and the stdio queue has a length cap and a test of its wait.

## Why

The review of the call-queue ticket (0835) found: plain `serve --http` without `--hosted` has no queue, so the ninth call gets HTTP 503; the stdio queue has no length cap, only the wait setting; the stdio wait is bounded by a setting and never by a per-call deadline, with no test of its own; and a hosted client that disconnects while waiting keeps its place and still runs.

## Proof

- Proof: a case sends more calls than the limit to plain `serve --http` and all complete; a case fills the stdio queue past its cap and gets the timed refusal; each fails on the base commit.

## Starts from

- Starts from: the queue landed by ticket 0835 in `crates/biomcp/src/mcp/` and its tests in `crates/biomcp/tests/runner/call_queue.rs`.

## Keeps

- Keeps: the settings and defaults 0835 introduced; per-source rate limits; the refusal's message and `retry_after_seconds`.

## Defers

- Defers: dropping a disconnected client's waiting call, unless it is small; running calls already continue after a disconnect.

## Changes

- Changes: `crates/biomcp/src/mcp/`, `crates/biomcp/src/config.rs`, `crates/biomcp/tests/runner/`, `docs/configuration.md`
