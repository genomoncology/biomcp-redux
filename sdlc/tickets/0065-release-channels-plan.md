# 0065 — Plan every release channel and the note for 0.9 users

Status: COMPLETE.
Landed: 8de04b7c5675471609d62f51747db45ca455cb59

Milestone: 5

## Outcome

One release plan in `sdlc/planning/release.md` names each channel 0.9 ships and what 1.0 does with it: the `biomcp-cli` package on PyPI, `install.sh` from biomcp.org, the Homebrew formula, the GHCR image, the desktop extension `manifest.json`, and the MCP registry entry `io.github.genomoncology/biomcp`. For each one it states the artifact, the platforms, who publishes it, and how 0.9 users move. A short upgrade note for 0.9 users lists renamed and dropped commands from the parity ledger, the new config file and keys, and the HTTP endpoint.

- Starts from: the 0.9 repo's release scripts, `manifest.json`, `server.json`, `Formula/`, `install.sh` and `Dockerfile`; ticket 0058; the parity ledger from 0062.
- PM decision: 2026-10-24. Every 0.9 channel continues in 1.0 under the same name unless the plan states a reason. Publishing still needs Ian.
- Proof: 0058's install gate covers every channel named here, the desktop manifest and registry entry included. The upgrade note's command list matches the parity ledger, which the docs check verifies.
- Defers: the publishing itself.
