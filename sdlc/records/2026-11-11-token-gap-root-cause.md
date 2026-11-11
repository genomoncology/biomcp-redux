# Where BioMCP's tokens go against curl

November 11, 2026. An expert-assist consultation (Fable, advice only) on the bench's round on fixed tooling (`biomed-agent-bench` ticket 0034, BioMCP main 4f4414616, Sonnet, v3's 19 questions) and the 1.0 baseline. It rebuilt the token count from the session records on the build machine. The PM weighed it and accepted the ranking; the replay figures are estimates, because they drop turns and shrink replies without modelling how the agent would then behave.

## The measurement

After the typed-tools, drug-name-lookup and call-queue fixes, refused calls fell from 52.5 to 5 a repeat and partial searches from about 40% to 0%, but tokens a repeat were typed with skill 4.34M, command line 3.97M, typed with no skill 3.36M (13 of 20 runs never called BioMCP), against curl's 3.30M (3.46M counting its one uncounted run). Milestone 7 needs at least 20% fewer than curl.

"Total tokens" is the sum over turns of the whole conversation re-read at that turn, so cost is turns times conversation size.

## Replay, cumulative, against curl at 3.46M

| Removed | Typed | CLI |
| --- | --- | --- |
| As measured | +26% | +15% |
| Short-name slips (bench client, 19 turns a repeat) | +19% | n/a |
| Wholly wasted usage-error turns (15 typed, 8 CLI) | +10% | +9% |
| Skill cut from about 2,900 to 600 tokens | 0% | 0% |
| Replies 25% smaller | −12% | −11% |
| Replies 50% smaller | −24% | −22% |

## Causes, ranked

1. **The first read.** The skill is re-read every turn: about 651k tokens a repeat in typed mode, 15% of its total.
2. **Bigger, less aimed replies.** A successful typed turn adds 3,420 tokens; curl 2,600; CLI 2,120. Typed replies reach code mode as a status line plus JSON in one string; only 16% of typed turns pick fields (CLI 45%, curl with jq over 90%). About 16% of reply bytes are envelope (`next`, `sources`, `query`, schema), and the status line repeats `next`.
3. **One call answers less than one curl call.** Typed made 298 BioMCP calls plus 75 curl commands where curl alone made 144. Loops of per-record gets: 81 PMA gets and 52 WHO diagnostic gets over two repeats, 28 trial-results gets. Drug labels: 70 calls of one label section each; curl fetches six sections in one call.
4. **Failed calls.** Real but not dominant: about 0.30M typed and 0.20M CLI in wasted usage-error turns.
5. **Turn count is the lever.** On the GPT model the CLI beats curl by 33% purely by cutting turns (252 to 167).

## Rejected

- Agents distrust BioMCP and double-check with curl: only one curl turn a repeat follows a failed BioMCP turn; stated reasons are gaps (EMA pages, FDA letters, FAERS and cBioPortal counts, gnomAD populations, trial enrolment) or "quicker to query openFDA directly".
- No tool can win on these questions: typed won by 15% or more on 4 of 19 questions, and GPT on the command line wins overall. Partly true: every setup has a shell and Sonnet knows these APIs.

## Not verified

- The bench hides tools behind tool search and gives every agent a shell, unlike Amazon Quick.
- Curl's two repeats were 3.94M and 2.97M: two repeats cannot resolve a 20% bar.

## What changed because of it

- The command map (0836) was rejected in review for growing the first read, and re-scoped: the first read must not grow and should shrink toward about 600 tokens; the map and examples move to on-request help.
- New tickets: several sections in one get (0846), searches carry what agents loop to fetch (0847). 0836's review also found the gate never runs the docs check and the build machine's Python is too old for it (0848). 0837 also drops the typed status-line prefix and the duplicated `next`. Chosen fields (0838) moves up.
- Bench ticket 0035: a round shaped like Amazon Quick (no shell, tools listed up front) and three more curl repeats.

## Correction from 0847's developer (same day)

The records show the "loops of per-record gets" were not all per-row loops. The 81 PMA gets paged one long FDA safety and effectiveness summary for two devices, about 20 pages each, searching for trial IDs and sensitivity sentences; the 26 trial-results gets paged one trial's results looking for participant-flow counts. Only the WHO diagnostic gets (one per product, for `prequalification_year`) and 6 trial gets (to learn whether results were posted) were per-row. So 0847 added just those two fields. The paging cost is a different fix: finding text inside a long section (as article text already offers), or larger pages, and participant flow as its own section (0840). The trial question that looped plain gets also wanted enrollment and acronym on search rows; deferred.
