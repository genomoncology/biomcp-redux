# 0016 — Enforce coverage from the owning outside-in suites

Status: OPEN.

Milestone: 1

## Outcome

The landing gate measures product-source line coverage from the owning binary and public-library suites, includes spawned CLI and MCP processes, and refuses a drop beyond the recorded floor tolerance.

Depends on: 0015
Depends on: 0014

- Starts from: architecture 10.3–10.4 and 11; Makefile landing-gate/test; runner launch::Environment::command and current MCP process launch callers; Cargo workspace test targets and public library contracts.
- Changes: `Makefile`, `coverage.floor`, `scripts/coverage.sh`, `crates/biomcp/tests/runner/launch.rs`, `crates/biomcp/tests/runner/proof.rs`, `README.md`.
- PM decision: 2026-10-24, m4284. Measure the binary YAML/MCP cases plus permanent public library contracts permitted by architecture 10.4. Measure product source only; exclude test targets, generated files and build scripts. One instrumented owning-suite run replaces the ordinary full test run inside the landing gate. Do not duplicate the suite or build a coverage framework.
- Scope: Check cargo-llvm-cov and the pinned toolchain LLVM tools through pm job before implementation. Follow its documented external-test sequence: obtain cargo llvm-cov show-env --sh, clean coverage artifacts, run ordinary cargo test --workspace --locked once in that environment, then cargo llvm-cov report --fail-under-lines. Do not call another instrumentation/test subcommand inside that environment. Use an absolute LLVM_PROFILE_FILE pattern containing binary and process identity; preserve that variable through Environment::command for nested runner and product launches. Keep every other isolation rule. One shared filename-filter setting excludes tests, generated output and build scripts from reports and thresholds; inspect the actual file list. Establish coverage.floor from the measured baseline; missing, malformed or nonfinite percentages fail. Apply architecture 10.3 tolerance of 0.2 percentage points to the native threshold. Only the lander raises an established floor; routine runs never initialize or lower it. make unreached reports missing lines from existing profiles without rerunning tests.
- Keeps: Existing routine behavior, test discovery and gate checks remain. No product environment hook, live provider call, new production dependency, coverage-driven implementation rewrite or test duplicated solely to raise the percentage. Keep make test available for targeted development; landing-gate runs the suite once under instrumentation. Installation or machine changes go to the machine owner when required.
- Proof: Through pm job, run final make landing-gate, invoking make coverage in place of the ordinary full test prerequisite. Inspect that same run report for nonzero spawned schema CLI/MCP dispatcher and public-library source execution and excluded test/generated/build-script paths. Exercise an impossible threshold through the same report invocation without another suite run, then restore the measured floor. Extend the existing runner isolation proof to check profiling across nested launches while hostile credential/proxy variables remain absent. Do not mutate the test process global environment. Cite concise job evidence; commit no report.
- Ownership: Start development after 0015 lands because it owns launch.rs. Prepare the design in parallel; use the latest branch for actual caller names. No second writer on the MCP branch.
- Defers: New cases belong to each behavior ticket. Release artifact coverage and performance timing remain in their existing release/performance work.
