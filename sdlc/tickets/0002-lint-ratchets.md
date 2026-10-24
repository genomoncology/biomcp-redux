# 0002 — Enforce Rust boundaries and size ratchets

Status: OPEN.

Milestone: 1

## Outcome

The routine Cargo test target and per-crate clippy policies reject the structural violations named in the foundation design from the first product commit.

- Starts from: `sdlc/planning/architecture.md` sections 2.1–2.4 and `sdlc/planning/rust-standards.md`; 0001 creates the workspace in parallel.
- Changes: `crates/biodata/clippy.toml`, `crates/biomcp/clippy.toml`, `crates/biomcp/tests/ratchets.rs`, `crates/biomcp/tests/ratchets/**`.
- Keeps: No product behavior or extra crates. Manifests and gate wiring belong to 0001. Use ordinary Rust integration tests without another script language. No credentials or network calls in ratchet tests.
- Scope: Enforce dependency allowlists for the five layers when present, including renamed and target-specific dependencies, and require workspace lint inheritance. Enforce product and adapter file sizes, test placement, reasoned exception locations, forbidden raw serde defaults, Quirk case references, store-only SQLite references and quota declaration confinement and uniqueness when those files exist. Configure no-I/O and mutable-state restrictions in biodata and environment/thread restrictions in the binary. Add other crate policies when those crates arrive. Use bounded parsing that handles ordinary aliases and one policy table per rule.
- Proof: Integrate 0001 and run `make landing-gate` through pm job before review. Exercise representative invalid temporary workspace fixtures for dependency direction, size and policy restrictions. Keep fixtures isolated. Ask 0001's developer to change its files rather than writing its branch.
- Defers: Schema drift, output budgets and case coverage enforcement arrive with their owning stages. ROADMAP.md changes belong only to the landing candidate.
