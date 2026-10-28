# 0403 — Make `help schema` work

Status: OPEN.

Milestone: 5

## Outcome

`biomcp help schema` prints the help for the `schema` command, like every other command's help entry: what `biomcp schema` lists, how `biomcp schema <id>` prints one schema, and an example. `biomcp help` lists it.

- Why: an outside agent found that `biomcp help schema` fails at main 88294ae1 while `biomcp schema` works (Jarvis m12594, 2026-10-28). Ian's direction is that the CLI tells an agent ahead of time what JSON it will emit.
- Starts from: the help registry, the `schema` command grammar and 0357's help completeness check.
- Rules: no new framework; the existing completeness check covers every top-level command.
- Proof: a case failed before the fix for `biomcp help schema`; the completeness check fails when a top-level command has no help entry.
- Changes: the help entry, its case and the completeness check if it missed this command.
- Defers: schema validation and field-path listing, which are new capabilities after 1.0 unless Ian moves them into 1.0.
