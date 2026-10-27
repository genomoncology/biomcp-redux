# 0369 — Grow the skill use cases into worked how-tos

Status: OPEN.

Milestone: 5

## Outcome

Each file in `skills/use-cases/` is a worked how-to: the question, the commands in order, what to read in each reply and where to go next. No two use cases start with the same command unless they diverge at once. Use case 11 maps a locus with commands that work. Every command runs against recordings through the help example runner.

- Why: the docs lead's product findings report (2026-10-27), item F2. The use cases are 9-line notes, several share their first command (01 and 06, 03, 07 and 15, 08 and 12), and biomcp.org has no worked how-to per use case. Ian's ruling that 1.0 ships every known fix (2026-10-27).
- Starts from: `skills/use-cases/`, the help example runner and README test input.
- Rules: a clinical reviewer checks each use case's question and reading guidance; the developer writes no clinical interpretation.
- Proof: the example runner executes every use case command against recordings; the clinical reviewer's acceptance is recorded in the ticket.
- Changes: `skills/use-cases/`, help examples and recordings.
- Defers: nothing.
