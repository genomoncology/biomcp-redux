# 0991 — hosted-mode-requires-http

Status: OPEN.

Milestone: 7

## Outcome

`biomcp serve --hosted` without `--http` is refused with a usage error that says hosted mode runs only over HTTP behind the hosting front end. Hosted always means HTTP.

## Why

Ian, 2026-11-16 (`sdlc/decisions/2026-11-16-review-follow-ups.md`, item 2), on finding C1 of `sdlc/records/2026-11-14-adversarial-review.md`.

Every per-caller control exists only on the HTTP route (`crates/biomcp/src/mcp/http.rs` `caller()`): `x-biomcp-use`, `x-biomcp-exclude-sources`, `x-biomcp-key-*` and `x-biomcp-account`. So hosted stdio applies no use tier and serves noncommercial sources, such as ORCID and Semantic Scholar, with the operator's fallback credentials (`command/hosted_use.rs:44-49`, `command/context.rs`).

## Proof

- Proof: a runner test shows `serve --hosted` with no `--http` exits with a usage error before serving, and `serve --hosted --http` still starts. It fails on the base commit.
- Proof: `docs/configuration.md`, `help serve` and `release/notes.md` say hosted needs `--http` (decision item 5).

## Starts from

- Starts from: `crates/biomcp/src/grammar/local.rs` (the `serve` grammar), the finding's evidence list.

## Keeps

- Keeps: plain `serve` over stdio unchanged; plain `serve --http` unchanged.

## Defers

- Defers: nothing.

## Changes

- Changes: the serve grammar or start-up check, its test, docs and the notes.
