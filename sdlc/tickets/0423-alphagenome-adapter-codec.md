# 0423 — Move the AlphaGenome codec into its adapter

Status: OPEN.

Milestone: 5

## Outcome

The AlphaGenome request types, encoder, reply validation and generated messages live in `biomcp-adapters::alphagenome` behind one score function. The fetch crate keeps only the generic gRPC stream decoder and tensor types. PM ruling 2026-10-28: `prost` joins the adapter crate's allowed dependencies for this.

- Why: the pre-1.0 code quality review `sdlc/records/2026-10-28-code-quality-review.md`. About 2,000 lines of provider knowledge sit in the transport, against architecture 4.13 and 5.6.
- Starts from: `crates/biomcp-fetch/src/grpc/alphagenome`, `grpc/generated`, `crates/biomcp-adapters/src/alphagenome.rs`, `crates/biomcp-query/src/variant/alphagenome*.rs`, `tests/ratchets/dependencies.rs`.
- Rules: no behavior change; the installed TLS proof from 0392 still passes.
- Proof: existing AlphaGenome cases and the installed proof pass; the fetch crate has no AlphaGenome names.
- Changes: the files above.
- Defers: nothing.
