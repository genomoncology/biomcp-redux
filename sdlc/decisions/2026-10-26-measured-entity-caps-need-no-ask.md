# Measured entity caps need no PM ask

Date: 2026-10-26

Decided by the PM.

A new or changed entity help cap, command or section step ceiling, or Markdown cap is set at its measured size plus 15 percent, rounded up, without a PM ask. The developer records the measurement job and the new cap in the owning ticket's progress, and the reviewer checks it.

- A PM ruling is still needed when a measurement exceeds a global cap: whole command tree help, the `biomcp` MCP description or the full `tools/list` reply. It is also needed when a measurement exceeds a design allocation by more than 50 percent, or when content is removed to fit.
- This replaces the rule in slice docs tickets that an entity help cap increase needs a PM ruling.

## Whole help ceiling raised for 0288

PM ruling 2026-10-26 (m8126 ask 19): the whole command tree help ceiling rises from 91,228 to 106,000 bytes. 0288 measured 91,961 bytes before the restored client aliases; 106,000 is the round figure just above measured plus 15 percent. The final alias help must fit it, and owning command caps stay at their exact measured size plus 15 percent.
