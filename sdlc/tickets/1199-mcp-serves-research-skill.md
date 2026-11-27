# 1199 — mcp-serves-research-skill

Status: OPEN.

Milestone: 7

## Outcome

`biomcp serve`, over stdio and HTTP and in hosted mode, serves one skill, `biomcp-research`, through the Skills over MCP extension (`io.modelcontextprotocol/skills`). The server declares `resources` and the extension, answers `skills/list` and `skills/get`, and serves the skill's files through `resources/read` at `skill://biomcp-research/SKILL.md` and `skill://biomcp-research/references/<use-case>.md`. The server instructions gain one line that names the skill URI and says when to load it. A setting turns serving off; it is on by default.

- Decided 2026-11-27: the MCP skill is `biomcp-research`: a short `SKILL.md` that routes each question type to its use case, plus the use-case files as `references/`. It does not repeat the command map, which the instructions already carry. `skills/SKILL.md` stays the CLI guide for the plugin and `skill install`.
- Decided 2026-11-27: the `description` says when to use it, naming the kinds of multi-step questions the use cases cover, within the Agent Skills limit of 1,024 characters.
- Decided 2026-11-27: files are served byte for byte from the embedded assets, so the manifest is static for a build. No scripts and no binary files.
- Decided 2026-11-27: hosted mode leaves out any use case whose calls name a source the hosted catalog excludes, using the same exclusion the hosted instructions use.
- Decided 2026-11-27: declare the extension wherever the negotiated revision defines `extensions`. List the skill files in `resources/list` on every revision that has resources, so a client without the extension can still read them from the instructions pointer.
- Decided 2026-11-27: `resources/directory/read` is not offered (`directoryRead` false); the manifest is complete.
- Decided 2026-11-27: the setting is `mcp_skills` in the config file, default `true`, documented with the other MCP settings.

## Why

Claude now loads skills from MCP servers (public beta, 2026-11-24), so BioMCP's worked examples can reach chat and coding users with nothing to install, costing only a name and description until a model loads them. Plan: `sdlc/planning/skills-over-mcp.md`.

## Proof

- Proof: stdio and HTTP cases under 2026-07-28 show the extension and `resources` in `server/discover`; `skills/list` returns one entry whose manifest lists every file with a SHA-256 digest and byte size that match the bytes `resources/read` returns, and whose `frontmatter` matches the served `SKILL.md`; `skills/get` returns the same entry by URI; an unknown skill or file URI returns `-32602`; list and get results carry `resultType`, `ttlMs` and `cacheScope`.
- Proof: a `--hosted` case shows a use case whose calls need a source the hosted catalog leaves out is absent from the manifest, and `cacheScope` is `"private"`.
- Proof: with the setting off, discovery declares no skills extension and `skills/list` is refused as an unknown method.
- Proof: the instructions stay within their measured first-read ceiling in every tool mode; `make landing-gate`.

## Starts from

- Starts from: `crates/biomcp/src/mcp/rpc.rs` (method routing), `current.rs` and `legacy.rs` (capabilities), `protocol.rs` and `hosted.rs` (instructions), `typed/help.rs` (its "skill render" pointer), `crates/biomcp/src/skill.rs` and `build.rs` (`embed_skills`), `typed/examples.rs` (which use-case calls a catalog serves); `skills/use-cases/`. Spec: https://modelcontextprotocol.io/extensions/skills/overview and its full text in the ext-skills repository.

## Keeps

- Keeps: tool lists, tool replies and the typed catalog ceiling under every revision; `biomcp skill` commands and the plugin's skill folder; the wrapper's pass-through of MCP bodies.

## Defers

- Defers: the pinned Inspector and SDK skill checks are 1200; the hosted repin and deployed check belong to the hosting repo; measuring whether models load the skill waits for a scripted client that loads MCP skills (record's arm E).

## Changes

- Changes: `crates/biomcp/src/mcp/`, `crates/biomcp/src/skill*`, `crates/biomcp/build.rs`, `skills/`, `tests/cases/mcp/`, the MCP settings docs.
