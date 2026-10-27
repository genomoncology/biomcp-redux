# 0352 — Write the PATH-resolved executable into generated MCP configs

Status: OPEN.

Milestone: 5

## Outcome

`biomcp mcp-config` writes the executable path a client will find through `PATH`, such as `/opt/homebrew/bin/biomcp`, when that path resolves to the running executable. It writes the running executable's own path only when no `PATH` entry resolves to it. Configs from a Homebrew install keep working after `brew upgrade`.

- Why: the docs lead's second-pass product findings (2026-10-27). On Homebrew the running executable resolves to a versioned Cellar path, so configs written today break at the next upgrade. Homebrew is a 1.0 channel.
- Starts from: `config.rs` `executable()`, the `mcp-config` command and its cases.
- Rules: compare files by identity after resolving symlinks; never write a `PATH` entry that resolves to a different file. No shell is invoked.
- Proof: a case with a `bin/biomcp` symlink on `PATH` pointing to a versioned folder writes the `bin` path; a case with no matching `PATH` entry writes the executable path; a case where `PATH` holds a different biomcp writes the executable path.
- Changes: the `mcp-config` path choice and cases.
- Defers: nothing. Joins the 0336 refresh.
