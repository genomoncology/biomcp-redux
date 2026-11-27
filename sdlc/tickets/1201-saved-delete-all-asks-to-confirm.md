# 1201 — saved-delete-all-asks-to-confirm

Status: COMPLETE.
Landed: cb042185a9f09914d692d48ed2e4a85e4f5b2cfe

Milestone: 7

## Outcome

When an MCP client declares form elicitation, the `saved` tool's `delete` with `all: true` asks the user to confirm through a form before it deletes anything, naming how many searches, lists and notes it will remove. Confirm deletes as today; decline or cancel deletes nothing and returns a reply that says so. A client that does not declare elicitation keeps today's behavior. No other call uses elicitation.

- Decided 2026-11-27: form mode only, one boolean confirm field; no URL mode.
- Decided 2026-11-27: under 2026-07-28 use the stateless multi round-trip form, so HTTP and the hosted service need no session state; under older revisions use it only on stdio, where the server can send the request.
- Decided 2026-11-27: the confirm text holds counts only, never saved names, notes or search words.

## Why

Deleting all saved work cannot be undone, and an agent can send that call on its own; this is the "confirm before destroying data" case elicitation exists for, and Claude made elicitation generally available on 2026-11-24. Plan: `sdlc/planning/skills-over-mcp.md`.

## Proof

- Proof: a stdio case where the client declares form elicitation: `delete` with `all: true` produces an elicitation request; a scripted accept deletes and the saved file is empty; a scripted decline leaves the file unchanged and the reply says nothing was deleted.
- Proof: the same call from a client that declares no elicitation deletes as today; deleting one named search or note never asks; `make landing-gate`.

## Starts from

- Starts from: `crates/biomcp/src/mcp/typed/saved.rs` (`delete`), `rpc.rs`, `current.rs` and `legacy.rs` (client capabilities per revision); the 2026-07-28 elicitation specification, https://modelcontextprotocol.io/specification/2026-07-28/client/elicitation, including its multi round-trip "input required" form; `sdlc/planning/saved-work.md`.

## Keeps

- Keeps: every other saved action and its reply schema; the CLI's `saved delete --all`, which a person types directly.

## Defers

- Defers: nothing.

## Changes

- Changes: `crates/biomcp/src/mcp/`, `tests/cases/mcp/`, `tests/cases/saved/`.
