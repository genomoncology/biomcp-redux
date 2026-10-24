# 0016 — Enforce coverage from the owning outside-in suites

Status: OPEN.

Milestone: 1

## Outcome

The landing gate measures product-source line coverage from the owning binary and public-library suites, includes spawned CLI and MCP processes, and refuses a drop beyond the recorded floor tolerance.

Depends on: 0015
Depends on: 0014

- Starts from: architecture 10.3–10.4 and 11; Makefile landing-gate/test; runner launch::Environment::command and current MCP process launch callers; Cargo workspace test targets and public library contracts.
- Changes: `Makefile`, `coverage.floor`, `scripts/coverage.sh`, `crates/biomcp/tests/runner/launch.rs`, `README.md`.
- PM decision: 2026-10-24, m4284. Measure the binary YAML/MCP cases plus permanent public library contracts permitted by architecture 10.4. Measure product source only; exclude test targets, generated files and build scripts. One instrumented owning-suite run replaces the ordinary full test run inside the landing gate. Do not duplicate the suite or build a coverage framework.
- Scope: Use cargo llvm-cov and its native report/failure controls, checking configured-machine availability through pm job before choosing the invocation. Preserve subprocess profiles through a narrow test-launch whitelist for LLVM_PROFILE_FILE and only other demonstrated-required instrumentation variables. Retain cleared credentials, proxy, home and cache isolation. Use collision-safe per-process profiles and report only product-source coverage. Establish coverage.floor from the measured current baseline, with the architecture 10.3 tolerance of 0.2 percentage points. Only a lander raises an established floor. No automatic lowering or silent reset. Keep routine output concise; make unreached provides source-line detail on demand. Correct the Changes claim if a small launch helper or script name is required.
- Keeps: Existing routine behavior, test discovery and gate checks remain. No product environment hook, live provider call, new production dependency, coverage-driven implementation rewrite or test duplicated solely to raise the percentage. Keep make test available for targeted development; landing-gate runs the suite once under instrumentation. Installation or machine changes go to the machine owner when required.
- Proof: Through pm job, run make coverage and make landing-gate on the final branch. Show spawned schema CLI and MCP dispatcher source lines contribute to the merged result, public library source contributes, and test/generated/build-script paths do not enter its denominator. A temporary impossible floor must fail the real coverage command; restore the measured floor before commit. Cite concise job evidence rather than checking in a report. An isolated launch check proves instrumentation survives while unrelated hostile environment remains absent.
- Ownership: Start development after 0015 lands because it owns launch.rs. Prepare the design in parallel; use the latest branch for actual caller names. No second writer on the MCP branch.
- Defers: New cases belong to each behavior ticket. Release artifact coverage and performance timing remain in their existing release/performance work.
