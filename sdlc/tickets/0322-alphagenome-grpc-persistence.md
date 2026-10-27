# 0322 — Cache, record and replay AlphaGenome streams

Status: COMPLETE.
Landed: b821b2cf0522263a633ba67c5b61537b6ddde7ef

Milestone: 5

Depends on: 0321

## Outcome

A complete AlphaGenome stream enters the checked cache and the recording format, and replay drives the same decoder and admission check with no connection.

- Why: Ian ruled on 2026-10-27 that AlphaGenome must work (`2026-10-27-alphagenome-streaming-transport.md`). PM ruling 2026-10-27 (m8126 ask 60) splits the transport along its review boundaries: 0320 codec, 0321 worker, 0322 persistence, 0323 cost and target proof.
- Starts from: the landed 0320 codec and 0321 worker, architecture section 4.13, the checked cache, and the recording and replay format.
- Rules: reply identity includes protocol version, authority, full method path, pinned schema identity and the exact credential-free request bytes. Only a complete stream with OK status, valid order, exact chunk counts, checked tensors and a passing identity check is cached; provider-default model requests use `Freshness::Never`. Recordings add a versioned gRPC tag, ordered bounded protobuf sidecars, message delays, terminal status and a typed interruption reason under the existing safe sidecar and atomic publication rules. Replay requires terminal status, fails closed on missing sidecars and advances the shared virtual clock. Offline mode never starts the network path. Limits apply on cache and replay reads.
- Proof: a recorded synthetic stream replays to the same tensors with zero connections; an interrupted stream is recorded as failed and never cached; a changed method or schema misses the cache; two processes share the ordinary cache; offline refuses without a connection.
- Changes: cache admission, recording, replay and cases.
- Defers: cost and target proof (0323) and the variant section (0198).
