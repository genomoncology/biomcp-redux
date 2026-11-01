# AlphaGenome joins 1.0 through a streaming gRPC transport

Date: 2026-10-27

Decided by Ian ("make alphagenome work"), recorded by the PM.

BioMCP keeps AlphaGenome predictions. `biomcp-fetch` gains a streaming gRPC client for AlphaGenome as designed in architecture section 4.13, as an explicit exception to the blocking transport and no-runtime rule. Released 0.9.3 already ships the same stack (tonic, prost, zstd and tokio) by default on every release target.

- The candidate dependencies in section 4.13 are approved for this source. `crates/biomcp/tests/ratchets/dependencies.rs` holds the allowed table.
- Work order: 0320 codec, 0321 worker, 0322 persistence and 0323 cost and target proof, all with zero provider calls; 0198 then adds the variant section with one bounded capture of a public variant.
- This replaces the 0271 recommendation to drop the source.
