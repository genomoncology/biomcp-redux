# 0959 — cdx-help-names-shared-row-fields

Status: COMPLETE.
Landed: 82dfaffc807ff42444cfef661f23aaf56e008cde

Milestone: 7

## Outcome

`help search diagnostic` (or the companion-diagnostic reply itself) tells a reader that each FDA CDx row carries the shared fields `diagnostic`, `indication`, `device`, `drug` and `row_date` beside FDA's raw `cells`, so scripts read those instead of FDA's column names.

## Why

dx-lab-director round 3 (bench 0050, 2026-11-14): the agent never found 0933's shared fields and read FDA's raw column names in both spellings (`"Diagnostic Name\n(Manufacturer)"`), calls 33, 35, 36; its report asked for exactly what 0933 already added. Evidence: bench `personas/dx-lab-director/runs/2026-11-14-r3/analysis.md`, log.md:42.

## Proof

- Proof: the help (or reply) text names the fields, checked in the diagnostic help case within the measured-size-plus-15% cap.

## Starts from

- Starts from: 0933's ticket and the CDx row type; `tests/cases/diagnostic/help.yaml`.

## Keeps

- Keeps: the reply schema (description changes only).

## Defers

- Defers: nothing.

## Changes

- Changes: diagnostic help text or a schema description, the help case, regenerated docs.

## Progress

- 2026-11-14, evidence read: in round 3 the agent read the whole `help search diagnostic` page (call 8). It saw the flat `Fields:` list, which named `diagnostic`, `indication`, `device`, `drug` and `row_date` but did not say they join FDA's two tables, so it read raw `cells` keys in both spellings (calls 33, 35, 36).
- Changed: `help search diagnostic` now says, right after its first sentence, "FDA CDx rows of both tables carry diagnostic, indication, device, drug and row_date beside FDA's cells, whose headers differ by table." It is added by help (`crates/biomcp/src/help.rs`), not the command summary: a first try in the summary also reached the typed `search_diagnostic` tool description, which then measured 2,500 characters against its 2,400 budget (gate j41040), and changed `request.json`. The reply and request schemas are unchanged.
- Measured (j41028): `help search diagnostic` is 8,561 bytes, so the help case cap moves from 9,649 to 9,846 (measured plus 15%). `tests/cases/diagnostic/help.yaml` asserts the new description; `docs/reference/commands.md` regenerated.
