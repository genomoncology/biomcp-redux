# 0357 — Complete the help reference

Status: OPEN.

Milestone: 5

Depends on: 0338, 0339

## Outcome

`biomcp help` gives each argument its default, a non-empty value type and its aliases such as `--query/-q`, and every command has at least one example that returns results against its recording. A help topic and a SKILL.md section list exit codes 0 to 5 and the error codes. `biomcp help` lists the network topics that replies point to, such as `help network proxy` and `help network ca_bundle`. A short top-level index names each command family with one line, and a relation map shows which entity leads to which. Markdown help orders columns Name, Description, then the rest, prints examples as command lines and the command name as a heading. Help wording drops jargon such as "native", "provider" and "literal" where a plain word serves, `health` says `use` like every other reply, and the skill text says "reply".

- Why: the docs lead's product findings report (2026-10-27), items B5, B7, B8 and B9, and the docs lead's second-pass product findings (2026-10-27), item G10. Ian's ruling that 1.0 ships every known fix (2026-10-27).
- Starts from: the help artifact builder, the Markdown renderer's help layout, `crates/biomcp/src/exit.rs`, SKILL.md and the help example runner.
- Rules: administrative examples (key, store, config) run for real in an isolated fake home; `serve` keeps its real example, proved by server lifecycle tests, and is the only command the example runner checks through scoped help (PM ruling m10791 ask 9). Wording changes keep each description's meaning; the whole-help ceiling of 106,000 bytes holds, raised only by a measured caps amendment. The `article text PMC5606870 --variant Pro245Ser` example is replaced or fixed so it returns results.
- Proof: every option a reply's next step suggests, such as `--fresh` from `get phenotype` (experiment 499), appears in that command's help; a check over `biomcp help` fails on a missing default where one exists, an empty value type or a command without an example; every example runs against recordings with results; the exit code topic matches `exit.rs`; Markdown help snapshots show the new layout.
- Changes: help text, the help builder, the Markdown help renderer, SKILL.md, help examples and cases.
- Defers: nothing.
