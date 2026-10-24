# 0012 — Validate and extend the runner assertions its cases use

Status: COMPLETE.
Landed: 18bc8b5986efe1db0649ff460034032b691c57fd

Milestone: 1

## Outcome

Real schema-command cases use a validated, unambiguous assertion syntax with array membership, exact counts/ranks and wildcard paths, while unused operators remain absent.

Depends on: 0010

- Starts from: `sdlc/plan.md` stage 5; architecture section 10.1; the current cases target, runner assertion/launch helpers and real schema-command cases. Reuse equality, output ceilings, path naming and filter behavior from 0010.
- Changes: `crates/biomcp/tests/runner/**`, `crates/biomcp/tests/cases.rs`, `tests/cases/schema/**`.
- PM decision: 2026-10-24. Build only operators a case in this ticket uses. Retain equals with scalar/array shorthand and max_bytes/max_tokens; add includes, exact count and exact rank_of through real schema cases. Defer subset, includes_all_of, before, unique, regex and general ranges until their first harvested case needs them. Record the full approved finite syntax in the runner README for those later tickets.
- Scope: Assertion objects contain known operators combined with AND; literal object equality uses equals. Validate every step and operator before running any command, rejecting unknown names and bad shapes. A wildcard expands arrays in order into one result array; every selected branch must have the remaining path and the right container. Reject zero-result wildcard selection. Missing paths fail while explicit null remains a value. includes requires an equal array member, count is exact nonnegative array length, and rank_of maps string members to their first one-based exact ranks; absent members fail. Empty ordinary arrays may equal [], count zero and fail membership normally. Keep one small dispatcher and readable pointer/expected/actual diagnostics.
- Keeps: Prove assertions through the real schema-printing command, using schema enum arrays and objects already present rather than fake product outputs. No extra product command, broad query language, MCP/replay/page behavior or generic testing framework. Missing pointers differ from null, empty wildcard results cannot silently satisfy a required assertion, and case state/filter/name behavior stays unchanged. Add no dependency for this bounded assertion set.
- Proof: Use schema biodata.source/1 /enum and /enum/* for membership/count/rank/wildcard success. Use actual nested schema objects with equals. Extend the existing actual-runner negative table for unknown operators, malformed exact counts/ranks, wrong actual types, missing rank members, wildcard misses and whole-output/pointer ceilings. Verify the named selected case failed, not zero tests. Only unavailable null/empty traversal boundaries may use one small internal table; add no fake product command or schema. Keep existing filter/name/isolation cases. Run make landing-gate through pm job after current main.
- Defers: Unused approved operators/ranges wait for real harvested cases. The README records their already reviewed semantics without implementing them. Following next steps, full walks, replay/offline configuration, MCP parity, schema validation of envelope responses and coverage/surface checks arrive with the surfaces they exercise. No complete-runner roadmap tick yet.
