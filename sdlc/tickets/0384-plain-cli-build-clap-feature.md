# 0384 — Restore the plain binary build and check it in the landing gate

Status: OPEN.

Milestone: 5

## Outcome

`cargo build --locked -p biomcp --bin biomcp` succeeds on main. The `biomcp` crate declares the clap `error-context` feature it uses in `grammar/errors.rs`, and the landing gate builds the binary alone, without test or dev feature unification, so a missing feature fails landing.

- Why: landed 4f70ac15 fails the plain binary build with E0432 and E0599, because `grammar/errors.rs` uses clap `ContextKind`, `ContextValue` and `error.get()` while `error-context` is off; routine test builds passed because their feature union turned it on (coordinator m10791 ask 6, 2026-10-27). The release build uses the plain path.
- Starts from: `crates/biomcp/Cargo.toml`, `grammar/errors.rs` and the Makefile's `landing-gate`.
- Rules: declare the feature at the crate that uses it; no workspace-wide feature change.
- Proof: the plain binary build passes; with the feature removed, the new gate step fails.
- Changes: `crates/biomcp/Cargo.toml` and the Makefile.
- Defers: nothing. Urgent: main's release path is broken until this lands.
