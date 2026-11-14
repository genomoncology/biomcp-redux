# 0937 — skill-reads-totals-and-uses-pair-lookup

Status: COMPLETE.
Landed: 22699a19dfee0a453fe4d48f4042edc99a0b7e26

Milestone: 7

## Outcome

The BioMCP skill tells agents to read a page's total before reporting a count or a list as complete, to use the DDInter pair lookup for a named pair, to ask for several sections in one get, and to stop after a missing-key refusal. Replies that are one page of more already say so; check the DDInter and CPIC ones do so early in the reply.

## Why

The persona reported "5 interaction records" for clopidogrel from DDInter's first page (DDInter holds 614 partners; omeprazole and esomeprazole are Major) and CPIC's first page as the whole (log.md:254-261); it retried a no-key NCI search four times (log.md:147) and read three sections as five calls (log.md:61-63). Evidence: bench `personas/dx-lab-director/runs/2026-11-14/` (j40302, BioMCP f4f726674) and its code-mode pair `runs/2026-11-14-raw/` (j40303); the analyst's source checks are listed at the end of `analysis.md`. This is the paper's W2 (narrow replies read as complete).

## Proof

- Proof: the skill's text names each step; a DDInter drug reply shows its total before its first row (measure where it sits today).

## Starts from

- Starts from: the skill in `skills/` and 0922's command map; the DDInter reply shape in `crates/biomcp-adapters/src/ddinter/` (or its query crate).

## Keeps

- Keeps: 0922's try-before-help guidance; skill size within its cap.

## Defers

- Defers: nothing.

## Changes

- Changes: the skill text; reply field order if the total sits late.

## Progress

- Measured on origin/main 31dc844d1 (job j40447, after `sync ddinter`): `get drug clopidogrel interactions` is 5,462 bytes; its first row starts at byte 445 and the interactions `total` (614) at byte 4,822, after all five rows. The only `total` inside the persona's `head -c 1200` was the name-resolution total (1) at byte 290, which reads as "one result". The envelope's own `page` block sits at byte 5,148. CPIC (job j40428): `search pgx --gene CYP2C19 --source cpic` puts `page.total` (27) at byte 2,189 of 2,639; `get pgx recommendations --drug-id RxNorm:32968 --gene CYP2C19` puts it (24) at byte 5,101 of 5,429.
- Changed reply field order only (no field added, removed or renamed; schemas list properties sorted, so they do not change): `DrugRecords` now writes `offset`, `total` and `truncated` before `records` (every drug section page, DDInter included), and every CPIC/ClinPGx collection writes `page` before `items`. After (jobs j40448, j40434 on a4f973984): the DDInter total 614 sits at byte 456, before the first row at 468, and `head -c 1200` shows it; CPIC search `page.total` at byte 200, recommendations at byte 305. MCP text replies are serialized through a sorted JSON value, so their order is alphabetical and unchanged; MCP agents get the whole reply.
- Checked live: `get drug clopidogrel interactions --source ddinter --partner omeprazole` returns the one pair (total 1). `--partner` needs `--source ddinter` and only the `interactions` section.
- Skill (`skills/SKILL.md`, 3,857 bytes of its 4,096 cap): where a get takes several sections, asks for all needed in one get (pgx, adverse-event, gwas, cell-line and author take one section, per review); says a reply holds one page and to read its `total` before reporting a count or calling a list complete; names the DDInter pair lookup with that command; says a `no-key` reply will not change on retry. `tests/cases/local/skill.yaml` asserts each line. Contract tests: `interaction_page_states_its_total_before_its_rows` (biomcp-types tests/response.rs; drug_contracts.rs is at its 600-line limit) and `pgx_pages_state_their_total_before_their_rows` (pgx_contracts.rs; covers the recommendations view and the `search pgx` page).
- Shared with 0936 (CPIC phenotype filter): `crates/biomcp-types/src/pgx/response.rs` (this ticket only moves `page` above `items`) and `crates/biomcp-types/tests/pgx_contracts.rs` (one appended test). Whichever lands second merges the other.
- Not done here: the envelope's top-level `page` block still follows `result` in every reply; moving it is an envelope-wide change outside this ticket.
