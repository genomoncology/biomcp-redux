# 0271 — Design a streaming gRPC transport for AlphaGenome

Status: COMPLETE.
Landed: a0fd1880b184b32837afb3cf0db37f8f20950c00

Milestone: 5

## Outcome

A reviewed design says how BioMCP calls AlphaGenome's TLS streaming gRPC service from the shared transport, or recommends dropping the source. It names the HTTP/2 and protobuf dependencies, how the shared credential, quota, cache, deadline, replay and recording rules apply to a streamed reply, the size and memory bounds, and the packaging proof on every release target.

- Why: AlphaGenome's current official client and proto use streaming gRPC, and the approved blocking HTTP transport cannot call it, so 0198 cannot deliver without this.
- Starts from: the official `dna_client.py` and `dna_model_service.proto` in google-deepmind/alphagenome, the transport sections of architecture.md, `2026-10-26-new-sources-follow-open-by-default.md`, and the 0198 delegate report in m7290.
- Keeps: one shared transport, no publisher SDK, no automatic retries, keys read only when the source is called, and no new dependency before the design is accepted.
- Changes: a design section in architecture.md or a variant slice note. No product code.
- Proof: the PM reviews the design after an exact-commit docs review, and Ian approves either outcome, because adding a streaming client or an async runtime would change the one-transport decision and dropping AlphaGenome would reverse his entity boundary ruling. A design that adds dependencies states their size, build time and license.
- Defers: the product section, which stays 0198. Choosing the public variant, build, interval and outputs for any capture goes to a clinical reviewer before a call.
