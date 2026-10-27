# 0323 — Measure the gRPC transport's cost and prove it on every release target

Status: OPEN.

Milestone: 5

Depends on: 0322

## Outcome

A build delegate measures what the landed gRPC transport adds and runs the installed candidate on every release target with the local fixture.

- Why: Ian ruled on 2026-10-27 that AlphaGenome must work (`2026-10-27-alphagenome-streaming-transport.md`). PM ruling 2026-10-27 (m8126 ask 60) splits the transport along its review boundaries: 0320 codec, 0321 worker, 0322 persistence, 0323 cost and target proof.
- Starts from: landed main after 0322, baseline 14e26555, release.md's targets and the 0293 rehearsal recipe.
- Rules: the same toolchain, profile, host and cache conditions for baseline and candidate. Raw timings and sizes are kept through ordinary job evidence. Package checks refuse a runtime Python, SDK, `protoc` or system ZSTD requirement. A failed target returns to the PM before 0198 starts. No AlphaGenome call.
- Target proof, PM ruling 2026-10-27 (m8126 ask 67): on each available target, run the public worker API synthetic TLS fixture through the fetch integration tests, and separately check the installed CLI package and its lack of runtime SDK, Python, `protoc` or system ZSTD needs. Installed CLI proof through the gRPC path belongs to 0198, after its adapter exists.
- Proof: executable size, transitive crate count, cold and warm build time and peak build memory against the baseline; the installed candidate passes the local TLS fixture on Linux x86_64 and arm64, macOS x86_64 and arm64, and Windows x86_64, or names the target it could not run.
- Changes: none expected beyond evidence and fixes it finds, each its own ticket.
- Defers: the variant section (0198).
