# AlphaGenome joins 1.0 through a streaming gRPC transport

Date: 2026-10-27

Decided by Ian ("make alphagenome work"), recorded by the PM.

BioMCP keeps AlphaGenome predictions. `biomcp-fetch` gains a streaming gRPC client for AlphaGenome as designed in architecture section 4.13, as an explicit exception to the blocking transport and no-runtime rule. Released 0.9.3 already ships the same stack (tonic, prost, zstd and tokio) by default on every release target.

- The candidate dependencies in section 4.13 are approved for this source: tonic and tonic-prost without default features, prost, h2, hyper, tokio, tokio-rustls on the existing rustls and platform verifier, and zstd without default features. Only `biomcp-fetch` depends on them, and the existing HTTP sources keep the blocking transport.
- The shared transport rules of section 4.13 hold: one lazy worker, one attempt with no retries, one quota permit per stream, an absolute deadline, the checked cache, recordings and replay, credential containment, and the bounds limits type.
- Work order: 0320 codec, 0321 worker, 0322 persistence and 0323 cost and target proof, all with zero provider calls; 0198 then adds the variant section, after a clinical reviewer chooses the public variant and outputs for one bounded capture.
- AlphaGenome joins the 1.0 release if it is ready with the rest; otherwise it ships in a 1.0 update. Ian decides release timing.
- This replaces the 0271 recommendation to drop the source.
