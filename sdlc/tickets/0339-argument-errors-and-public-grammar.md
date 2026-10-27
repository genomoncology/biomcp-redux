# 0339 — Name bad arguments and give the public grammar complete descriptions

Status: OPEN.

Milestone: 5

## Outcome

`usage.arguments` names the missing or bad argument. A negative number is accepted as an option's value, so `search trial --lat 40.7 --lon -74.0060 --distance 50mi` works. `key set` on an existing key says it needs `--replace`. `serve --http` on a non-local address without `--allow-host` returns a usage error that names `--allow-host`. The test-harness flags `--replay`, `--record`, `--case-cache` and `--observe` are hidden from public help and from the grammar `make docs-generate` reads. Every global option and top-level command has a one-line description in `biomcp help`, and `serve`, `sync`, `key set`, `schema` and `help` list their arguments. `article references` has its own description, and `search author` describes only its own options.

- Why: the docs lead's product findings report on main a01667d (2026-10-27), rechecked by the PM on the release candidate build from hosted run 40542150253. The generated biomcp.org reference pages read this grammar, so gaps and harness flags reach the public site.
- Starts from: the clap definitions, the help artifact builder, the `usage.arguments` mapping and `make docs-generate`.
- Rules: name the argument, never echo its value, as 0013 and 0025 say. Harness flags keep working for tests.
- Proof: CLI cases for each behavior above; a check over `biomcp help` fails if a global option, top-level command or argument lacks a description, or if a harness flag appears; the negative longitude case runs against a recording.
- Changes: the CLI grammar, help text, error mapping, CLI cases and the docs generator input.
- Defers: defaults, value types, examples and aliases in help, which go to a 1.0 update.
