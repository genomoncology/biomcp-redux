# 0390 — Let the single-source test stub take a capturing callback

Status: OPEN.

Milestone: 5

## Outcome

`Stub::for_source` accepts the same capturing reply callback as `Stub::for_sources`, an `impl FnMut(&str) -> Result<String, String> + Send + 'static`, and delegates to it with its fixed host and content type. Existing callers that pass a function pointer compile unchanged.

- Why: the coordinator's RCA (m12089, 2026-10-28) found three test setup compile failures with the same cause, 0371 j19380, 0372 j19381 and 0375 j19395. `for_sources` was widened to capturing closures at 80ab2601f3, but `for_source` still takes a function pointer, and 69 runner files use the single-source form. Each failure cost a full build.
- Starts from: the `Stub` test helper and `for_sources`.
- Rules: one method change. No new test framework, rule, guard or docs.
- Proof: a test passes a capturing closure to `for_source`; the existing suite compiles and passes unchanged.
- Changes: the `Stub` helper and one test.
- Defers: nothing. It blocks no lane; the three lanes already use `for_sources`.
