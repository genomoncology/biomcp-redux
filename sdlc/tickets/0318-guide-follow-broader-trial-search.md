# 0318 — Tell agents to follow broader trial searches before treating a screen as complete

Status: COMPLETE.
Landed: 55650cb989ac3ceea328c8382a12b305c43f29a2

Milestone: 5

## Outcome

The agent guide's trial screening workflow tells an agent that an exact variant search is a starting point, and that it should run the broader gene and program searches BioMCP suggests before reporting a screen as complete.

- Why: an investigation team testing 1.0 found that an exact KRAS G12A colorectal trial search returned six trials but missed two relevant leads that broader searches found. BioMCP already suggested broader queries; the agent did not follow them. Trial ruling 1 keeps search literal, so the fix belongs in guidance and next steps, not in silent broadening.
- Starts from: `skills/SKILL.md` and its use cases, the trial search next steps, trial slice ruling 1, and the team's commands once they share them.
- Rules: no automatic broadening and no clinical judgment in the guide; it describes the search sequence, not which trials fit. Next step wording may be sharpened so the broader search reads as part of a complete screen. Guide and help caps follow the measured caps rule.
- Proof: the guide's trial screening section names the broader searches; a case shows an exact variant search's next steps include the gene-level search; the embedded guide budget passes.
- Changes: the agent guide and, if needed, trial search next step wording.
- Defers: nothing.
