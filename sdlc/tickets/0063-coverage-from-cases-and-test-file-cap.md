# 0063 — Measure coverage from the cases and cap test file length

Status: OPEN.

Milestone: 1

## Outcome

`make coverage` measures product-code coverage from the outside-in runner cases alone, and `coverage.floor` holds that number, so it says how much code real commands reach and compares fairly with 0.9's figure. The library tests that architecture 10.4 allows are reported separately and never count toward the floor. The ratchet test applies the 600-line file cap to test files under `crates/*/tests/` as well as product code. `biomcp-fetch/tests/bundle.rs` (846 lines) and `cache.rs` (750) get split along their behaviors.

- Starts from: `Makefile` coverage target; `coverage.floor`; architecture 10.3, 10.4 and 11; the ratchet test's file-length rule.
- PM decision: 2026-10-24. The new floor is the measured cases-only number on main when this lands, even though it is lower than 96.27, and it only rises from there. A drop in cases-only coverage that the library tests hide is exactly what this change exposes.
- Proof: the coverage report prints the cases-only and library-test numbers. The gate fails when cases-only coverage drops below the floor. A planted 601-line test file is refused by the ratchet.
- Defers: nothing.
