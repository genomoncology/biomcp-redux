# PM handoff, 2026-11-17

Written by the outgoing PM for BioMCP core, hosting, the persona bench (biomed-agent-bench) and BioData's plan, when Ian switched agents. Ticket status lives in pm (`pm daily`, `pm item N`); this says what is in flight, why, and what comes next. Delete it once nothing cites it.

## Standing approvals and rules from Ian

- "Do whatever you got to do to get things done." Drive queues unasked, in parallel, with developer agents, independent reviewers and landers.
- 1.0 baseline budget: $130 approved. Then on 2026-11-17: "I'm not worried about the GLM costs" and "You have my permission to do parallel."
- Anthropic API is out of credit: leave the Anthropic key alone and don't top it up (Jarvis, m37923). Agents in the bench run GLM 5.3; graders run gpt-6.1-terra. GLM never grades (GLM graders matched hand grades on 23 of 33 facts, 2026-11-16).
- Release bar: `sdlc/decisions/2026-11-12-1-0-is-done-when-tools-are-efficient-and-personas-reach-a-minus.md` stands. Every known fix ships (`2026-10-27-1-0-ships-every-known-fix.md`). Don't defer a finding without asking Ian.

## BioMCP tickets in flight

Developer brief used for every BioMCP ticket: workspace scratch `dev_brief2.md` (session-local; its rules: worktree per ticket, never compile on the Beelink, `pm job` with gate `make --keep-going landing-gate docs-check schema-release-check`, additive optional schema fields, error-text changes need `release/schema-reviews.json`, size caps measured plus 15%, 600-line files and 80-line functions, Edit tool not heredocs). Landing chain: review accepts with `pm ticket review N accept --rev SHA --branch B`, then `pm ticket land N`; if the land gate fails on schemas, merge main and regenerate.

- 1044 Trial eligibility splits inclusion and exclusion. Branch `ticket/1044-eligibility-split`. First review rejected (empty list on a page read as "no exclusions"). Fixed: a list appears only on pages holding its items; when split, lists replace the full text (`preamble` + `inclusion` + `exclusion`); a guard stops splitting when another line ends in "criteria:", except list items (FLAURA splits). Adds a narrow `required_removals` category to the schema gate for `criteria` becoming optional (approved by PM under Ian's no-backwards-compatibility ruling). Paused: 2d584ef85 (merged with main incl. 1042 and 1045) passed the full gate in j42498; head 1573b93d4 adds only a Progress note. Next: independent re-review of the three fixes and the `required_removals` category, then land.
- 1046 PubMed search rows say open access. Branch `ticket/1046-pubmed-oa`. Review returned findings (Europe PMC timed at 11 to 19 s). Fixed: lookup runs alongside the PubMed summaries, 3 s limit, a search option to skip it and a config default (on), a timed-out lookup is named in the page's sources with a retry step, the README case asserts the AKT1 open-access values, `--fresh` passes through. Paused at c8f4ad272 (fixes at d259513d3, merged with main 4b721738b). Gate j42495 failed one library test, `stale_summary_fallback_stays_partial_when_its_page_is_read_offline`: an offline reread of a stored page now runs the Europe PMC lookup, named with reason `offline` where the test expects `proxy`. Recommended fix: skip the lookup when reading offline (an offline read should make no network call). Next: that fix, gate, independent re-review of the four fixes, land.
- 1043 Trial search rows say why stopped and count sites. Branch `ticket/1043-trial-row-stop-sites` at 29040d2f0, worktree `worktrees/biomcp-redux-1043-trial-row-stop-sites`. Paused, code complete: stop reason on rows, `--why-stopped WORDS`, `--site-count` (facility counts over up to 5,000 trials, names as written). 128 search recordings were re-keyed (contents unchanged). Last job j42502 passed all touched cases; only `check-cheap` failed on a long function, fixed in 29040d2f0 but not yet built. Next: `make check-cheap` and touched cases, then the full gate (the secrets scan may flag registry page tokens in the renamed recordings; reviewed entries go in `release/secrets.ignore.jsonl`, as 1018 did), independent review, land.
- Landed today: 1034 to 1042 and 1045 (main 4b721738b).
- Held for Ian: 1028 gene-set enrichment (branch `ticket/1028-gene-set-enrichment`, unmerged). Recommendation: keep the Oct 25 ruling and leave it out.

The six stale review records were closed on main today (bdc8c849a, 876387841); still-real findings became 1042 to 1046. The main checkout `repos/biomcp-redux` sits on an old planning branch, so `pm daily` run there still lists them as open.

## Persona baseline (biomed-agent-bench ticket 0066)

Rules, set before any run, are in `sdlc/tickets/0066-one-zero-baseline-gated-on-grader-check.md` there; its Progress has phase one's numbers.

- Phase one done: rare-disease, three counted runs per arm, BioMCP 8763878419bb, $10.46. Code mode 6.3 stated / 4.7 missed / 0.3 wrong of 13, $1.31 a run; BioMCP 5.0 / 6.0 / 1.0, $0.60 a run. BioMCP got one fact wrong in every run: the first problem to look at.
- Spread was wide (BioMCP 3.5 to 7.5 stated), so by the rule every persona runs three times per arm.
- Leak flags (4 of 10 runs) had two causes, both fixed in the briefs (bench 4bc0db2, c4241d6): a code-mode logging script saved expanded key URLs; GLM guessed the input path from the skill's folder.
- Hand-grade check done (`personas/_eval/handgrade-0066-rare-disease/README.md`): terra consensus agrees with hand grades on 67 of 70 facts (95.7%); grader 1 89.7%, grader 2 92.3%. The hand-grader corrected 15 of its own blind grades to the every-part rule after seeing the graders' verdicts, which the README says. Two facts are wrong and must be fixed before phase two: fact 8 (NCBI now gives 211 tests from 49 labs; GTR's site says 51) and fact 2 (ClinGen's gene pages are plain HTML, not a JavaScript shell). Facts 1, 6, 10, 11 and 13 are imprecisely worded. On corrected grades code mode stated 7.7 per run, BioMCP 5.3; BioMCP's misses were mostly an empty gnomAD frequency and the support-group links (r1 wrongly said no source has them; GARD does). Those are the first traces to read.
- Phase two plan: once 1043, 1044 and 1046 land, build and seed BioMCP main on the M5 (`tools/build-biomcp.sh`, `tools/seed-cache.sh` through `pm job`), then run all eleven personas (rare-disease again) three times per arm, four personas at once, on that one build.
- Bridge to 2026-11-16: `personas/_eval/bridge-2026-11-16/README.md`. Terra is stricter than the old Claude graders; compare across eras only through its terra column.
- After the baseline: group misses into problems, fix one at a time, rerun one persona that shows it (Ian's cycle; `notes/agent-20261117/eval-driven-agent-development-approach.md`).

## Raising BioMCP agents' scores (PM's recommendation to Ian, 2026-11-17)

Ian found the expected gain from 1042 to 1046 disappointing: they patch one fact here and there, inside a run-to-run spread of up to four facts. The recommended next steps, before phase two:

1. Read the BioMCP arm's traces, not just scores. It made 72 calls to code mode's 104 and seems to stop early; it got the same rare-disease fact wrong in all three runs. Find where it stopped, what it misread and what it never asked. About 65% of misses so far were agent-side.
2. Teach research, not commands: a short research plan per question type in the skill (e.g. rare disease: confirm gene, disease links, variants, trials, then check the answer against the question), and replies that make "partial" and "more here" unmissable. Help text already takes 9 to 17% of input tokens.
3. Test a BioMCP-plus-web arm. The workspace focus says the agent may use other tools alongside BioMCP; that is the shipped setup.
4. Smaller: one persona on gpt-6.1-terra arms to see whether BioMCP's edge grows with a stronger model; keep cost and efficiency in the scorecard, since the 2026-11-12 bar judges both.

## Waiting on Ian

- 1028 gene-set enrichment (above).
- The release bar: PM withdrew its separate proposed bar; the 2026-11-12 decision stands, with the baseline as evidence for its review.

## Sent to Jarvis and Botassembly

- m37851 bench machinery offered to Botassembly; m37896 reporting requirements (budgets, problem map, versions, runs, status, scoring). Botassembly's open items: grader check against hand grades (being done in 0066), a regrade-from-records feature, item history.

## Not checked this session

- Hosting and BioData's plan.
