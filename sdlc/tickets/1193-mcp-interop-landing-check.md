# 1193 — mcp-interop-landing-check

Status: OPEN.

Milestone: 7

## Outcome

The landing gate runs a standing MCP interop check over stdio. It runs the newest official TypeScript and Python MCP clients in both auto and legacy negotiation, plus one older official client for each revision BioMCP advertises. Each one initializes, lists tools and makes one tool call against a replayed source, with no network. The check takes about a minute.

## Why

The schema and reply-form regressions that the 1.0 standards review found were all invisible to BioMCP's own tests, and only real clients caught them. A check against the official SDKs stops the next one before it lands.

## Starts from

- Starts from: `Makefile` `landing-gate` and its `release-tests` stage; the existing replay fixtures under `tests/`; the official SDKs, `@modelcontextprotocol/sdk` on npm and `mcp` on PyPI, pinned to exact versions in a lock file.

## Decisions

- PM decision (2026-11-27): pin every client version exactly, and record the revision each one speaks. Use `npx`/`uv` with a lock file, not global installs.
- PM decision (2026-11-27): if the gate machine lacks Node or uv, the check targets the M5 through pm job, the way the hosting gates do. Say which in the Makefile comment.
- PM decision (2026-11-27): about a minute in total. If it costs more, cut to the newest client per SDK plus the oldest advertised revision, and say why.
- PM decision (2026-11-27): this check depends on 1191 and 1192 for the 2024-11-05 client and the 2026-07-28 list fields. Land it after them, or mark those clients as expected to fail until they land, and never weaken the assertion.

## Proof

- Proof: `make landing-gate` shows the interop stage passing, with its time; against main before 1192, the newest TypeScript client's `tools/list` fails in this stage.

## Changes

- Changes: `Makefile`, `tests/interop/`.

Depends on: 1191
Depends on: 1192
