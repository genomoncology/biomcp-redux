# 0321 — Run AlphaGenome streams through one bounded gRPC worker

Status: OPEN.

Milestone: 5

Depends on: 0320

## Outcome

`biomcp-fetch` makes one deadline-limited streaming gRPC call over TLS through a lazy worker that owns one runtime, with blocking callers, the shared quota and the existing proxy and certificate rules, proved against a local TLS HTTP/2 service.

- Why: Ian ruled on 2026-10-27 that AlphaGenome must work (`2026-10-27-alphagenome-streaming-transport.md`). PM ruling 2026-10-27 (m8126 ask 60) splits the transport along its review boundaries: 0320 codec, 0321 worker, 0322 persistence, 0323 cost and target proof.
- Starts from: the landed 0320 codec, architecture section 4.13, `quotas.rs`, the proxy and verifier code, and 0.9.3's reference client.
- Rules: tonic and tonic-prost without default features with client channel, codegen and ring TLS, plus the h2, hyper, tokio and tokio-rustls they need, only in `biomcp-fetch`. Bounded command and response channels; shutdown cancels pending streams and joins. One attempt with no retries, hedging, reconnect-and-resend or service-config retries. One quota permit covers a whole stream. The absolute leg deadline covers admission, quota wait, DNS, CONNECT, TLS, upload, every message, decoding and trailers, and `grpc-timeout` carries the remaining budget. gRPC status maps to the existing reasons. The key goes only into outgoing metadata, provider text is allowlist filtered, and no key appears in errors or traces. Proxy routes and the platform verifier with additive CA roots are reused. No AlphaGenome call and no key read.
- DNS: PM ruling 2026-10-27 (m8126 ask 62), matching the 0283 cancellation contract: the system resolver stays, with no new resolver dependency. The caller returns at the absolute deadline during DNS, the stream never starts after the deadline, and its permit is released; a started system lookup may finish on its blocking thread after the deadline or shutdown, bounded by the system resolver timeout, and is documented as the one exception to the no-detached-work rule.
- Proof: local fixture cases for TLS and HTTP/2 negotiation, an added CA and invalid certificate refusal, explicit and environment proxy routing with exclusions, a deadline in each stream phase, one attempt on refusal or interruption, permit lifetime, status mapping and worker shutdown.
- Changes: the worker, transport wiring, quota declaration, lockfile and cases.
- Defers: persistence (0322), cost and target proof (0323) and the variant section (0198).
