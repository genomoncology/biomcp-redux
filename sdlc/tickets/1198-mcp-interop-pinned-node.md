# 1198 — mcp-interop-pinned-node

Status: COMPLETE.
Landed: 3c16c495383e9f6a0921a6bda5813f652ccf923d

Milestone: 7

## Outcome

The landing gate's `mcp-interop` stage brings its own pinned Node. It downloads an exact Node release for the machine's OS and architecture into the job's `TMPDIR` (or a reused cache under it), checks its SHA-256 against the value committed in the repo, and runs npm and the TypeScript clients with that Node. The stage passes on a gate machine whose system Node is 18.

## Why

The newest official TypeScript MCP client needs a newer Node than some gate machines have, so the stage from 1193 would fail any core landing gate that runs on such a machine. The machine's system Node stays untouched.

## Starts from

- Starts from: `Makefile` `mcp-interop`, `tests/interop/` (from 1193); the official release files and `SHASUMS256.txt` at https://nodejs.org/dist/.

## Decisions

- PM decision (2026-11-27): pin one exact Node LTS version that the pinned clients support. Commit its SHA-256 values for linux-x64, linux-arm64, darwin-x64 and darwin-arm64, and refuse any download whose hash differs.
- PM decision (2026-11-27): never use or change the system Node for this stage; never install globally.
- PM decision (2026-11-27): keep the stage near its current time. Caching the download between runs under the job's temp area is allowed, and the first download's time is reported.

## Proof

- Proof: `make landing-gate` passes through pm job on a gate machine whose system Node is 18 and on one with a current Node, with the stage's time.

## Changes

- Changes: `Makefile`, `tests/interop/`, `scripts/`.
