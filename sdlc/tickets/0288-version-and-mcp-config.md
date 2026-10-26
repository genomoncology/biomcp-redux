# 0288 — Add the version and mcp-config housekeeping commands

Status: OPEN.

Milestone: 5

## Outcome

`biomcp version` prints the version, and `--verbose` adds the build commit, target, schema versions and the config and cache paths. `biomcp mcp-config` prints a ready-to-paste MCP client configuration that runs this binary over stdio, using the binary's absolute path, and `--client NAME` selects one of the documented client formats. Both make no source request and read no key.

- Why: architecture.md's housekeeping grammar names `version` and `mcp-config`, 0.9.3 shipped both, and the 0275 accounting ports them instead of dropping them.
- Starts from: architecture.md's command grammar at its housekeeping line, the 0.9.3 `version` and `mcp-config` help and options, `paths.rs` for config and cache paths, and the generated schema registry.
- Rules: client formats name each client by its config file shape, not by vendor, except where a format has no neutral name, which comes to the PM. Output never includes key values.
- Proof: cases assert `version` and `version --verbose` fields, `mcp-config` output parses as JSON with the absolute binary path and `serve` arguments, an unknown `--client` is a usage error, and both run with an empty environment and offline.
- Changes: the grammar, two small commands, help, the MCP tool description if it lists housekeeping, and the cross-cutting accounting rows.
- Defers: nothing.
