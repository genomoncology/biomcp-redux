# 0849 — lean-reply-envelope-by-default

Status: OPEN.

Milestone: 7

## Outcome

Every reply's per-source outcome drops `terms` and `terms_url`, keeping `source`, `use`, `state`, `count` and `notices`; the licence text and links stay in `help sources`. Every reply schema takes one coordinated new major for it. `query` stays.

## Why

`sdlc/records/2026-11-11-token-gap-root-cause.md`: about 16% of reply bytes are envelope, and every reply lands in the conversation that is re-read each turn. 0838's design review measured a live 3-row trial search: dropping the `query` echo, the per-source licence fields and the server config path saved 15%; with the config path gone in 0837 and `query` kept, this is about 10–12% on short pages. A default applies to every turn, where the opt-in `fields` option helps only the turns that use it.

## Proof

- Proof: routine cases show no `terms` or `terms_url` in replies and the licence text in `help sources`; a size table over ten recorded replies before and after; the release check passes with the new majors.

## Starts from

- Starts from: source outcome building in `crates/biomcp-types/src/response/`, the reply schemas, `help sources`, `sdlc/decisions/2026-11-01-no-backwards-compatibility.md`.
- PM decision: 2026-11-11. Build after 0837 and 0838. Ian is told before it lands, because it changes where licence text appears; attribution by source name stays on every reply.

## Keeps

- Keeps: source name and use on every reply; default rows unchanged (0847 adds fields to rows on purpose).

## Defers

- Defers: other envelope trims until measured.

## Changes

- Changes: `crates/biomcp-types/src/response/`, `schemas/`, `crates/biomcp/src/help/`, `tests/cases/`
