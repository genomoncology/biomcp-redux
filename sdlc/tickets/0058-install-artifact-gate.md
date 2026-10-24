# 0058 — Test release artifacts the way users install them

Status: OPEN.

Milestone: 1

## Outcome

A pm job builds each release artifact that 0.9 ships (the PyPI wheel, the install script's binary, the Homebrew formula's binary, the Docker image, the desktop extension bundle from `manifest.json`, and the MCP registry entry's metadata) without publishing anything. It installs each one in a clean container or on a clean machine, and runs the MCP handshake, `tools/list` and one replay case. It checks that the build is a release build, that the version matches, that the embedded schemas and help are present, and that the claimed platform tags match the real glibc floor. 0.9 shipped a debug wheel that overflowed its stack and lost its assets (issues 282 and 287), Docker images that were missing, private or amd64 only, wheels that claimed manylinux 2_28 but needed glibc 2.39, and an install script that broke on stock macOS bash. The 0.9.3 release took eight rehearsals.

- Starts from: milestone 5's release pipeline from 0.9; the 0.9 repo's release scripts; `skills/gh-actions.md` (public releases only on GitHub Actions).
- PM decision: 2026-10-24. The gate runs at the end of milestone 1 and again before every release. Linux x86_64 and arm64 run in containers on our machines. macOS and Windows run on a machine that has them, or wait for the release rehearsal if none does. The ticket says which it used. Two more cases run here: every stdout line from `biomcp serve` over stdio is JSON-RPC, including the first run that creates the cache and config folders; and two processes searching at once on one cache both succeed.
- Proof: the job's report lists each artifact, platform and check with its result. A planted debug-profile wheel is refused.
- Defers: publishing, which needs Ian.

- Reviewed reliability boundary, 2026-10-24: Port minimal local packaging producers because this repo has no wheel/installer/formula/Docker producers yet; do not reuse old project paths blindly. Add one installed-executable selection boundary in the existing launcher for CLI/MCP/process proofs, with no build-tree fallback. Run from empty cwd with isolated HOME/config/cache, cleared credentials/proxy and no checkout mounted; embedded help/schema must work. Use only local release artifacts for installer/formula and offline wheel install. Bind release provenance to producer configuration and candidate bytes, not speed/name/size; planted debug wheel uses the same inspection/install path. Check ELF symbol requirements and execute at the declared manylinux floor. Execute each container architecture, not just manifest inspection. Exercise stdout throughout MCP lifecycle. Reuse existing two-process publication proof with installed processes, shared ordinary cache and event overlap, not fixed sleeps. Report unavailable platform execution as deferred. No publishing or remote registry/formula changes.

## Changes

`release/**`, `Makefile`, `README.md`, `crates/biomcp/tests/cases.rs`, `crates/biomcp/tests/runner/launch.rs`, `crates/biomcp/tests/runner/mod.rs`, `crates/biomcp/tests/runner/configuration.rs`, `crates/biomcp/tests/runner/publication.rs`, `crates/biomcp/tests/runner/README.md`.
