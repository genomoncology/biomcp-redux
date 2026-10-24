# 0001 — Establish the Cargo workspace and landing gate

Status: COMPLETE.
Landed: 1ea89af819b0d5ac9d98394743492c4de74c7a2a

Milestone: 1

## Outcome

A clean checkout builds the stage-one Cargo workspace and exposes `make landing-gate` as the routine entry point with the workspace lint table inherited by each existing crate.

- Starts from: `sdlc/plan.md` stages 1 and 2, `sdlc/planning/architecture.md` sections 2.1–2.4, and `sdlc/planning/rust-standards.md`. Main has no Rust code.
- Changes: `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `rustfmt.toml`, `deny.toml`, `Makefile`, `.gitattributes`, `.gitignore`, `crates/biodata/Cargo.toml`, `crates/biodata/README.md`, `crates/biodata/src/lib.rs`, `crates/biomcp/Cargo.toml`, `crates/biomcp/src/main.rs`, `crates/biodata/clippy.toml`, `crates/biomcp/clippy.toml`, `crates/biomcp/tests/ratchets.rs`, `crates/biomcp/tests/ratchets/**`, `ROADMAP.md`.
- Keeps: Create only biodata and the binary that hosts the ratchet integration target. Add each remaining crate when its stage needs it. Product text uses BioMCP or BioMCP 1.0. Never call sources, embed credentials, or add clinical commands.
- Scope: Pin the exact toolchain, inherit the workspace lint table, and wire formatting, clippy with warnings denied, the ratchet target, ordinary tests and license/advisory checks into the gate. Keep future runner and schema features absent until their stages. Use the smallest executable entry point without fabricated output or unused APIs. Confirm dependency versions and features against primary documentation when introducing them and explain dependencies in commit bodies.
- Proof: Developers build and push their disjoint changes in parallel. After both branches are ready, the coordinator assigns the 0001 developer to assemble both commits on its ticket branch and run `make landing-gate` through pm job. Fresh reviewers check each ticket against the combined commit. Both tickets land together. 0001 owns any manifest edit needed by ratchet test tooling.
- Defers: Clippy policies and ratchets belong to 0002. Entity types, transport and the case runner follow the plan. ROADMAP.md boxes change only on the landing candidate; the five-crate box waits until all five exist.
