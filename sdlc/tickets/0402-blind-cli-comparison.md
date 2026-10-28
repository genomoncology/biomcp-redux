# 0402 — Compare 0.9.3 and 1.0 blind before publishing

Status: OPEN.

Milestone: 5

Depends on: 0293

## Outcome

A fresh agent with no project knowledge runs about 20 tasks with the released 0.9.3 CLI and the final 1.0 candidate CLI in an empty folder. The report scores each task on both versions for ease of use, tokens spent, chaining, `jq` use, access to the underlying data and answer correctness. Every task where 1.0 does worse is fixed before publication or cites the ruling that explains the difference.

- Why: `decisions/2026-10-28-blind-cli-comparison-before-publishing.md`. Ian makes this the last step before publishing 1.0.
- Starts from: the 0.9.3 release binary, the installed 1.0 candidate from 0293, the earlier outside question sets and 0.9.3 comparison evidence.
- Rules: a clinical reviewer chooses the tasks to match real cancer care and biomedical research questions across the entity families, including multi-step and `jq` tasks; tasks are fixed before either binary runs. The agent sees only the two binaries, their own help and the task list. Both versions use the same keys by name, which the agent never prints or records. Correctness is judged against the clinical reviewer's answers, which stay out of Git and away from the agent under test. Live calls stay within each source's published limits. No licensed content goes into the repository; the report keeps public IDs, commands and counts.
- Rubric (PM ruling 2026-10-28): the scoring also uses MCP-Bench's six judged dimensions, so results compare with that public benchmark: task fulfillment, grounding of each claim in tool output, tool choice, argument accuracy, chaining one result into the next call, and call efficiency. The task set includes fuzzy multi-entity briefings like MCP-Bench's: recent papers and preprints in a date window, variant frequency, recruiting trials with sponsors and sites in a named city, trial adverse-event rates, label and approval facts, FAERS safety and device events for test kits, and a pharmacogenomic variant.
- Proof: a fresh reviewer checks the report; every 1.0 regression has a fix ticket that landed or a cited ruling.
- Changes: none expected beyond fix tickets the comparison finds.
- Defers: publication, which needs Ian.
