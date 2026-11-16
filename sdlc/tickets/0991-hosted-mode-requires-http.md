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

## Progress

- Decision: the check sits in the `serve` grammar (`crates/biomcp/src/grammar/local.rs`), after clap parses, so `serve --hosted` without `--http` returns a `usage.arguments` error naming `--http` with the new fixed message "hosted mode runs only over HTTP behind the hosting front end; add --http" and a `help serve` next step, exit 2, before any store, fetch or listener starts. The message is a new `ErrorMessage` literal, which widens an enum in the error schema (allowed for the 1.0 candidate). `--hosted=false` still serves stdio.
- Proof: `tests/cases/local/hosted-needs-http.yaml` (fails on the base, where `serve --hosted` starts serving stdio). Hosted HTTP start-up stays covered by the existing hosted HTTP runner trials.
- Tests that started hosted stdio now start hosted HTTP through one helper, `mcp::exchange_hosted`, with the use header that keeps what they checked. Legs that only compared hosted stdio with hosted HTTP are dropped (`typed_policy`, `hosted_surface`, `variant/search_policy`, the stdio `mcp-hosted` hosted legs). Renamed trials: `mcp-hosted/keys/stdio-license` is now `mcp-hosted/keys/account-license`; the hosted-mode pool contract is now `mcp-hosted/http/health`; `mcp-hosted/stdio/reader` is gone.
- `scripts/check_front_map.py` measures the hosted first read over loopback HTTP with `x-biomcp-use: academic`, the closest match to the old unrestricted hosted stdio base.
- Dead code: none removed. Every hosted branch in `src/` is also reached by hosted HTTP; the stdio server now always passes `hosted = false`, but that is a value, not a branch.
- Docs: `docs/configuration.md` hosted section, `help serve`, the `--hosted` option help and `release/notes.md`.
