# Keys are registered through the CLI

Date: 2026-10-24

Decided by Ian.

A person registers a source's API key once with `biomcp key set <source>`. BioMCP prompts for the key, reads it without echoing it, and stores it in the config file's `keys` in the XDG config folder (`2026-10-23-settings-live-in-an-xdg-config-file.md`). The key never appears as a command argument, so it stays out of shell history. When standard input is not a terminal, BioMCP reads one line from it, so a password manager can pipe the key in.

- These are API keys, not secrets that need encryption. The file is plain JSON, written with owner-only permissions where the platform has them.
- `biomcp key set` refuses to replace a key that is already stored unless the person passes `--replace`.
- `biomcp key list` names each source that takes a key and whether one is stored, and never prints a value. `biomcp key remove <source>` deletes one.
- Every command reads the stored key from the config file it already loads, so a key costs no extra work per call and no environment variable is needed.
- The documented environment variables still work after the config file, so containers and 0.9 users keep working. The docs lead with `biomcp key set`.
- The MCP tool refuses `key` commands, so an agent never handles a key.
