# 0017 — Send declared form and JSON POST requests through the shared transport

Status: OPEN.

Milestone: 1

## Outcome

Declared form and JSON POST endpoints use the same checked request, quota, deadline, retry, cache, record and replay path as GET, with method/body identities that cannot reuse another request's reply.

Depends on: 0014

- Starts from: architecture 4.2 and 4.4–4.7; declaration.rs Endpoint/SourceSpec/Arg; request.rs Leg::{json,json_opt}, send, attempts, call, exchange and decode; retry.rs canonical/identity; cache_request.rs; record.rs recording identity validation in the accepted 0014 branch.
- Changes: `crates/biomcp-fetch/src/declaration.rs`, `crates/biomcp-fetch/src/lib.rs`, `crates/biomcp-fetch/src/request.rs`, `crates/biomcp-fetch/src/retry.rs`, `crates/biomcp-fetch/src/cache_request.rs`, `crates/biomcp-fetch/src/record.rs`, `crates/biomcp-fetch/tests/**`.
- Scope: Add SourceSpec::post_form/post_json, endpoint method and explicit read-only POST idempotence, and Leg::post_json with a Serialize body. Existing json/json_opt route form parameters into the declared form body while GET retains current URL behavior; reject a method/API mismatch before storage or network. Extend credential-free canonical identity with method and exact effective request body, using unambiguous framing. Keep current GET recording/cache compatibility where possible. Send configured query/header credentials only on the wire; no credentials in identity, files or diagnostics. Serialize bounded bodies once for all attempts, with typed input-free local failure. Share all existing response validation and mode policy. Retry only declared idempotent endpoints; a non-idempotent POST makes one admitted attempt. Preserve per-attempt quota and deadline limits.
- Keeps: No HTTP-client duplication, new runtime, external provider calls or adapter consumer before the shared transport is complete. Record bypasses cache reads/fallback; replay never falls through to HTTP or ordinary cache. Keep checked 404 absence and stale-positive rules. No generic request middleware or premature CLI flags.
- Proof: Public API contracts against the existing local stub inspect form/JSON content types, escaping and exact bodies; demonstrate equal requests share identity and method/body changes do not; exercise configured credentials without persistence; show idempotent POST retries and non-idempotent refusal to retry; record then replay with server absent and verify cache/mode/decoder behavior. Wrong API/method, required-key absence and serialization failures cause no network or recording access. Existing GET contracts and make landing-gate pass through pm job after current main.
- Defers: XML/MixedText and bounded scoped batch fan-out are subsequent whole transport steps before adapters. Binary recording sidecars arrive with the first article supplement source. Query grammar and normal output envelopes arrive with the first gene command.
