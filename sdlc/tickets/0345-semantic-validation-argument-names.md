# 0345 — Name the argument in semantic query validation errors

Status: OPEN.

Milestone: 5

Depends on: 0339

## Outcome

When a query passes argument parsing but fails a later typed or cross-field check, `usage.arguments` names the argument or arguments involved, through one shared typed cause that every entity's query validation returns, never by guessing from text.

- Why: 0339 names arguments for parsing, missing values and integer conversion, but the shared `QueryInvalid` cause carries no argument identity, so later semantic checks still return fixed guidance (coordinator m10137, PM ruling 2026-10-27).
- Starts from: the shared query validation error, each entity's query constructors and 0339's argument naming.
- Rules: name arguments, never echo values, as 0013 and 0025 say. One shared cause boundary; no per-command duplicate validation.
- Proof: cases for a bad typed ID, a cross-field conflict and a range check each name the right argument.
- Changes: shared query validation types, entity query constructors and cases.
- Defers: nothing. It ships in 1.0.
