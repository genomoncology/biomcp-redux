# 1138 — distinguish-publisher-backoff-from-sync

Status: OPEN.

Milestone: 7

## Outcome

When a publisher refuses a cold read, BioMCP says so, with the publisher's remaining delay and a next step the caller can take, instead of reporting `syncing` with a 60-second retry.

## Why

In the 2026-11-22 comparison (task dx-1b), GenCC refused BioMCP's first read with a daily per-IP limit. The reply prose said "Publisher Retry-After was 18855 seconds" while the machine-readable state said `syncing` and `retry_after_seconds: 60`. Connector-only agents retried and then gave up on five GenCC facts; agents with a shell went around BioMCP to the publisher. Code: `crates/biomcp-fetch/src/bundle/first.rs:102-130`, `crates/biomcp-query/src/disease/validity.rs:68-76`.

## Changes

- The stored failure keeps its cause and the publisher's deadline; the reply separates "local copy not ready" from "publisher refused until T".
- The state is not `syncing` when nothing is syncing; `retry_after_seconds` reflects the earliest useful retry.
- A usable stale copy is still served and labelled.
- Hosted mode never suggests an explicit sync.
- Keeps single-flight first reads and the explicit local sync override of the local cooldown (ticket 1070).

## Proof

- Extend the existing 429 first-read fixture: two reads against one store; reopen after the 60-second local wait but before the publisher delay; count upstream requests (no new one), and assert the state, remaining delay and no hosted sync hint. Distinguish a true in-progress sync, a refused cold read and a stale copy.

## Defers

- Sharing one cache across evaluation runs is the evaluation's concern, not BioMCP's.
