# 0202 — Derive the desktop manifest tool list from the shipped tools

Status: OPEN.

Milestone: 1

Depends on: none

## Outcome

The desktop bundle declares exactly the tools the server lists: `biomcp`, `search` and `fetch`. The 0197 review found `manifest.json` declares only `biomcp`, while `crates/biomcp/src/mcp/protocol.rs` and `mcp/research.rs` add `search` and `fetch` since 0189. `release/gate.py` compares the installed and declared lists, so the next release candidate fails its installed-artifact check.

- Starts from: `manifest.json`, `release/artifacts.py`, `release/gate.py` and the server's tool list.
- Keeps: the exact installed comparison in the release gate.
- Changes: `manifest.json`, `release/artifacts.py`, `release/gate.py`, a source-only check run by `make docs-check` or the ordinary landing gate, and its test.
- Fix: one owner for the tool list. Either the manifest is generated from the server's declared tools, or a cheap source-only check in the ordinary landing gate compares the two lists and fails on any difference. Choose the smaller change; descriptions come from the server.
- Proof: the source-only check fails on the current tree and passes after the fix. A test adds a fake fourth tool and the check fails. The installed comparison runs on the next release rehearsal unchanged.
- Defers: release rehearsal itself, which stays with milestone 5.
