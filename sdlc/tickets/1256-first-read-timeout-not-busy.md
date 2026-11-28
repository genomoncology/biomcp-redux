# 1256 — first-read-timeout-not-busy

Status: OPEN.

Milestone: 10

Depends on: 1247

## Outcome

A first-read download that runs out of time reports its own cause and exit code, not the store as `busy` with exit code 5, which means another caller holds the store.

## Why

Found while fixing 1247: after that fix the reason (`deadline-exceeded`) and the next step (the sync call) are right, but the same reply still marks the store `busy` and exits 5. A model or script reading `busy` waits for a lock that no one holds.

## Proof

- Proof: an outside-in case on a fresh store with a recorded first read that runs out of time: the reply does not say `busy`, and the exit code is the one BioMCP documents for an unavailable source, not the busy-store code. A true store lock still reads `busy` with exit 5.

## Starts from

- Starts from: `crates/biomcp-fetch/src/bundle/first.rs` and where the store-busy status and exit 5 are set for a first-read timeout.
