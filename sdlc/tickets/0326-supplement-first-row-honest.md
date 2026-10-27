# 0326 — Stop calling a spreadsheet's first row its header

Status: OPEN.

Milestone: 5

## Outcome

A supplement spreadsheet hit returns the sheet's first row as `first_row` with its original row number, never labeled as column headers, and offers a next step that reads the top rows of that sheet with `--rows`, so an agent can see the real column headings.

- Why: experiment 497 (ticket 0325, evidence on the evidence store under 0325, PM evaluation 2026-10-27 for Jarvis m9494) found a matching row whose `header` was a merged caption in row 1 while the real column headings sat in row 2, so the hit's numeric cells could not be read safely. `crates/biomcp-query/src/article/supplement/page.rs` lines 127 to 132 take row 1 as the header unconditionally.
- Starts from: `article/supplement/page.rs` `collect`, the supplement response type and schema, and the existing `--rows` range option.
- Rules: no header detection by guessing; the row keeps its original number and the cells stay unchanged. The next step names the exact sheet and a bounded row range from 1.
- Proof: a replay case with a caption in row 1 and headings in row 2 shows `first_row` with number 1, the hit row number, and a next step whose `--rows` read returns rows 1 to the bound; the schema and caps pass.
- Changes: the supplement page, its response type, schema, next step and cases.
- Defers: nothing.
