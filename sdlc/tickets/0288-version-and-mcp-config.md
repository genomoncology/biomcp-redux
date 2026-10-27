# 0288 — Add the version and mcp-config housekeeping commands

Status: COMPLETE.
Landed: dc04141bdc25ce34cd62a5380d228e763a57a950

Milestone: 5

## Outcome

`biomcp version` prints the version, and `--verbose` adds the build commit, target, schema versions and the config and cache paths. `biomcp mcp-config` prints a ready-to-paste MCP client configuration that runs this binary over stdio, using the binary's absolute path, and `--client NAME` selects one of the documented client formats. Both make no source request and read no key.

- Why: architecture.md's housekeeping grammar names `version` and `mcp-config`, 0.9.3 shipped both, and the 0275 accounting ports them instead of dropping them.
- Starts from: architecture.md's command grammar at its housekeeping line, the 0.9.3 `version` and `mcp-config` help and options, `paths.rs` for config and cache paths, and the generated schema registry.
- Rules: client formats keep their 0.9 names. Output never includes key values.
- Proof: cases assert `version` and `version --verbose` fields, `mcp-config` output parses as JSON with the absolute binary path and `serve` arguments, an unknown `--client` is a usage error, and both run with an empty environment and offline.
- Changes: the grammar, two small commands, help, the MCP tool description if it lists housekeeping, and the cross-cutting accounting rows.
- Defers: nothing.
- PM ruling 2026-10-26 (m8126 ask 16): `mcp-config` emits two neutral JSON shapes, `mcp-servers` (the default, with `json` as an alias) and `servers`. The 0.9 absolute-path option stays accepted as a no-op because output always uses the absolute path. Amended after m8340: public products may be named, so the 0.9 shell command format is kept under its 0.9 client name for parity, printing the add command with the absolute binary path.
