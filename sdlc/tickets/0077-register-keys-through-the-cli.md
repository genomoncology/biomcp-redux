# 0077 — Register API keys through the CLI

Status: OPEN.

Milestone: 1

## Outcome

`biomcp key set <source>` prompts for a source's API key without echo and stores it in the config file's `keys`. `biomcp key list` shows which sources have a stored key, and `biomcp key remove <source>` deletes one. Every command then uses the stored key with no environment variable, as `decisions/2026-10-24-keys-are-registered-through-the-cli.md` says.

- Why: Ian ruled on 2026-10-24 that a person registers a key once and BioMCP reads it from its XDG config folder, so keys stay out of environment variables and shell history.
- Starts from: architecture 4.5 and 9.1; binary `config.rs` and `paths.rs`; `SecretName` and `Secrets`; `biomcp help key <source>`; the sources that take a key today: NCBI, OncoKB and Semantic Scholar.
- Set: `<source>` is a source name from `help key`, such as `semantic-scholar`, and an unknown or keyless source is a usage error that lists the valid names. On a terminal, BioMCP prints one prompt line and reads the key with echo off; otherwise it reads one line from standard input. An empty key is refused. A stored key is never replaced without `--replace`. The key is never accepted as an argument.
- Write: BioMCP reads the current config file, changes only `keys.<name>`, keeps every other setting, writes a temporary file beside it and renames it, creating the folder if needed, with owner-only permissions on Unix. A config file named by `--config` is the one changed.
- List: each source that takes a key, its config name, its environment variable, and `stored`, `environment` or `none`. No value is ever printed.
- Remove: deletes `keys.<name>` and keeps everything else; removing an absent key reports `none` and succeeds.
- MCP: the tool refuses `key` commands with a usage error, so an agent never handles a key.
- Help: `help key <source>` leads with `biomcp key set <source>` and still names the environment variable.
- Keeps: precedence of the config file over the environment variable, redaction everywhere, and the keyless and keyed behavior of every source.
- Proof: outside-in cases with an isolated config folder: set from piped input, then a keyed synthetic request shows the key was sent and the output and recording hold no value; set refuses an existing key and `--replace` replaces it; other settings in the file survive a set and a remove; list shows `stored`, `environment` and `none` with no value; remove; unknown source; empty key; MCP refusal; and on Unix the file mode is 0600. A unit test of the no-echo prompt is not needed when the terminal path is one call to a maintained crate; name the crate in architecture 2.1. Then make landing-gate runs through pm job after current main.
- Defers: encryption and OS keychains, which the ruling says keys do not need.
