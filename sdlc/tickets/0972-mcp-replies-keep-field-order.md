# 0972 — mcp-replies-keep-field-order

Status: OPEN.

Milestone: 7

## Outcome

MCP replies list each object's fields in the same order as CLI JSON (the order the reply types declare), so what help and the skill say about field order is true over MCP too, without changing any value.

## Why

0962 (2026-11-15): MCP turns each reply into a JSON object that sorts keys alphabetically, so 0966's shared CDx fields (which lead each CLI row) come after FDA's `cells` over MCP, and `help search diagnostic` now says they start each row, which is false over MCP. Agents reading a truncated MCP reply miss what the CLI shows first, as the round 4 agent did with `head -40`.

## Proof

- Proof: an MCP case on the recorded CDx page shows `row_date` before `cells` in the text content; the full suite passes; cache keys, continuation tokens and replay matching are shown unaffected (or migrated) by a named check.

## Starts from

- Starts from: 0962's Progress (serde_json `preserve_order` in the workspace Cargo.toml, or ordered text in MCP text content only); the MCP reply path in crates/biomcp/src/mcp/.

## Keeps

- Keeps: every value; the 1.0.0 schema; recordings and cache keys still match, or the change says what it migrates.

## Defers

- Defers: nothing.

## Changes

- Changes: serde_json features or the MCP reply serialization, regenerated schemas/docs, cases.

## Progress

- 2026-11-15, route: serde_json `preserve_order` in the workspace Cargo.toml. The text-only route cannot meet the Outcome: local stdio servers send replies once, as `structuredContent` with no text copy (`config::mcp_reply_form`), and that object is serialized from a `serde_json::Value`, so only an ordered `Value` changes what local clients see.
- Audit of what hashes, compares or keys on JSON text, and whether key order reaches it:
  - Request cache keys and recording file names (`biomcp-fetch` `prepared.rs`, `request.rs`, `retry.rs`, `grpc/persistence.rs`, `secret.rs`, `privacy.rs`): hash URLs and request body bytes. Every JSON request body is a typed struct or `BTreeMap` (gnomAD, CIViC, Open Targets, cBioPortal, Semantic Scholar, GraphQL probe), and struct serialization never passes through `Value`, so the bytes do not change.
  - Replay matching (`record.rs`): compares method, post metadata (body SHA-256), source and URL; recorded bodies are kept as `RawValue` text. Unaffected; all recorded cases pass.
  - Query, snapshot and continuation keys (`source_snapshot.rs`, `store/pool.rs`, `store/ownership.rs`, article `identity()` keys, `trial/portable_continuation.rs`, `gwas/next.rs`): serialize typed query structs; `biomcp-types` holds no `Value`, `Map` or `HashMap`. Unaffected. A snapshot payload is looked up by its id, and its text is only parsed, so issued tokens stay valid.
  - Bundle revisions (`bundle/set.rs`): hash the downloaded bytes; the re-serialized row text is stored and parsed, never hashed.
  - `Value` equality and `Hash`: order-insensitive in serde_json with `preserve_order`, so in-process comparisons do not change.
  - Schema checks: `checked_in_schemas_match_types` compares file text; the release check compares parsed schemas.
- Measured (j41258, every workspace test plus docs-check and schema-release-check with only the feature on): 5 cases, 2 schema tests and the schema text check failed, all from code or tests that relied on alphabetical names: Markdown view section order (`fetch-keeps-fence-backslash`, and the publisher-terms check in two trial cases), the order of a corrected next step's flags (`source-filter-errors`), the drug-label test's expected section list, the drug header schema key test, and schemars' declared-order schema text.
- Changed: `key_order::sorted` restores alphabetical names where they were read before: Markdown rendering, `rewrite::query_value`, command words and the replacement step's flags. Checked-in schema files are written with sorted names, so they and the binary that reads them are byte-identical. Research fetch metadata drops `result` with `shift_remove`, which keeps the other fields in place (`remove` swaps the last field in). The two tests that read source or schemars order now sort.
- Proof: `diagnostic/fda-companion-shared-first` runs the July CDx search over local MCP (structured content) and hosted MCP (the text copy); both show `row_date` and `drug` before byte 701 of row 0 and `drug` before 2,918 of row 1, the limits where FDA's cells start. The case runner now checks the text copy when a reply sends one. Prep job j41267: fmt and schema regeneration left no diff; `--bin` tests, all cases and schema-release-check passed.
- serde_json's `preserve_order` brings in indexmap (already a dependency) and foldhash 0.2.0, which until now was a test-only dependency. foldhash is Zlib-licensed, so `deny.toml` names it as an exception, as it does zlib-rs.
- Seen in passing: `biomcp mcp-config` JSON now lists `command` before `args` (json! insertion order); no value changes.
- Review fix: with `preserve_order`, `Map::remove` swaps the last field into the removed slot. Hosted `valid_values` removal, dropped next steps (`schema/steps.rs`), and the MCP tool definition trims (`typed/catalog.rs`, `typed/grouping.rs`, `output_schema.rs`) now use `shift_remove`. The case runner gained `key_order` (exact pointer → the object's names in order, read from the raw reply). `mcp/hosted-error-field-order` shows a hosted refusal that keeps `valid_values` lists the error fields in CLI order, and one that drops them (both pathway sources disabled) gives `option, code, message` (a swapping remove would put `message` before `code`; not run without the fix). The MCP `diagnostic` tool sentence now matches help ("start with row_date, diagnostic, indication, device and drug, before FDA's cells"). j41343: check-cheap, mcp/mcp-parity, diagnostic and hosted cases, and the `biomcp-types` schema tests with `--features schema` (62) passed.
