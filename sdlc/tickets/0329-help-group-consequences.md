# 0329 — Say which sources each disable group turns off

Status: OPEN.

Milestone: 5

## Outcome

Help for source settings and `biomcp health` list the sources each disable group turns off, so a commercial user sees that `restricted` turns off PubMed, Europe PMC, PMC reading, ClinVar, ClinicalTrials.gov and the other restricted sources before setting it.

- Why: experiment 497 (ticket 0325, evidence on the evidence store under 0325, PM evaluation 2026-10-27 for Jarvis m9494)'s first run used the documented commercial recipe and lost every common article source without noticing. The labels and the recipe are Ian's rulings (`2026-10-24-sources-are-open-by-default.md`, `2026-10-26-new-sources-follow-open-by-default.md`); only the explanation is missing.
- Starts from: the source registry's `Use` labels, the help topics for sources and config, and `biomcp health` output.
- Rules: no label or recipe change. The list is generated from the registry, so it cannot drift. Help says `restricted` means each publisher's terms need checking.
- Proof: a help case lists the sources in each group from the registry, and a health case shows the same groups; help caps pass.
- Changes: help, health output and cases.
- Defers: nothing.
