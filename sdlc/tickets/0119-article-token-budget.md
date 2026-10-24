# 0119 — Measure the shipped article token budget

Status: OPEN.

Milestone: 1

## Outcome

The article slice's token table in `planning/slices/article.md` gives the measured tokens of shipped 1.0 output for every row, from the recorded replay cases, and compares each with 0.9 where the slice measured 0.9. The PDF rows and examples 69 to 71 name their landed cases instead of "Pending 0052". Each 1.0 default that costs more tokens than 0.9 for the same question states a reason, and each measured row has a ceiling in its case. This ticks the article token budget box.

- Why: ceilings are enforced per case, but about 18 of the table's 23 rows still say shipped tokens are unknown, and the PDF rows predate 0052.
- Starts from: the token table and example list in `planning/slices/article.md`; the article replay cases; the size ceiling rule in `planning/slices.md`.
- Proof: measurements come from offline replay of landed cases, with the command and byte counts recorded; no provider call. A row whose question has no recorded case says so and names the case to add, and that case is added. Then make landing-gate runs through pm job.
