# 0188 — Make next-step policy exhaustive over response bodies

Status: COMPLETE.
Landed: 6ab3a4bafc5a62bbbc5c4f04a061e4ca3def52e5

Milestone: 4

## Outcome

Every response body type gets its next-step policy from one exhaustive match in the command's step assembly, so a new body type cannot compile until its policy is chosen. No body silently falls through to the generic limit of five.

- Why: `command/steps.rs` repeats the policy as a whitelist of body variants, and any unlisted variant falls through to `truncate(5)`. That dropped valid next steps twice in landed or accepted code: ClinPGx (0167, fixed in ba735a43) and WHO medicines (0184, fixed in cbc920b6). Medium severity, no wrong clinical answer found.
- Starts from: `crates/biomcp/src/command/steps.rs`; architecture.md 6.10's next-step bounds; the PMA exception; 0149's selectable-candidate rule; the ClinPGx and WHO outside-in regressions.
- Rules: one local helper over the query body type with no wildcard arm, naming for each variant whether it is bounded, selectable with one get per candidate on the page plus the page step, or nested with its own page handling. Current action limits and the PMA exception are unchanged. No new framework or shared rule.
- Proof: the existing ClinPGx and WHO regressions and the retained paired commands pass unchanged; one outside-in case per policy kind asserts its steps. A reviewer confirms the match has no wildcard. No provider call. Then make landing-gate runs through pm job.
