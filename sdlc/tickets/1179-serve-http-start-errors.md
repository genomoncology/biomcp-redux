# 1179 — serve-http-start-errors

Status: OPEN.

Milestone: 7

## Outcome

`biomcp serve --http` on a busy port or a bind address that is not an IP prints an error naming the address and the cause, and exits non-zero. `serve --http` without `--saved-folder` offers no `saved` tool and no `save` step, as docs/configuration.md says.

## Why

The 2026-11-24 adversarial review saw `serve --http` exit 5 on a busy port and exit 2 on a non-IP address with no output at all, and wrote a note through the `saved` tool of a plain HTTP server, which the docs say is offered only with `--saved-folder` and which would be shared by every caller.

## Proof

- Proof: CLI cases for a busy port and a non-IP address check stderr and the exit code; an HTTP `tools/list` case without `--saved-folder` lists no `saved` tool, and a `saved` call is refused.

## Starts from

- Starts from: the `serve` command and the HTTP server start-up; typed tool list assembly for the HTTP transport; `docs/configuration.md:202`.

## Keeps

- Keeps: Stdio MCP keeps its local `saved` tool; `--saved-folder` keeps the per-account behaviour.

## Defers

- Defers: Nothing.

## Changes

- Changes: serve start-up errors, HTTP tool list, cases.
