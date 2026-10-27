# 0320 — Add the streaming gRPC transport for AlphaGenome to the shared fetch layer

Status: OPEN.

Milestone: 5

## Outcome

`biomcp-fetch` can make one bounded, deadline-limited streaming gRPC call over TLS, with the shared quota, cache, recording, replay and credential rules, proved against a local TLS HTTP/2 test service with synthetic messages and zero AlphaGenome calls, and with its size and build cost measured.

- Why: Ian ruled on 2026-10-27 that AlphaGenome must work (`2026-10-27-alphagenome-streaming-transport.md`); its service is TLS streaming gRPC only, and the shared transport is blocking HTTP.
- Starts from: architecture section 4.13 and its pinned official sources, released 0.9.3's `alphagenome` feature and proto generator in the 0.9 repository as a working reference, `crates/biomcp-fetch`, `quotas.rs`, the recording and replay format, and release.md's targets.
- Rules: the dependencies the decision approves, only in `biomcp-fetch`, without default features. Generated messages from the pinned proto import closure are committed, so builds need no `protoc` or network, and Apache notices are kept. One lazy worker owns the runtime; callers stay blocking; shutdown cancels and joins. One attempt, no retries, hedging or reconnect-and-resend; one permit per stream; the absolute leg deadline covers every phase; only a complete stream with OK status and valid chunks is cached; replay makes no connection. The key goes only into outgoing metadata and never into identities, errors, traces or recordings. The limits type bounds every allocation, including decompressed tensor bytes and the ZSTD window. Proxy routes and the platform verifier with additive CA roots are reused. No AlphaGenome call and no key read.
- Proof: local fixture cases for TLS and HTTP/2 negotiation, platform plus added CA and invalid certificate refusal, proxy routing, deadline in each stream phase, one attempt on refusal or interruption, permit lifetime, malformed and oversized streams, decompression limits, recording, replay and offline; a measured comparison of executable size, dependency count, cold and warm build time and peak build memory against the unchanged baseline; installed checks on the release targets through the 0293 rehearsal recipe.
- Changes: `biomcp-fetch`, the lockfile, generated messages and their generation step, and cases.
- Defers: the AlphaGenome variant section and its capture, which are 0198.
