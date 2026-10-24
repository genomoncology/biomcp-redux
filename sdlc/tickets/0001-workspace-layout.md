# 0001 — Establish the Cargo workspace and landing gate

Status: OPEN.

Milestone: 1

## Outcome

A clean checkout builds the stage-one Cargo workspace and exposes `make landing-gate` as the routine entry point with the workspace lint table inherited by each existing crate.

- Starts from: `sdlc/plan.md` stages 1 and 2, `sdlc/planning/architecture.md` sections 2.1–2.4, and `sdlc/planning/rust-standards.md`. Main has no Rust code.
- Changes: `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `rustfmt.toml`, `deny.toml`, `Makefile`, `.gitattributes`, `.gitignore`, `crates/biodata/Cargo.toml`, `crates/biodata/README.md`, `crates/biodata/src/lib.rs`, `crates/biomcp/Cargo.toml`, `crates/biomcp/src/main.rs`.
- Keeps: Create only biodata and the binary that hosts the ratchet integration target. Add each remaining crate when its stage needs it. Product text uses BioMCP or BioMCP 1.0. Never call sources, embed credentials, or add clinical commands.
- Scope: Pin the exact toolchain, inherit the workspace lint table, and wire formatting, clippy with warnings denied, the ratchet target, ordinary tests and license/advisory checks into the gate. Keep future runner and schema features absent until their stages. Use the smallest executable entry point without fabricated output or unused APIs. Confirm dependency versions and features against primary documentation when introducing them and explain dependencies in commit bodies.
- Proof: Run targeted workspace checks through pm job. Integrate 0002 before the required pre-review `make landing-gate` run. Coordinate branch ownership with its developer. Both tickets land together after a passing combined gate.
- Defers: Clippy policies and ratchets belong to 0002. Entity types, transport and the case runner follow the plan. ROADMAP.md boxes change only on the landing candidate; the five-crate box waits until all five exist.
