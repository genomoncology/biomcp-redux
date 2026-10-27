# 0338 — Print help for the usual help entry points and name unknown commands

Status: OPEN.

Milestone: 5

## Outcome

`biomcp`, `biomcp -h`, `biomcp --help`, `biomcp <command> --help` and `biomcp help <partial path>` print help and exit 0, and `biomcp --version` prints the version and exits 0. A partial path such as `help get` lists its subcommands. An unknown command, such as `discover BRAF` or `help frobnicate`, returns a distinct unknown-command usage code that names the nearest valid command. A retired 0.9 command names its 1.0 replacement from the upgrade map. `list` leaves the command grammar.

- Why: the docs lead's product findings report on main a01667d (2026-10-27), rechecked by the PM on the release candidate build from hosted run 40542150253. Every one of these returns `usage.schema: expected schema <known artifact id>` with exit 2, and `get gene --help` returns `usage.arguments`. A first-time user or agent meets this before anything else, and 0.9.3 answered all of them.
- Starts from: the binary's argument parsing and dispatch, the help artifact, the error code list and the upgrade map.
- Rules: the help printed for these entry points is the existing `biomcp help` content, in the requested format, scoped to the path. Messages name the command, never echo other input, as 0013 and 0025 say. A nearest-command suggestion uses only the known command list.
- Proof: CLI cases for each entry point above exit 0 with help text; `help frobnicate`, `discover BRAF` and `serve --bogus` return the unknown-command code; a retired 0.9 command names its replacement; `biomcp help` no longer lists `list`.
- Changes: the binary's parser and dispatch, error codes, CLI cases and the schema baseline if the error schema changes.
- Defers: help defaults, examples and Markdown layout, which go to 0357.
