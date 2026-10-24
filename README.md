# BioMCP

A clean rebuild of BioMCP on the biodata layer. When it is ready, its history replaces the biomcp repository's, keeping that repository's stars and releases.

`make landing-gate` runs formatting, lint, dependency policy and separate instrumented case and library test suites. `make test` runs the ordinary suite for development. The coverage floor counts only the outside-in runner, including CLI and MCP child processes. Permitted library contracts have a separate coverage report and never count toward the floor. The denominator contains product Rust source; test targets, internal test modules, generated output and build scripts are excluded.

The build machine needs `cargo-llvm-cov` and the pinned toolchain's LLVM tools. `make coverage` follows the [external-test sequence](https://github.com/taiki-e/cargo-llvm-cov#get-coverage-of-external-tests) and prints each collection separately. The floor lives in `coverage.floor`; the threshold allows the architecture's counting tolerance. Only the lander raises an established floor. A failed suite has no complete coverage result.

`make unreached` lists missing lines from the existing profiles without rerunning tests. `sh scripts/coverage.sh report` repeats only the concise floor check. These commands require profiles from a successful coverage run in the same checkout. Full reports remain separate from the routine gate.

BioMCP uses platform TLS trust roots. To add a company certificate authority, set `ca_bundle` in the config file to a PEM file. This adds certificates for every source and keeps verification enabled. `biomcp help network ca_bundle` explains this setting.

Set `proxy` in the config file to an HTTP or HTTPS proxy URL to use that route for every source. Otherwise BioMCP reads `HTTPS_PROXY`, `HTTP_PROXY` and `NO_PROXY`. Each proxy variable applies to its own scheme. `NO_PROXY` accepts comma-separated exact hosts or IPs, `.domain`, `*.domain`, optional ports, and `*`. An explicit config proxy overrides these environment settings. `biomcp help network proxy` explains routing.
