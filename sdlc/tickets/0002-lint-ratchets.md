# 0002 — Enforce Rust boundaries and size ratchets

Status: COMPLETE.
Landed: 5b1d2e10dfae711e59c97a07e635e923a38aa65c

Milestone: 1

## Outcome

The routine Cargo test target and per-crate clippy policies reject the structural violations named in the foundation design from the first product commit.

- Starts from: `sdlc/planning/architecture.md` sections 2.1–2.4 and `sdlc/planning/rust-standards.md`; 0001 creates the workspace in parallel.
- Changes: `crates/biodata/clippy.toml`, `crates/biomcp/clippy.toml`, `crates/biomcp/tests/ratchets.rs`, `crates/biomcp/tests/ratchets/**`, `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `rustfmt.toml`, `deny.toml`, `Makefile`, `.gitattributes`, `.gitignore`, `crates/biodata/Cargo.toml`, `crates/biodata/README.md`, `crates/biodata/src/lib.rs`, `crates/biomcp/Cargo.toml`, `crates/biomcp/src/main.rs`, `ROADMAP.md`.
- Keeps: No product behavior or extra crates. Manifests and gate wiring belong to 0001. Use ordinary Rust integration tests without another script language. No credentials or network calls in ratchet tests.
- Scope: Enforce dependency allowlists for the five layers when present, including renamed and target-specific dependencies, and require workspace lint inheritance. Enforce product and adapter file sizes, test placement, reasoned exception locations, forbidden raw serde defaults, Quirk case references, store-only SQLite references and quota declaration confinement and uniqueness when those files exist. Configure no-I/O and mutable-state restrictions in biodata and environment/thread restrictions in the binary. Add other crate policies when those crates arrive. Use bounded parsing that handles ordinary aliases and one policy table per rule.
- Proof: Build and push the disjoint changes, then give the 0001 developer the revision for the combined pre-review gate. Run the same ratchets against isolated temporary trees: a valid tree passes; forbidden dependencies, renamed and target-specific forbidden dependencies, missing lint inheritance and oversized files each fail with the path and rule. Use temporary source changes and real clippy to prove that oversized functions, forbidden biodata I/O and mutable-state types fail. Remove planted changes before the combined gate. Ask the 0001 developer for manifest edits.
- Implementation: Resolve workspace inheritance and package renames for normal, build and target-specific dependencies. Distinguish permitted test dependencies. Use a TOML parser when needed; avoid a general Rust parser. Each existing crate config sets the standards' function threshold and test allowances. Exception locations start empty. Reject dead_code and unused_imports suppressions, including reasoned ones. Verify policies apply when clippy runs from the workspace root.
- Defers: Schema drift, output budgets and case coverage enforcement arrive with their owning stages. ROADMAP.md changes belong only to the landing candidate.
