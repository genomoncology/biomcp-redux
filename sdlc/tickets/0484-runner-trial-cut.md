# 0484 — Keep only the runner trials a command line cannot express

Status: OPEN.

Milestone: 5

Depends on: 0456

## Outcome

The runner keeps about 150 Rust trials, for behavior a command-line call cannot show: MCP framing, HTTP and streaming, TLS, cache contention, the runner's own checks, and the few calls into internal functions that have a stated reason. The other trials go, and so does their helper code.

- Why: Ian's ruling in `decisions/2026-10-29-one-small-outside-in-suite.md`. About 1,320 Rust trials and 351 runner files with 74,000 lines sit beside the cases. About 1,170 of the trials launch the binary with an argv, so they repeat what a case does. 247 only check retired-command refusals, 130 of them for `study` alone.
- Starts from: `crates/biomcp/tests/runner/` and the trial registry in `cases.rs`.
- Rules: a trial stays only when no command-line case can show its behavior, and its name says which kind it is. A retired command keeps one refusal case in YAML. Helper files that no kept trial uses go. Add no new trial framework. Every shared Rust output ceiling that survives the cut moves to YAML, value for value, so output budgets live only in YAML (from 0431, m15432).
- Proof: the trial count and runner line count are reported before and after. Each kept trial names the reason a case cannot replace it. 0483's check still passes. The reply diff is empty.
- Changes: the runner and the trial registry.
- Defers: nothing.
