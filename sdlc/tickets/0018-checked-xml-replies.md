# 0018 — Decode checked XML replies through the shared transport

Status: OPEN.

Milestone: 1

## Outcome

Leg::xml returns checked typed replies from declared GET or form endpoints through all transport modes, and MixedText preserves inline element text in document order.

Depends on: 0017

- Starts from: architecture 4.2 and the MixedText contract in 5.4; request.rs Leg::send/attempts/decode; cache_request.rs checked_cached/offline; record.rs text bodies; declaration.rs endpoints; existing transport stub and public contracts. Read the POST branch after it lands for exact prepared-request boundaries.
- Changes: `Cargo.toml`, `Cargo.lock`, `crates/biomcp-fetch/Cargo.toml`, `crates/biomcp-fetch/src/lib.rs`, `crates/biomcp-fetch/src/xml.rs`, `crates/biomcp-fetch/src/request.rs`, `crates/biomcp-fetch/src/cache_request.rs`, `crates/biomcp-fetch/tests/xml.rs`, `crates/biomcp/tests/ratchets/source.rs`.
- Scope: Add the architecture-approved quick-xml decoder with minimal serde features after verifying its current official API and license. Reuse the single prepared request, attempts, quota, body/deadline limits, mode and checked-cache path. XML MIME checks and decoding are selected locally without duplicating retry, cache or recording policy. Decode/check failures remain input-free bad_reply, invalid cached XML is evicted and refetched, and offline/replay preserve their accepted policies. Define MixedText from the actual decoder API so nested inline elements contribute text in order, including plain text, character/entity references and CDATA. Keep mapping and biological interpretation outside transport. Bound malformed/deep input using parser controls and existing body limits without an unbounded secondary parse tree. No external entity or network resolution.
- Keeps: Existing JSON/GET/POST identities and recording formats remain usable. Reply::check remains the single raw validity boundary. No adapter or CLI grammar, live data, generic DOM or alternate request stack. Exact accepted XML media types and cache representation choices need a recorded ruling if architecture does not settle them.
- Proof: Public API local-stub contracts prove checked GET/form XML, malformed XML/wrong media type/error-body refusal, bounds and deadline behavior, cache reuse/stale fallback/invalidation, record text then replay with server absent and offline reads. A compact architecture-permitted MixedText table covers interleaved/nested sup/sub/i/b, character references, escaped text, CDATA and empty elements without dropping variant text. Existing JSON/POST contracts and make landing-gate pass through pm job after current main. No source under test needs licensed answers or provider access.
- Defers: Scoped batch is the next whole transport step. The first article command adds its real recorded mixed abstract case; this ticket uses synthetic markup. Binary recording sidecars wait for the first article supplement source.
