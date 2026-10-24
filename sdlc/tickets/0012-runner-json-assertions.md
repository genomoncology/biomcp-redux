# 0012 — Complete the runner's JSON assertion vocabulary

Status: OPEN.

Milestone: 1

## Outcome

YAML cases through the existing runner express the architecture's remaining JSON assertions and wildcard paths, with precise failures for missing data and incorrect answers.

Depends on: 0010

- Starts from: `sdlc/plan.md` stage 5; architecture section 10.1; the current cases target, runner assertion/launch helpers and real schema-command cases. Reuse equality, output ceilings, path naming and filter behavior from 0010.
- Changes: `crates/biomcp/tests/runner/**`, `crates/biomcp/tests/cases.rs`, `tests/cases/schema/**`, `crates/biomcp/Cargo.toml`, `Cargo.toml`, `Cargo.lock`, `deny.toml`.
- Scope: Add subset, includes, includes_all_of from a floor file, count ranges, rank_of ranges, before, unique and regular-expression assertions; add architecture wildcard path traversal over arrays. Keep one assertion dispatcher with small focused helpers. Validate assertion shapes before executing a case, reject unsupported/unknown operators, and preserve a pointer/expected/actual diagnostic. Floor paths resolve relative to the case file under the explicit external --cases contract, without assuming the repo root. Reuse byte/token ceilings over compact serialized JSON.
- Keeps: Prove assertions through the real schema-printing command, using schema enum arrays and objects already present rather than fake product outputs. No extra product command, broad query language, MCP/replay/page behavior or generic testing framework. Missing pointers differ from null, empty wildcard results cannot silently satisfy a required assertion, and case state/filter/name behavior stays unchanged. Add a regex dependency only if the existing workspace has no reusable parser; verify current release/license and record the code it saves.
- Ownership: The cache lane 0011 owns root dependency declarations/lock if it is running concurrently. Coordinate any regex manifest needs through the coordinator; this lane owns binary test dependencies and assertion files.
- Proof: Extend real YAML cases to exercise each successful assertion, then invoke the actual runner on selected isolated negative cases for each distinct failure class, including wildcard misses, malformed assertion shapes, invalid regex and floor-file errors. Verify the named case failed with a readable diagnostic, not zero selected tests. Keep boundary forms in tables and avoid one separate helper test per operator when command proof covers it. Run make landing-gate through pm job after shared dependency integration and current main.
- Defers: Following next steps, full walks, replay/offline configuration, MCP parity, schema validation of envelope responses and coverage/surface checks arrive with the surfaces they exercise. No complete-runner roadmap tick yet.
