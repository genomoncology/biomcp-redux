# 0317 — Page trial eligibility text at line boundaries and mark a split criterion

Status: OPEN.

Milestone: 5

## Outcome

A page of trial eligibility text ends at a line break in the registry text, so a criterion and its nested exceptions stay together. When a single line is longer than the page budget, the page says the criterion continues on the next page.

- Why: an investigation team testing 1.0 on a real screening question found that the eligibility page for one trial stopped midway through a hepatitis B exception, so an agent reading only that page could exclude a patient wrongly; following the continuation recovered the full text. `crates/biomcp-query/src/trial/get/page.rs` lines 91 to 103 cut the text at a 6,000-byte character budget wherever it falls, which breaks trial slice ruling 6: a criterion is never split.
- Starts from: `trial/get/page.rs`, `TrialEligibilityPage`, trial slice ruling 6 in `2026-10-24-trial-slice-rulings.md`, and the trial eligibility cases.
- Rules: the boundary is structural, the last line break that fits the budget, never a recognized English heading or word. A nested exception on its own indented line stays with its parent when both fit; when they cannot both fit, the page ends before the parent line. A single line longer than the budget is cut and the page carries an explicit flag that the criterion continues. Offsets stay character offsets, and the continuation resumes exactly where the page ended. The page budget follows the measured caps rule.
- Proof: replay cases with an exclusion whose indented exception straddles the old boundary keep both on one page; a line longer than the budget is flagged as continued; following every continuation reproduces the registry text exactly.
- Changes: eligibility paging, its response type field, schema and cases.
- Defers: nothing.
