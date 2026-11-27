# 1200 — mcp-skill-interop-verify

Status: OPEN.

Milestone: 7

## Outcome

The landing gate's `mcp-interop` stage checks the skill the way real clients see it. The pinned MCP Inspector CLI runs `--method skills/list --verify` against `biomcp serve` over stdio and passes. The pinned official TypeScript client lists the skill with its skills client and reads `SKILL.md` and one reference file. The release's manual checks gain one line: in an interactive Claude Code session (v2.1.331 or later) and in claude.ai with the connector, the skill appears and loads.

- Decided 2026-11-27: pin the Inspector and SDK versions exactly in the existing lock file, run them with the stage's pinned Node, and never install globally.
- Decided 2026-11-27: pass `--require-digests`, since BioMCP's manifest is static and a missing digest is a defect.
- Decided 2026-11-27: keep the stage near its current time; if the Inspector adds more than about 20 seconds, report the time rather than dropping the check.

## Why

BioMCP's own cases can agree with BioMCP's own mistakes; the 1.0 standards review found that only real clients catch schema slips. Inspector's verify is the extension's own conformance, digest and frontmatter check. The manual line exists because `claude -p` cannot load MCP skills.

## Proof

- Proof: `make landing-gate` shows `mcp-interop` passing with the skill checks and its time; against main before 1199, the Inspector verify exits non-zero (13, no skills to check) in this stage.

## Starts from

- Starts from: `Makefile` `mcp-interop`, `tests/interop/` (`check.mjs`, `package.json`, lock file), 1193 and 1198; Inspector CLI skill verification: https://github.com/modelcontextprotocol/inspector/blob/main/clients/cli/README.md#skill-verification---verify; the TypeScript SDK's `ext/skills` client in 2.4.0.

## Keeps

- Keeps: every existing interop client and assertion.

## Defers

- Defers: the deployed check against the hosted service belongs to the hosting repo.

## Changes

- Changes: `Makefile`, `tests/interop/`, `sdlc/planning/release.md` (manual checks).

Depends on: 1199
Depends on: 1198
