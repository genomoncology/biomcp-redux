# 1057 — refusals-name-the-real-cause

Status: OPEN.

Milestone: 7

## Outcome

Two refusals say what is actually wrong. `biomcp get trial NCT04613596 --lat 41.1 --lon -80.65` says that distances need the `locations` section, or simply adds it. `biomcp get drug mobocertinib markets --source drugsfda` says that `markets` takes no `--source`.

## Why

Both refusals blame a value that is fine, so an agent that follows them cannot fix its call. In one run the agent made this drug call four times, was refused four times, and never used BioMCP again in that run.

## Proof

- Proof: a case for each command whose reply names the real cause; both fail on the base commit.

## Starts from

- Starts from: Seen on main be96d8d40 (1.0.0), Linux, 2026-11-19, while agents used the command line build on real tasks.
- `get trial NCT04613596 --lat 41.1 --lon -80.65` replies `usage.arguments`, `option: --lat`, with the valid value described as "Bounded nonempty text...". The same call with the word `locations` after the ID works and gives each site's miles.
- `get drug mobocertinib markets --source drugsfda` replies "option is missing or invalid; choose an accepted value" beside a list of accepted values that includes `drugsfda`. Help says `markets` takes a name and no source.
- PM decision, 2026-11-19: for the trial call, add `locations` when `--lat` and `--lon` are given with no section, since that is the only thing the caller can mean. For the drug call, refuse and name the cause.

## Keeps

- Keeps: every other refusal's text and code.

## Defers

- Defers: a general review of refusal wording; file what the developer finds as a list.

## Changes

- Changes: the two commands' argument checks, their cases and help.
