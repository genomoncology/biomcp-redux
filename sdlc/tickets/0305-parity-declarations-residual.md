# 0305 — Declare the residual direct parity plans for 0.9.3

Status: OPEN.

Milestone: 5

Depends on: 0298

## Outcome

The six residual direct cases outside trial, pathway and 0300 to 0304 have reviewed 0.9.3 declarations or a source-grounded unsupported reason, and the dry inventory shows no direct case without an owner or a plan. The six are three drug error cases and one case each for schema, cell line and adverse event.

- Why: the 0298 residual audit found these six after the family split, including `cell-line/aliases.yaml`, whose query limit of 5 projects because the released `cli/cell_line/dispatch.rs` lines 65 to 69 serialize `/pagination/limit`. PM ruling 2026-10-27 (m8126 ask 42) gives them one owner in the parallel family batch, so neither the core nor a whole-question exclusion absorbs them.
- Starts from: the landed 0298 endpoint catalog, inventory and guards, the 0.9.3 release source and binary, and the six harvested cases.
- Rules: released 0.9.3 is the baseline. Each asserted fact is either declared or listed unsupported with the 0.9.3 source line that shows why; projectable metadata is never hidden by excluding a whole question. No live calls.
- Proof: the dry inventory reports zero pending residual gaps, and a reviewer checks each of the six against 0.9.3 source.
- Changes: these six declarations only.
- Defers: the whole run, which is 0297.
