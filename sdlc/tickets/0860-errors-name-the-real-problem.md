# 0860 — errors-name-the-real-problem

Status: OPEN.

Milestone: 7

## Outcome

Usage and source errors name the real problem: `variant normalize` without a build names `--build`, not `--source`; the incidence error names the listed value; normalize returns the provider's reason for a rejection; an unknown snapshot name has its own error, not an expired pool; `get article` states its 20-ID limit.

## Why

From the bench 0039 persona review (2026-11-12, runs on BioMCP 58d83b6da), `personas/<slug>/runs/2026-11-12/review.md` and `log.md`: tumor-board log.md:30-32, rare-disease log.md:106, 128 and 132, msl log.md:18: agents lost calls guessing.

## Proof

- Proof: one routine case per error, each asserting the named flag or reason.

## Starts from

- Starts from: the error construction for these commands.

## Keeps

- Keeps: error codes and schema unless a new code is required.

## Defers

- Defers: naming the bad value in every usage error (0855's Defers).

## Amendment, 2026-11-12

From bench 0040's rerun, reproduced on main 59150d16d (`sdlc/records/2026-11-12-persona-rerun-failures.md`):

- Four more cases: an error naming `--gene` when it was not passed (#15); blaming cBioPortal when the profile asked for is copy-number (#22); "expired" for something that never existed (#23); a generic refusal of `--fields` on `get trial` (#24).

## Changes

- Changes: `crates/biomcp-query/src/`, `crates/biomcp/src/`, `tests/cases/`
