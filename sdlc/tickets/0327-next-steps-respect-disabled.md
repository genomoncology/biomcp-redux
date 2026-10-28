# 0327 — Keep next steps runnable under disabled sources

Status: COMPLETE.
Landed: 4cbb7811722ba7727587b59e6abfbbc92e42975e

Milestone: 5

## Outcome

An article or author reply never suggests a next step that calls a disabled source, and next step commands repeat only the source exclusions the user gave on the command line, not those from the config file.

- Why: experiment 497 (ticket 0325, evidence on the evidence store under 0325, PM evaluation 2026-10-27 for Jarvis m9494) found known-article replies that disabled Semantic Scholar still suggested Semantic Scholar citation commands, which could not run, and long exclusion lists from the config repeated in every next command, using output for no gain.
- Starts from: the next step builders, the source admission and `disable_sources` handling, and the article reply cases.
- Rules: a step whose source is disabled is left out; when that removes every step of a kind, the reply says the source is disabled instead. Config-file exclusions stay in force when the next command runs because the config persists, so the command does not repeat them; command-line exclusions are repeated so the command reproduces the call.
- Proof: a case with Semantic Scholar disabled shows no Semantic Scholar step; a case with exclusions in config shows next commands without them; a case with `--disable-sources` on the command line repeats it.
- Changes: next step building and cases.
- Defers: source filtering of next steps for other entities, which waits for an observed case of a disabled-source step there; no shared request-to-sources planner is built for 1.0 (PM ruling 2026-10-27, m8126 ask 69).
