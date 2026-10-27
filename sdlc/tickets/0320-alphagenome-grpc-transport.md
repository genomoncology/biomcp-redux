# 0320 — Decode AlphaGenome messages and tensors within fixed limits

Status: COMPLETE.
Landed: f2984336564754825086a1565931064c481ac1cc

Milestone: 5

## Outcome

`biomcp-fetch` holds committed messages generated from AlphaGenome's pinned proto import closure and a codec that turns a sequence of stream messages into checked tensors within one transport-owned limits type, with no network code.

- Why: Ian ruled on 2026-10-27 that AlphaGenome must work (`2026-10-27-alphagenome-streaming-transport.md`). PM ruling 2026-10-27 (m8126 ask 60) splits the transport along its review boundaries: 0320 codec, 0321 worker, 0322 persistence, 0323 cost and target proof.
- Starts from: architecture section 4.13 and its pinned official sources, released 0.9.3's `alphagenome` feature and proto generator in the 0.9 repository as a working reference, and `crates/biomcp-fetch` at 14e26555.
- Rules: only prost and zstd without default features, added to `biomcp-fetch`. Generated messages are committed with a reviewed regeneration step against the pinned commit, so builds need no `protoc` or network, and Apache notices are kept. The limits type covers request bytes, individual message bytes, total wire bytes, message count, tensor rank and dimensions, chunk count, expanded tensor bytes and the ZSTD window; every allocation checks its limit first with checked arithmetic. Unknown dtypes, compression, payload variants and inconsistent counts fail as `bad_reply`. No AlphaGenome call and no key read.
- Generated code: PM ruling 2026-10-27 (m8126 ask 61): the source ratchet allows `dead_code` only on the one private generated proto module, named exactly in the ratchet, and nowhere else; regeneration must reproduce the committed files exactly. No other lint change.
- Proof: synthetic message fixtures decode a valid multi-chunk tensor, and malformed, oversized, wrong-order, wrong-count and decompression-bomb inputs each fail before allocating past their limit; regeneration reproduces the committed messages.
- Changes: `biomcp-fetch` codec and limits, generated messages, their generation step, the lockfile and cases.
- Defers: the network worker (0321), persistence (0322), cost and target proof (0323) and the variant section (0198).
