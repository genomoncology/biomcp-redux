# Measured entity caps need no PM ask

Date: 2026-10-26

Decided by the PM.

A new or changed entity help cap, command or section step ceiling, or Markdown cap is set at its measured size plus 15 percent, rounded up, without a PM ask. The developer records the measurement job and the new cap in the owning ticket's progress, and the reviewer checks it.

- A PM ruling is still needed when a measurement exceeds a global cap: whole command tree help, the `biomcp` MCP description or the full `tools/list` reply. It is also needed when a measurement exceeds a design allocation by more than 50 percent, or when content is removed to fit.
- This replaces the rule in slice docs tickets that an entity help cap increase needs a PM ruling.

## Whole help ceiling raised for 0288

PM ruling 2026-10-26 (m8126 ask 19): the whole command tree help ceiling rises from 91,228 to 106,000 bytes. 0288 measured 91,961 bytes before the restored client aliases; 106,000 is the round figure just above measured plus 15 percent. The final alias help must fit it, and owning command caps stay at their exact measured size plus 15 percent.

## search_diagnostic action budget for 0964

PM ruling 2026-11-15: `search_diagnostic` gets its own typed action budget of 2,800 characters, beside `search_trial`'s 3,500, in `crates/biomcp/tests/runner/typed_schemas.rs`. It serves five FDA and NCBI sources with their own filters, and 0964's `condition` filter measured 2,432 against the ordinary 2,400 (j41241) with its description already one line. 2,800 is measured plus 15 percent, rounded up. 0901 had already hidden `firm` from the listing to stay under 2,400; that stays. Other actions keep 2,400, and the 128 KiB all-tools cap still guards clients that load every tool. The next filter that passes 2,800 needs a new ruling.
