# BioMCP

A clean rebuild of BioMCP on the biodata layer. When it is ready, its history replaces the biomcp repository's, keeping that repository's stars and releases.

`make landing-gate` runs formatting, lint, dependency policy and one instrumented workspace test suite. `make test` runs the ordinary suite for development. Coverage includes the CLI and MCP child processes and the public library contracts. The denominator contains product Rust source; test targets, generated output and build scripts are excluded.

The build machine needs `cargo-llvm-cov` and the pinned toolchain's LLVM tools. `make coverage` follows the [external-test sequence](https://github.com/taiki-e/cargo-llvm-cov#get-coverage-of-external-tests) and prints the total. The floor lives in `coverage.floor`; the threshold allows the architecture's counting tolerance. Only the lander raises an established floor. A failed suite has no complete coverage result.

`make unreached` lists missing lines from the existing profiles without rerunning tests. `sh scripts/coverage.sh report` repeats only the concise floor check. These commands require profiles from a successful coverage run in the same checkout. Full reports remain separate from the routine gate.

`biomcp health` prints the implemented sources, publisher terms, enabled state and bounded source reachability. It reports missing keys and disabled sources as skipped. It does not synchronize local bundles. `--disable-sources pubmed,restricted` disables individual source names or the `restricted` and `noncommercial` groups. The same setting lives in the JSON configuration as `"disable_sources": ["pubmed", "restricted"]`. Configuration and command settings combine by union; a command cannot enable a source disabled by configuration. Disabled sources cannot supply cached replies or restore stored article pools. Publisher terms remain authoritative, and the source category grants no rights in individual records.
