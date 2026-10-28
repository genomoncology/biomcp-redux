# 0391 — Pin release tools in one manifest and rehearse on a cold host

Status: OPEN.

Milestone: 5

## Outcome

One tracked manifest pins every release tool and version: Rust, rustup, Python, cargo-deny, the secret scanner and any other tool a release stage installs. The release workflow, the Linux container recipe, the native matrix recipe and `rust-toolchain.toml` all read it or are checked against it. A cold-host rehearsal runs as a `pm job` in a fresh container with an empty HOME and no inherited toolchain or Cargo cache. It installs from the manifest, runs every release stage even after one fails and reports every failure in one pass.

- Why: the coordinator's 0336 preparation (m12181, 2026-10-28) found Rust 1.95.0 repeated in four places, rustup pinned only in `linux.sh`, Python versions that diverge, separate cargo-deny and scanner pins, a Linux recipe that keeps its Cargo cache, and shell stages that stop at the first failure. The release rules require one manifest and a cold-host rehearsal before the hosted run.
- Starts from: `.github/workflows/release.yml`, `release/` scripts including `linux.sh` and `matrix.sh`, `rust-toolchain.toml` and the Makefile release targets.
- Rules: reuse the existing release commands and the five-platform native matrix; no new framework. Native installed proof stays separate from the Linux container proof; the cold-host rehearsal covers the Linux stages and the native stages run as today on the hosted matrix. The rehearsal makes no provider calls and publishes nothing.
- Proof: a check fails when any recipe pins a tool version that differs from the manifest; one cold-host rehearsal job passes every stage, after a seeded failure showed that later stages still ran and every failure was reported.
- Changes: the manifest, the workflow and release recipes that read it, one check and the rehearsal target.
- Defers: nothing. The installed-CLI local TLS gRPC proof for AlphaGenome joins the 0293 final rehearsal through the existing `alphagenome_tls.rs` recipe, not this ticket.
