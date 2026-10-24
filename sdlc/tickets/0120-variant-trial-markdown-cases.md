# 0120 — Prove the Markdown view for variant and trial commands

Status: OPEN.

Milestone: 2

## Outcome

Every variant and trial command prints a readable Markdown view through the generic view from 0064, with cases that assert its shape for a search, a get card and each section, as the gene and article cases do.

- Why: 0064's view is generic, but cases cover only gene and article output, so a variant or trial shape the view renders poorly would go unnoticed.
- Starts from: `crates/biomcp/src/view`, the gene and article Markdown cases, and the landed variant and trial replay cases.
- Proof: Markdown cases from existing replays, with size ceilings; any view fix stays inside the generic view. No provider call. Then make landing-gate runs through pm job.
